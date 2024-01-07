use anchor_lang::{
    prelude::Pubkey,
    solana_program::{self},
    system_program, AccountDeserialize, InstructionData, ToAccountMetas,
};
use solana_program::instruction::Instruction;
use solana_program_test::{tokio, ProgramTest, ProgramTestContext};
use solana_sdk::{account::Account, signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_initialize() {
    let SetUpTest {
        validator,
        user,
        counter_pda,
    } = set_up_test();

    let mut context = validator.start_with_context().await;

    let init_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Initialize {
            counter: counter_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Initialize {}.data(),
    };

    let mut init_tx = Transaction::new_with_payer(&[init_ix], Some(&user.pubkey()));
    let recent_blockhash = context.last_blockhash;
    init_tx.partial_sign(&[&user], recent_blockhash);

    context
        .banks_client
        .process_transaction(init_tx)
        .await
        .unwrap();

    let counter: anchor_counter::Counter = load_and_deserialize(context, counter_pda).await;

    assert_eq!(counter.count, 0);
}

#[tokio::test]
async fn test_increment() {
    let SetUpTest {
        validator,
        user,
        counter_pda,
    } = set_up_test();

    let mut context = validator.start_with_context().await;

    let init_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Initialize {
            counter: counter_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Initialize {}.data(),
    };

    let increment_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Increment {
            counter: counter_pda,
            user: user.pubkey(),
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Increment {}.data(),
    };

    let mut init_increment_tx =
        Transaction::new_with_payer(&[init_ix, increment_ix], Some(&user.pubkey()));
    let recent_blockhash = context.last_blockhash;
    init_increment_tx.partial_sign(&[&user], recent_blockhash);

    context
        .banks_client
        .process_transaction(init_increment_tx)
        .await
        .unwrap();

    let counter: anchor_counter::Counter = load_and_deserialize(context, counter_pda).await;

    assert_eq!(counter.count, 1);
}

#[tokio::test]
async fn test_double_increment() {
    let SetUpTest {
        validator,
        user,
        counter_pda,
    } = set_up_test();

    let mut context = validator.start_with_context().await;

    let init_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Initialize {
            counter: counter_pda,
            user: user.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Initialize {}.data(),
    };

    let increment_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Increment {
            counter: counter_pda,
            user: user.pubkey(),
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Increment {}.data(),
    };

    let increment_ix_2 = increment_ix.clone();

    let mut init_increment_tx = Transaction::new_with_payer(
        &[init_ix, increment_ix, increment_ix_2],
        Some(&user.pubkey()),
    );
    let recent_blockhash = context.last_blockhash;
    init_increment_tx.partial_sign(&[&user], recent_blockhash);

    context
        .banks_client
        .process_transaction(init_increment_tx)
        .await
        .unwrap();

    let counter: anchor_counter::Counter = load_and_deserialize(context, counter_pda).await;

    assert_eq!(counter.count, 2);
}

//UTILITY STUFF

pub struct SetUpTest {
    pub validator: ProgramTest,
    pub user: Keypair,
    pub counter_pda: Pubkey,
}

pub fn set_up_test() -> SetUpTest {
    let mut validator = ProgramTest::default();
    validator.add_program("anchor_counter", anchor_counter::ID, None);

    let user = Keypair::new();
    validator.add_account(
        user.pubkey(),
        Account {
            lamports: 1_000_000_000,
            ..Account::default()
        },
    );

    //get the counter PDA
    let (counter_pda, _) = Pubkey::find_program_address(&[b"counter"], &anchor_counter::ID);
    SetUpTest {
        validator,
        user,
        counter_pda,
    }
}

pub async fn load_and_deserialize<T: AccountDeserialize>(
    mut ctx: ProgramTestContext,
    address: Pubkey,
) -> T {
    let account = ctx
        .banks_client
        .get_account(address)
        .await
        .unwrap()
        .unwrap();

    T::try_deserialize(&mut account.data.as_slice()).unwrap()
}

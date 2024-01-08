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
    init_tx.partial_sign(&[&user], context.last_blockhash);

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
        user: _,
        counter_pda,
    } = set_up_test();

    let mut context = validator.start_with_context().await;

    let init_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Initialize {
            counter: counter_pda,
            user: context.payer.pubkey(),
            system_program: system_program::ID,
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Initialize {}.data(),
    };

    let increment_ix = Instruction {
        program_id: anchor_counter::ID,
        accounts: anchor_counter::accounts::Increment {
            counter: counter_pda,
            user: context.payer.pubkey(),
        }
        .to_account_metas(None),
        data: anchor_counter::instruction::Increment {}.data(),
    };

    let mut init_increment_tx =
        Transaction::new_with_payer(&[init_ix, increment_ix], Some(&context.payer.pubkey()));
    init_increment_tx.partial_sign(&[&context.payer], context.last_blockhash);

    let _res = context
        .banks_client
        .process_transaction(init_increment_tx)
        .await;

    let counter: anchor_counter::Counter = load_and_deserialize(context, counter_pda).await;

    assert_eq!(counter.count, 1);
}

#[tokio::test]
async fn test_double_increment() -> anyhow::Result<()> {
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

    let init_increment_tx = Transaction::new_signed_with_payer(
        &[init_ix, increment_ix, increment_ix_2],
        Some(&user.pubkey()),
        &[&user],
        context.last_blockhash,
    );

    let _res = context
        .banks_client
        .process_transaction(init_increment_tx)
        .await;

    let counter: anchor_counter::Counter = load_and_deserialize(context, counter_pda).await;

    assert_eq!(counter.count, 2);

    Ok(())
}

#[tokio::test]
async fn test_multi_signer() -> anyhow::Result<()> {
    Ok(())
}

/// Struct set up to hold the validator, an optional user account, and the counter PDA.
/// Returned from the set_up_test() function
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

/// Fetch the account from the ProgramTestContext and deserialize it.
/// Taken from the MarginFi Github tests: https://github.com/mrgnlabs/marginfi-v2/blob/main/test-utils/src/test.rs#L468
pub async fn load_and_deserialize<T: AccountDeserialize>(
    mut ctx: ProgramTestContext,
    address: Pubkey,
) -> T {
    let account = ctx
        .banks_client
        .get_account(address)
        .await
        .unwrap() //unwraps the Result into an Option<Account>
        .unwrap(); //unwraps the Option<Account> into an Account

    T::try_deserialize(&mut account.data.as_slice()).unwrap()
}

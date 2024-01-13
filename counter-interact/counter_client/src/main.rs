use anchor_counter_interface::{
    increment_ix_with_program_id, initialize_ix_with_program_id, IncrementKeys, InitializeKeys,
};
use anyhow::anyhow;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::transaction::Transaction;
use std::path::{self, Path};
use std::str::FromStr;

fn main() -> anyhow::Result<()> {
    /*SET THESE TO YOUR VALUES OR READ FROM ENV*/
    let path_to_keypair = path::Path::new("/home/derked/.config/solana/id.json");
    let program_id = "CQ2VvuR8Du2WQq1XWmzBKxmK4arc7BVReWMxUMW3nJs5";
    let rpc_url = "http://localhost:8899";

    let SetUpClient {
        rpc,
        user,
        program_id,
    } = SetUpClient::new(rpc_url, &path_to_keypair, program_id)?;

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        Err(anyhow!("Please specify an action: init, incr, init_incr"))?;
    }

    let (counter_pda, _) = Pubkey::find_program_address(&[b"counter"], &program_id);

    let sig = match args[1].as_str() {
        "init" => initialize(&rpc, &counter_pda, &user, program_id),
        "incr" => increment(&rpc, &counter_pda, &user, program_id),
        "init_incr" => initialize_and_increment(&rpc, &counter_pda, &user, program_id),
        _ => Err(anyhow!("Please specify an action: init, incr, init_incr"))?,
    };

    match (sig, args[1].as_str()) {
        (Ok(sig), _) => println!("Tx Successful with Signature: {:?}", sig),
        (Err(e), "init") => {
            if e.to_string().contains("custom program error: 0x0") {
                println!("Counter Account Already Initialized!");
            } else {
                Err(anyhow!("Something went wrong with initializing: {:?}", e))?;
            }
        }
        (Err(e), "incr") => Err(anyhow!("Something went wrong with incrementing: {:?}", e))?,
        (Err(e), "init_incr") => {
            if e.to_string().contains("custom program error: 0x0") {
                println!("Counter Account Already Initialized! Cannot increment afterwards!");
            } else {
                Err(anyhow!(
                    "Something went wrong with initializing or incrementing: {:?}",
                    e
                ))?;
            }
        }
        _ => unreachable!(),
    }

    display_counter_info(&rpc, &counter_pda)?;
    Ok(())
}

pub struct SetUpClient {
    pub rpc: RpcClient,
    pub user: Keypair,
    pub program_id: Pubkey,
}

impl SetUpClient {
    pub fn new(rpc_url: &str, path_to_keypair: &Path, program_id: &str) -> anyhow::Result<Self> {
        let rpc =
            RpcClient::new_with_commitment(rpc_url.to_string(), CommitmentConfig::confirmed());
        let program_id = Pubkey::from_str(program_id)?;
        let user = solana_sdk::signature::Keypair::read_from_file(path_to_keypair)
            .map_err(|e| anyhow!("Error reading keypair: {:?}", e))?;

        Ok(Self {
            rpc,
            user,
            program_id,
        })
    }
}

///Initialize the Counter Account
fn initialize(
    rpc: &RpcClient,
    counter_pda: &Pubkey,
    user: &Keypair,
    program_id: Pubkey,
) -> anyhow::Result<Signature> {
    let init_keys = InitializeKeys::from([*counter_pda, user.pubkey(), system_program::ID]);

    let ix = initialize_ix_with_program_id(program_id, init_keys)?;
    let recent_blockhash = rpc.get_latest_blockhash()?;

    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&user.pubkey()), &[&user], recent_blockhash);

    let sig = rpc.send_and_confirm_transaction(&tx)?;

    Ok(sig)
}

///Displays the count in the Counter PDA
fn display_counter_info(rpc: &RpcClient, counter_pda: &Pubkey) -> anyhow::Result<()> {
    println!("Counter Account @ Address: {:?}", counter_pda);
    let counter_acct_data = rpc.get_account_data(&counter_pda)?;
    let counter_acct = anchor_counter_interface::CounterAccount::deserialize(&counter_acct_data)?.0;

    println!("Counter count: {:?}", counter_acct.count);

    Ok(())
}

///Increment the Counter Account
fn increment(
    rpc: &RpcClient,
    counter_pda: &Pubkey,
    user: &Keypair,
    program_id: Pubkey,
) -> anyhow::Result<Signature> {
    let increment_keys = IncrementKeys::from([*counter_pda, user.pubkey()]);

    let ix = increment_ix_with_program_id(program_id, increment_keys)?;
    let recent_blockhash = rpc.get_latest_blockhash()?;

    let tx =
        Transaction::new_signed_with_payer(&[ix], Some(&user.pubkey()), &[&user], recent_blockhash);

    let sig = rpc.send_and_confirm_transaction(&tx)?;

    Ok(sig)
}

///Initialize the Counter Account and Increment
///Will not increment if the Counter Account is already initialized
fn initialize_and_increment(
    rpc: &RpcClient,
    counter_pda: &Pubkey,
    user: &Keypair,
    program_id: Pubkey,
) -> anyhow::Result<Signature> {
    let init_keys = InitializeKeys::from([*counter_pda, user.pubkey(), system_program::ID]);
    let init_ix = initialize_ix_with_program_id(program_id, init_keys)?;

    let increment_keys = IncrementKeys::from([*counter_pda, user.pubkey()]);
    let increment_ix = increment_ix_with_program_id(program_id, increment_keys)?;

    let recent_blockhash = rpc.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[init_ix, increment_ix],
        Some(&user.pubkey()),
        &[&user],
        recent_blockhash,
    );

    let sig = rpc.send_and_confirm_transaction(&tx)?;
    Ok(sig)
}

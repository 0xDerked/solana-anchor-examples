use anchor_counter_interface::{
    increment_ix_with_program_id, initialize_ix_with_program_id, IncrementKeys, InitializeKeys,
};
use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::transaction::Transaction;
use std::path::{self, Path};
use std::str::FromStr;

fn main() -> Result<()> {
    //Set these to your correct values
    let path_to_keypair = path::Path::new("/home/derked/.config/solana/id.json");
    let program_id = "CQ2VvuR8Du2WQq1XWmzBKxmK4arc7BVReWMxUMW3nJs5";
    let rpc_url = "http://localhost:8899";

    let SetUpClient {
        rpc,
        user,
        program_id,
    } = SetUpClient::new(rpc_url, &path_to_keypair, program_id)?;

    let (counter_pda, _) = Pubkey::find_program_address(&[b"counter"], &program_id);

    let mut sig = initialize(&rpc, &counter_pda, &user, program_id);

    match sig {
        Ok(sig) => println!("Initialize Tx Successful with Signature: {:?}", sig),
        Err(e) => {
            if e.to_string().contains("custom program error: 0x0") {
                println!("Counter Account Already Initialized ... continuing");
            } else {
                Err(anyhow!(
                    "Something went wrong initializing the counter account: {:?}",
                    e
                ))?;
            }
        }
    }

    display_counter_info(&rpc, &counter_pda)?;

    sig = increment(&rpc, &counter_pda, &user, program_id);

    match sig {
        Ok(sig) => println!("Increment Tx Successful with Signature: {:?}", sig),
        Err(e) => Err(anyhow!(
            "Something went wrong incrementing the counter account: {:?}",
            e
        ))?,
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
    let init_keys = InitializeKeys::from([
        *counter_pda,       //counter
        user.pubkey(),      //user
        system_program::ID, //system program
    ]);

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

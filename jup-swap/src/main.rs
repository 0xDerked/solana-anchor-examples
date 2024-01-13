use dotenv::dotenv;
use jupiter_swap_api_client::quote::QuoteRequest;
use jupiter_swap_api_client::swap::SwapRequest;
use jupiter_swap_api_client::transaction_config::TransactionConfig;
use jupiter_swap_api_client::JupiterSwapApiClient;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("No RPC_URL provided");
    let api_base_url = env::var("API_BASE_URL").expect("NO API_BASE_URL");
    let pk_base58 = env::var("PRIVATE_KEY").expect("No PRIVATE_KEY provided");
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    let jup_swap_client = JupiterSwapApiClient::new(api_base_url);

    let wallet = Keypair::from_base58_string(&pk_base58);
    println!("Wallet: {}", wallet.pubkey());

    const BONK: Pubkey = pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");
    const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

    let quote_request = QuoteRequest {
        amount: 10_000_000,
        input_mint: SOL,
        output_mint: BONK,
        slippage_bps: 50,
        ..QuoteRequest::default()
    };

    let quote_response = jup_swap_client.quote(&quote_request).await?;
    println!("Quote: {:?}", quote_response);

    let swap_reponse = jup_swap_client
        .swap(&SwapRequest {
            user_public_key: wallet.pubkey(),
            quote_response: quote_response.clone(),
            config: TransactionConfig::default(),
        })
        .await?;

    println!("Got swap response, sending tx");

    let versioned_tx: VersionedTransaction = bincode::deserialize(&swap_reponse.swap_transaction)?;
    let signed_tx = VersionedTransaction::try_new(versioned_tx.message, &[&wallet])?;

    let sig = rpc_client.send_and_confirm_transaction(&signed_tx).await?;

    println!("Tx Successful with Signature: {:?}", sig);

    Ok(())
}

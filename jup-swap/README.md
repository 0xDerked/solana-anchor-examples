This is an example of how to execute a swap using the Jupiter Swap API with Rust. It is currently set up to swap 0.01 SOL for BONK on mainnet.

This was actually really straightforward because of the awesome `jupiter-swap-api-client` [crate](https://github.com/jup-ag/jupiter-api-rust-example/tree/main) that's provided by Jupiter.

## Setup

You'll need an RPC url, a wallet private key with some SOL, and the api base url. The API base url is `https://quote-api.jup.ag/v6` for mainnet. I stuck these all in a dotenv file, but you could read or create them from anywhere. I use Helius as my rpc provider.

## coding the swap

Jupiter made this super easy because the crate linked above has an example folder that is nearly a clone of this. The idea is to create an `RpcClient` for our interaction with the cluster and a JupiterSwapApiClient` for help getting quote and swap objects.

Lines 18-26 do this setup. Lines 28 and 29 define the token addresses for our swap, SOL and BONK.

Lines 31-36 defined the QuoteRequest object we are going to send to the jupiter API. The amount is in lamports and represents 0.01 SOL. We are inputting SOL and getting out BONK. There are a bunch of different settings you can explore on the object.

After getting the quote response, we need to call the swap endpoint to get a `VersionedTransaction` object back we can sign and submit to the cluster. Jupiter uses `VersionedTransaction` because there are so many accounts that need to be passed into the transactions, a legacy transaction can't handle them all. You can read about `VersionedTransaction` in the Solana docs.

- Jupiter Docs: https://station.jup.ag/docs/apis/swap-api

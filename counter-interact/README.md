This is meant to be used with the `anchor-counter` project.

It is examples of interacting with a deployed program on Solana via Rust.

# Notes

Use solores to create the interface from the IDL (https://github.com/igneous-labs/solores/tree/master)

- cargo install solores
- set up the directory correctly -- interface crate and then client crate with the interface crate as a dependency
- solana docs around transaction commitment level: https://docs.solana.com/developing/transaction_confirmation

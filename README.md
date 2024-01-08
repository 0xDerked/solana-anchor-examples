# Introduction

This is a collection of detailed notes and examples of my journey in learning Solana development. I found there to be very few detailed resources (especially recent) on Solana development, particulary when it came to interacting with Solana programs with Rust. This includes both testing with the `solana_program_test` [crate](https://docs.rs/solana-program-test/latest/solana_program_test/) and interacting with the network via the `solana_sdk` [crate](https://docs.rs/solana-sdk/1.17.15/solana_sdk/index.html). My goal with this repo is to provide a reference for myself, but also hopefully help future developers who are trying to break into the Solana ecosystem.

This repo focuses heavily on Rust. The Solana Program (Smart Contract) development is written in Rust. I also chose to interact with and unit test the written contracts in Rust. I chose to do this because I am personally focused on learning Rust more in depth. I generally prefer to do my unit testing in the same language I wrote the smart contracts and then integration / frontend testing with Typescript. The EVM equivalent is having a combination repo with Foundry/Hardhat and writing unit tests/fuzzing with Foundry and then integration type testing with Hardhat.

I previously authored a repo called the [ethers-js-cheatsheet](https://github.com/thallo-io/ethers-js-cheatsheet) which helped me and many other developers learn how to interact with EVM based smart contracts with the ethers library. I hope to accomplish a similar goal with this repo.

# Prerequisites

I am not going to cover how to install Rust, Solana, Anchor -- nor am I going to focus on how Rust works (except in situations I think it's important). There are plenty of resources on the internet that cover how to install these things and on Rust itself. This repo is intended for intermediate to advanced developers who are new to Solana (like me!).

I'm also not going to spend too much time explaining the account based architecture of Solana programs except in the beginning where learning Anchor/how the program structure looks like.

# Contents

Each subfolder / project should be able to run standalone. I kind of suck with Github/Github submodules so I just slammed things into a single repo.

Each subfolder has its own README.md with more detailed information.

1. Counter Program

   - A really simple program that makes a global PDA (Program Dervied Account) and increments it. Unit tests in Rust. Primarily focused on learning the wiring and provides notes about how to use the `solana_program`, `solana_program_test`, `solana_sdk`, and `anchor_lang` crates.

2. Counter Interact

   - A simple Rust binary that interacts with the Counter Program. Primarily focused on learning how to interact with a deployed program on Solana via Rust. This is meant to be used with the `anchor-counter` project. Deploy the `anchor-counter` program locally and then run this binary to interact with it.

3. Jupiter Swap
   - Uses the Jupiter API and `solana_sdk` crate to actually make a swap on mainnet. This will actually swap real SOL so be careful. I like doing things on mainnet when it's cheap enough. It's currently set up to only swap a few cents back and forth.

# Disclaimer

Obviously these examples are not audited code. They are built from my own learning and patching together of resources.

# Feedback

This is obviously a learning repo so my own understanding of things is not perfect. I'm simply sharing what seems to have worked for me. If you are an experienced Solana developer, please feel free to correct my mistakes by opening a PR or reaching out to me on Discord or Twitter.

# Resources Used

This is just a dump of resources I found helpful in doing this. Some of these are a few years old, and I had to patch together a lot of different things to get things working. My primary advice is to join Discords and spam questions as they are the best feedback loop.

- [76 Developers Discord](https://discord.gg/HrqDu9hZsS)
- [SOLdiers Discord](https://discord.gg/cCfxJSmJzD)
- [Margin Fi Github](https://github.com/mrgnlabs/marginfi-v2/tree/main)
- [Solana Official Docs](https://docs.solana.com/introduction)
- [Solana Dev Course](https://www.soldev.app/course)
- [Solana Cookbook](https://solanacookbook.com/core-concepts/accounts.html#facts)
- [Blogpost About Testing an Anchor Solana Program With Rust -- OLD](https://medium.com/@jacob_62353/testing-an-anchor-solana-program-in-rust-65144b0cc5ce)

This is meant to be used with the `anchor-counter` project.

It is examples of interacting with a deployed program on Solana via Rust.

# anchour_counter_interface

My biggest hurdle was figuring out how to get types to work with for a Rust client. Usually you will have the IDL if the program was written with anchor, but you may not have access to the actual crate with all the types. I stumbled upon this awesome repo called [solares](https://github.com/igneous-labs/solores) which seems pretty active with the last push being ~2 weeks ago from 1/14/24.

You can just run `solares anchor_counter.json` and it will generate the interface crate. I've included in the interface crate in the repo, but if you delete it and run this command you should be able to reproduce it.

# counter_client

Our goal here is to create a program that interacts with a deployed program on a Solana cluster. We are going to use our localnet cluster and deploy the `anchor-counter` program from the other repo. Then we will code up a client to interact with this deployed program.

## deploy the anchor-counter program

Before we can interact with a deployed program, we need to spin up a test validator on localnet and deploy our anchor-counter program. In a new terminal window run `solana-test-validator -r` to run a new test validator with a clean state. Then in the `anchor-counter` repo run `anchor deploy` to deploy the program to the localnet cluster.

## coding the client

First things first we need to create an RPC Client to communicate with the localnet cluster. The url is straightforward for localnet as `"http://localhost:8899"`. You will need to set your `progam_id` and `path_to_keypair` to match your individual repo.

Line 75 uses the `RpcClient` struct from the `solana_client` crate to create a client. We specifically are going to use the commitment level of `confirmed` since that is recommended by the official Solana docs. It's a pretty common issue to run into bugs when you have mismatched commitment levels on your RPC Client and your specifications from sending transactions. I had this issue along the way but found that explicitly setting the RPC to `confirmed` and utilizing the `send_and_confirm_transaction` method on the `RpcClient` ran smoothly. We return the `RpcClient` instance along with the `user: Keypair` and the `program_id: Pubkey` so we can use them later.

I've set up the client to take specific instructions of `init, incr, and init_incr`. If you are familiar with rust you should see this matching pattern on line 35 where we parse the input and then call the correct function we want. All the functions take in the client, counter_pda, user, and program_id. For example, `cargo run init` should run the initialize function with our client, counter_pda, user, and program_id. Each function returns a `Result<Signature>` which is then matched for an output message.

## the initialize function and other functions

These functions are the meat of interacting with the client. Similar to our tests in the `anchor-counter` repo, we need to create a set of instructions, then a Transaction, and then send the Transaction to the cluster via the client.

`solares` and the `anchor_counter_interface` make this super straightforward and provide Structs for creating the Keys/Accounts required for the instruction, as well as a helper method for each function to create and return the instruction.

The `InitializeKeys` struct provides a way for us to create the accounts needed to be passed to our `Initialize` function. The interface helps us out here in reminding us we need the `counter_pda, user, and system_program`.

Once we create the keys, we can create the instruction using the `intialize_ix_with_program_id` function from the interface which will give us an `Instruction` to use in our `Transaction`.

Similar to the `anchor-counter` tests, we can create a new transaction with the `new_signed_with_payer`. We then send the transaction to the cluster with the `send_and_confirm_transaction` method on the `RpcClient`. We return the signature of the transaction so we can match on it later.

All the other functions follow a similar pattern.

Quick note here we are using a synchronous client, but there is also an option to use an asynchronous client. An example of the async client is used in the jupiter swap repo.

## display_counter_info

Solares gives us an interface that makes it easy to deserialze and read account data. We can fetch the data from the account address using the rpc client. We then can use the deserialize method on the `CounterAccount` provided by the interface to deserialize the data into a human readable format.

# Notes

- Use solores to create the interface from the IDL
- cargo install solores
- set up the directory correctly -- interface crate and then client crate with the interface crate as a dependency
- solana docs around transaction commitment level: https://docs.solana.com/developing/transaction_confirmation

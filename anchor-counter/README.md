# Set Up Stuff

You might need to generate a new keypair and sync the keys using `anchor keys sync`

My `Anchor.TOML [programs.localnet]` address will be different than yours and my `[provider] wallet=` path will be different. You will need to change this to wherever your keygen path is stored.

# Writing The Anchor Contract

`programs/anchor-counter/src/lib.rs`

Not going to explain much here on the specifics of the anchor contract since it's pretty well documented in the SolDev app course section on Anchor [here](https://www.soldev.app/course/intro-to-anchor). The contract I've written is different than the one in the course. I chose to write this one using a PDA representing global state because it made way more practical sense to me in something I would do in an actual program.

The first `#[program]` macro is our entry point and defines the 2 functions our program has. `Initialize` to initialize the counter Program Derived Address and `Increment` to increment the counter account by one.

Each function takes a `Context` parameter specified by a type generic. We define these type generics in structs using the `[#derive(Accounts)]` macro. The way I like to think about this is just as the accounts that the function needs to take in. Each function should have a corresponding `Context` struct.

The `Initialize` struct uses the `#[account(...)]` macro to create the [Program Derived Address](https://www.soldev.app/course/pda). PDAs are nice because you can deterministically find them again later on the client side, etc. It also defines the `user` account as a `Signer` which means that public key must sign the transaction with its corresponding private key. This must be annotated as mutable because lamports (money) will be deducted from the account to pay the transaction fee and to initialize the account. The `system_program` is necessary for actually creating the counter account.

```
pub struct Initialize<'info> {
    #[account(init, payer=user, space = 8+8, seeds = [b"counter"], bump)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
```

The `Increment` struct is pretty straightforward. It takes in the `counter` account we initialized, specifying the seeds used to create it so it will not accept a different counter account.

# Unit Tests in Rust for anchor-counter

`programs/anchor-counter/tests/counter_test.rs`
Run the tests with `cargo test-sbf`

I found the unit tests in Rust using the `solana-program-test` crate to be awesome! There's very little to no documentation about it though, so I'm going to try to be detailed here. I patched this together from an old article, the crate source, and the MarginFi Github.

The 2 key structs are the `ProgramTest` and the `ProgramTestContext` from the `solana_program_test` crate. We can get a fresh `ProgramTestContext`, which is basically the state of our local test blockchain, for each test. This is super useful and my expectation for unit testing.

I wrote a helper struct called `SetUpTest` and implemented the new function to return a `validator: ProgramTest`, a optional funded `user: Keypair` and the `counter_pda: Pubkey` for each test. You can start / get a new context with `validator.start_with_context()`.

## impl SetUpTest

The first thing that took me forever to figure out was how the hell to get a local validator in the test environment. Eventually from scouring through stuff I realized the key lies in the `ProgramTest` struct. You can create a `ProgramTest` struct using the name of your program and the program_id. You can just pass `None` as the built-in function and it will automatically pattern match to find the entrypoint. I banged my head on this for awhile until I drilled down to the source and found the `match` clause that was doing this.

Lines 201-203 create our validator.

Lines 207-214 add an optional user to our account we can use to sign transactions. We add a random keypair and give it some funds. You actually do not need to do this as the `ProgramTestContext` will come with a `payer` Keypair that is funded, but it's useful to know if you want to test transactions using a different account.

Line 217 finds the PDA we created in our contract using the "counter" seed and our program address. Note our `anchor_counter` create is available which holds the `ID` constant and other nice types we will use later.

## test_initialize

Let's see if we can get our Counter account to initialize in a test environment. Solana `Transactions` take a list of `Instructions`. We will also need a `recent_blockhash` and a `Signer` with some SOL to sign and pay the tx fee.

Line 19 starts the `ProgramTestContext` and gives us a locally running validator which contains a `Client`, `payer: Keypair`, and `last_blockhash`.

We now need to create our initialize `Instruction`. We can use the `solana_program` crate to create an `Instruction` struct. Within the `Instruction`, we will use our types created in the `anchor_counter::accounts` create to create the `Initialize` struct, which is the accounts needed for the `Initialize` function in our program. The type expectation for `Instruction` is actually `Vec<AccountMeta, Global>`, but anchor provides us with a super handy `to_account_metas` function to make sure the types are correct.

All `Instruction` types require a data field as well, which is any additional parameters passed into the function. We have none of these, but we still need to provide it. Anchor provides us with a `data()` function on the `anchor_counter::instruction` crate for each of our instructions. We are using the `Initialize` instruction here which anchor automatically creates for us based on our program.

Next, we need to create the `Transaction` to send to the blockchain via our client. We use the `solana_sdk::Transaction` struct for this. There are a few different methods on `Transaction` that allow you to create a new `Transaction` which you can explore in the source. I found the most straightforward one is the `new_signed_with_payer`, which as it sounds creates a new `Transaction` with the list of instructions,is signed by the specified signing Keypairs, and specifies the `Keypair` that is going to pay the transaction fee.

Next we send the transaction to the chain using the `banks_client` object provided by our `ProgramTestContext`. I found the `process_transaction` method to be the easiest to work with, but there are a few other options to explore in the source that have to do with different commitment levels (I talk about commitment levels in the client side interaction repo).

Finally, we want to read our data from our client account and make sure the `count` is 0 and the account was initialized. I had a real struggle figuring out how to do this, but eventually found the `load_and_deserialize` method in the MarginFi Github. My basic understanding is that the `Account` object contains a field called data which is just a Vector of u8 bytes. To get this into a human readable format, we need to deserialize it into a specified type that implements the `AccountDeserialze` trait. For us, this is `anchor_counter::Counter`.

Line 259 specifies the `load_and_deserialze` function. We take the context and the address we are interested in. We retrieve the account using the client from the context and then unwrap it a few times to get the generic `Account` struct. Then, taking the type we specified, we deserialize the `data` field of the `Account` into our `Counter` struct.

## test_increment

Very similar to our `test_initialize` function except we also want to call our `Increment` function to increment the `Counter` account by 1.

The super cool thing here is the way Solana transactions work you can pass in a list of instructions to execute in one atomic transaction. We do that hear by creating both the `init_txt` and the `increment_tx` and then passing these into the `Transaction::new_signed_with_payer` function.

Also note how I decided to use `context.payer` here to sign and execute the transaction. This is just showing the alternative option that the `context` comes with a prefunded `payer` Keypair.

## test_bogus_counter_acct

I wanted to include an example where the transaction is expected to fail. This is an example where instead of passing the proper counter account, we pass in the user account pubkey as the counter account also. This will fail because the program will correctly deduce that the account is not owned by the proper program. The program expects the account passed to be owned by itself. If you try to pass in the `bogus_pda` account that uses the wrong seed, you will get an error that the account doesn't exist/hasn't be initialized.

# Typescript Tests for anchor-counter

`tests/anchor-counter.ts`
Run the tests with `anchor test`

I did not like writing unit tests with typescript because I couldn't figure out how to reset the state of the blockchain between tests. Therefore, each unit test is dependent upon the previous one. I think this is pretty shit for unit tests so I'll probably write all my unit tests in Rust and just keep typescript stuff for when I'm trying to build a front end.

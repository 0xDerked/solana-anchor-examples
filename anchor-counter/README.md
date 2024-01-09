# Set Up Stuff

You might need to generate a new keypair and sync the keys using `anchor keys sync`

My `Anchor.TOML [programs.localnet]` address will be different than yours and my `[provider] wallet=` path will be different. You will need to change this to wherever your keygen path is stored.

-   Run the rust unit tests with `cargo test-sbf`
-   Run the typescript tests with `anchor test`
-   Can use None as the processor and will still be fine with cargo test-sbf (this confused me for a long time and I still don't really understand why it complains about the typing when I try to use the processor! macro, but whatever)
-   Took the load_and_deserialize with the type generic from the MarginFi test suite

# Notes

-   Unable to reset the ledger state between tests in Typescript.

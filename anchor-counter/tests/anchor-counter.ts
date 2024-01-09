import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { AnchorCounter } from '../target/types/anchor_counter'
import { expect } from 'chai'

describe('anchor-counter', () => {
    // Configure the client to use the local cluster.

    const provider = anchor.AnchorProvider.local()
    anchor.setProvider(provider)
    const program = anchor.workspace.AnchorCounter as Program<AnchorCounter>

    it('initializes the counter', async () => {
        const [counter, _] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('counter')],
            program.programId,
        )

        //anchor automatically fills the user of Account type Signer with the provider and the SystemProgram
        await program.methods
            .initialize()
            .accounts({
                counter,
            })
            .rpc()

        const counterAccount = await program.account.counter.fetch(counter)
        expect(counterAccount.count.toNumber()).to.equal(0)

        console.log('counter address', counter.toBase58())
    })

    it('increments the counter', async () => {
        const [counter, _] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from('counter')],
            program.programId,
        )

        await program.methods
            .increment()
            .accounts({
                counter,
            })
            .rpc()

        const counterAccount = await program.account.counter.fetch(counter)
        expect(counterAccount.count.toNumber()).to.equal(1)

        await program.methods
            .increment()
            .accounts({
                counter,
            })
            .rpc()

        const counterAccount2 = await program.account.counter.fetch(counter)
        expect(counterAccount2.count.toNumber()).to.equal(2)
    })

    // it('will this fail if we put in a random counter account', async () => {
    //     await program.methods
    //         .increment()
    //         .accounts({
    //             counter: provider.
    //         })
    //         .rpc()
    // })
})

import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { SolanaAnchorDemo } from '../target/types/solana_anchor_demo'

describe('solana-anchor-demo', () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env())

    const program = anchor.workspace
        .SolanaAnchorDemo as Program<SolanaAnchorDemo>

    it('Is initialized!', async () => {
        const tx = await program.methods.initialize().rpc()
        console.log('Your transaction signature', tx)
    })
})

import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Escrow } from '../target/types/escrow';

describe('escrow', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Escrow as Program<Escrow>;

  it('Lets make escrow!', async () => {
    // Add your test here.
    const tx = await program.methods
      .make(new anchor.BN(1), new anchor.BN(1), new anchor.BN(1))
      .accounts({})
      .signers([])
      .rpc();
    console.log('Your transaction signature', tx);
  });
});

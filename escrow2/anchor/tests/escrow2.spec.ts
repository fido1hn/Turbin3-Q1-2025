import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { Keypair } from '@solana/web3.js'
import { Escrow2 } from '../target/types/escrow2'

describe('escrow2', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider)
  const payer = provider.wallet as anchor.Wallet

  const program = anchor.workspace.Escrow2 as Program<Escrow2>

  const escrow2Keypair = Keypair.generate()

  it('Initialize Escrow2', async () => {
    await program.methods
      .initialize()
      .accounts({
        escrow2: escrow2Keypair.publicKey,
        payer: payer.publicKey,
      })
      .signers([escrow2Keypair])
      .rpc()

    const currentCount = await program.account.escrow2.fetch(escrow2Keypair.publicKey)

    expect(currentCount.count).toEqual(0)
  })

  it('Increment Escrow2', async () => {
    await program.methods.increment().accounts({ escrow2: escrow2Keypair.publicKey }).rpc()

    const currentCount = await program.account.escrow2.fetch(escrow2Keypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Increment Escrow2 Again', async () => {
    await program.methods.increment().accounts({ escrow2: escrow2Keypair.publicKey }).rpc()

    const currentCount = await program.account.escrow2.fetch(escrow2Keypair.publicKey)

    expect(currentCount.count).toEqual(2)
  })

  it('Decrement Escrow2', async () => {
    await program.methods.decrement().accounts({ escrow2: escrow2Keypair.publicKey }).rpc()

    const currentCount = await program.account.escrow2.fetch(escrow2Keypair.publicKey)

    expect(currentCount.count).toEqual(1)
  })

  it('Set escrow2 value', async () => {
    await program.methods.set(42).accounts({ escrow2: escrow2Keypair.publicKey }).rpc()

    const currentCount = await program.account.escrow2.fetch(escrow2Keypair.publicKey)

    expect(currentCount.count).toEqual(42)
  })

  it('Set close the escrow2 account', async () => {
    await program.methods
      .close()
      .accounts({
        payer: payer.publicKey,
        escrow2: escrow2Keypair.publicKey,
      })
      .rpc()

    // The account should no longer exist, returning null.
    const userAccount = await program.account.escrow2.fetchNullable(escrow2Keypair.publicKey)
    expect(userAccount).toBeNull()
  })
})

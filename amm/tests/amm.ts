import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { createMint, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Amm } from "../target/types/amm";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const { connection } = provider;
  const wallet = provider.wallet as anchor.Wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.Amm as Program<Amm>;

  const admin = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const mintX = await createMint(
      connection,
      wallet.payer,
      admin.publicKey,
      null,
      6
    );
    const mintY = await createMint(
      connection,
      wallet.payer,
      admin.publicKey,
      null,
      6
    );

    const seed = new BN(1, 10, "le");

    const ix = await program.methods
      .initialize(seed, 150, provider.publicKey)
      .accountsPartial({
        initializer: wallet.publicKey,
        mintX,
        mintY,
        systemProgram: SYSTEM_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .instruction();

    const blockhash = await connection.getLatestBlockhash();

    const tx2 = new anchor.web3.Transaction({
      feePayer: wallet.publicKey,
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
    }).add(ix);

    const txSig2 = await anchor.web3.sendAndConfirmTransaction(
      connection,
      tx2,
      [wallet.payer]
    );
    console.log("Your transaction signature", txSig2);
  });
});

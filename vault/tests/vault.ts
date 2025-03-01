import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Vault } from '../target/types/vault';
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { confirmTransaction } from "@solana-developers/helpers";
import { BN } from 'bn.js';

describe('vault', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Vault as Program<Vault>;
  const connection = program.provider.connection;

  const user = anchor.web3.Keypair.generate();
  let vault;
  let vaultState;
  let vaultBump;
  let vaultStateBump;

  before(async () => {
    await airdrop(connection, user.publicKey, 100);

    [vaultState, vaultStateBump] = PublicKey.findProgramAddressSync([
        Buffer.from("state"),
        user.publicKey.toBuffer()
    ], program.programId);
    console.log("Vault State Address: ", vaultState);
    
    [vault, vaultBump] = PublicKey.findProgramAddressSync([
        vaultState.toBuffer(),
    ], program.programId);
    console.log("Vault Address: ", vault);

  });

  it('Vault Is initialized!', async () => {
    // put some sol in the vault to ensure it is not gabage collect
    // the vault is not a pda so anchor does not handle
    // rent exception ginko ginko
    await airdrop(connection, vault, 1);

    const tx = await program.methods
        .initialize()
        .accountsPartial({
            signer: user.publicKey,
            vaultState: vaultState,
            vault: vault
        })
        .signers([user])
        .rpc();
    console.log('Your transaction signature', tx);
  });
  
  it('Deposit Is Successful!', async () => {
    const tx = await program.methods
        .deposit(new BN(10))
        .accountsPartial({
            signer: user.publicKey,
            vaultState: vaultState,
            vault: vault
        })
        .signers([user])
        .rpc();
    console.log('Your transaction signature', tx);
  });
  
  it('Withdraw Is Successful!', async () => {
    const tx = await program.methods
        .withdraw(new BN(10))
        .accountsPartial({
            signer: user.publicKey,
            vaultState: vaultState,
            vault: vault
        })
        .signers([user])
        .rpc();
    console.log('Your transaction signature', tx);
  });
  
  it('Vault is Closed Is Successful!', async () => {
    const tx = await program.methods
        .closeVault()
        .accountsPartial({
            signer: user.publicKey,
            vaultState: vaultState,
            vault: vault
        })
        .signers([user])
        .rpc();
    console.log('Your transaction signature', tx);
  });

});


async function airdrop(connection, address: PublicKey, amount: number) {
    let airdrop_signature = await connection.requestAirdrop(
      address,
      amount * LAMPORTS_PER_SOL
    );

    let confirmedAirdrop = await confirmTransaction(connection, airdrop_signature, "confirmed");
  
    return confirmedAirdrop;
  }
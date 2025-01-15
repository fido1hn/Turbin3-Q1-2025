import wallet from '../dev-wallet.json';
import { createUmi } from '@metaplex-foundation/umi-bundle-defaults';
import {
  createMetadataAccountV3,
  CreateMetadataAccountV3InstructionAccounts,
  CreateMetadataAccountV3InstructionArgs,
  DataV2Args,
} from '@metaplex-foundation/mpl-token-metadata';
import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from '@metaplex-foundation/umi';
import { bs58 } from '@coral-xyz/anchor/dist/cjs/utils/bytes';

// Define our Mint address
const mint = publicKey('HvSPE9akGcEZ3GwPYWczcAEayXrn2Pid2hFUcLJwk4qp');

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

(async () => {
  try {
    // Start here
    let accounts: CreateMetadataAccountV3InstructionAccounts = {
      mint,
      mintAuthority: signer,
    };

    let data: DataV2Args = {
      name: 'Hip Hop',
      symbol: 'HIP',
      uri: 'https://arweave.net/123',
      sellerFeeBasisPoints: 100,
      creators: null,
      collection: null,
      uses: null,
    };

    let args: CreateMetadataAccountV3InstructionArgs = {
      data,
      isMutable: false,
      collectionDetails: null,
    };

    let tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    let result = await tx.sendAndConfirm(umi);
    console.log(bs58.encode(result.signature));
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();

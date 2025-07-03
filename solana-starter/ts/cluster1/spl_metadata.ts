import wallet from "../../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";

// Define our Mint address
const mint = publicKey("8JLE1Pi4yNDJ9LLe1TrpGqJJJDUg1WPGP3btz879YHCW")

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
            updateAuthority: signer,
        }
        let data: DataV2Args = {
            name: "TilumbuBoy",
            symbol: "TLB",
            uri: "https://app.ardrive.io/#/file/08f310da-070b-4e9f-a6d9-e2b641ae9b09/view",
            sellerFeeBasisPoints: 5, // 5%
            creators: null, // No creators
            collection: null, // No collection
            uses: null // No uses
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: true,
            collectionDetails: null, // No collection details
        //     ???
        }

        let tx = createMetadataAccountV3(
             umi,
             {
                 ...accounts,
                 ...args
             }
         )

        let result = await tx.sendAndConfirm(umi);
        console.log(bs58.encode(result.signature));
        console.log(`Metadata account created for mint: ${mint.toString()}`);
        console.log(`Transaction signature: ${bs58.encode(result.signature)}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

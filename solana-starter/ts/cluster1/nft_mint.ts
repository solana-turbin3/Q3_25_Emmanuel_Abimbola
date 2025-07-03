import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";
import wallet from "../../turbin3-wallet.json"
import {base58} from "@metaplex-foundation/umi/serializers";

// create the traditional rpc connection
const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

// the keypair is then gotten from our wallet.
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);

// this gives UMI access to use the import functions, and also brings in the metaplex shii
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);
// const metadataUri = "https://app.ardrive.io/#/file/08f310da-070b-4e9f-a6d9-e2b641ae9b09/view";
//const metadataUri = "https://gateway.irys.xyz/CRiNDSV91Ve2doq6ftePNgoQoU6RxrbxcENWEJP3zuJr";
const metadataUri = "https://devnet.irys.xyz/2sjRzNdC2FJy3cWKfewZ9WiDuMrXVWr1EYfcHCVjSP3N";

(async () => {

    // const nft =
    let tx = createNft(umi, {
        mint,
        name: "TrumpFarm",
        symbol: "$TFF",
        uri: metadataUri,
        sellerFeeBasisPoints: percentAmount(5.5) 
});

    // the docs offers some better implementation which I used here. Using the deserializer.
    let result = await tx.sendAndConfirm(umi);
    console.log(base58.deserialize(result.signature));
    
    // const signature = base58.encode(result.signature);
    //console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();
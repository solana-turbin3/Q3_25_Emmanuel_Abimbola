import wallet from "../../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFileFromJson, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { create } from "domain";

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        //const image = "https://gateway.irys.xyz/CRiNDSV91Ve2doq6ftePNgoQoU6RxrbxcENWEJP3zuJr";
        const image = "https://devnet.irys.xyz/2sjRzNdC2FJy3cWKfewZ9WiDuMrXVWr1EYfcHCVjSP3N";
        const metadata = {
            name: "EuroJohnson",
            symbol: "EUJ",
            description: "Family and friends alike. They all love this brilliant figure. Become a part of the revolution today.",
            image,
            primarySaleHappened: false,
             properties: {
                 files: [
                     {
                         type: "image/png",
                       uri: image
                     },
                 ]
             },
             creators: [{
                "address": signer.publicKey,
                "share": "90"
             }]
         };

         // This is the wrong implementation of the code for folks that have a hardtime learning it.
         // const Metadata = await createGenericFileFromJson(metadata);
         //const uploader = await umi.uploader.upload(metadata)

        // I commented it out because I wanted it to persist while still having ability to use. Btw, the commented is the wrong implementation
        //const myUri = "https://gateway.irys.xyz/CRiNDSV91Ve2doq6ftePNgoQoU6RxrbxcENWEJP3zuJr"
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

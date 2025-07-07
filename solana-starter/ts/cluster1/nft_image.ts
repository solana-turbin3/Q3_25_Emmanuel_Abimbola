import wallet from "../../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
// import { assertByteArrayIsNotEmptyForCodec } from "gill"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

//this takes the keypair and makes manipulation possible. Lol!
let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

// Irys is a provider that makes easy uploading of files for use
umi.use(irysUploader());
umi.use(signerIdentity(signer));

// This function loads and uploads the image file to the Irys database
(async () => {
    try {
        //1. Load image
        // for this, y'all gonna get a nice photo of me in my 20s.
        const imagePath = await readFile("cluster1/shrinath_rug.png");

        //2. Convert image to generic file.
        const image = createGenericFile(
            imagePath, 
            "generug.png", 
            {contentType: "image/png"
            });

        //3. Upload image
        const [myUri] = await umi.uploader.upload([image]);
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();


// For Improvement, consider adding a buffer/rate-limit. The code has the potential to run for so long without any stopping.
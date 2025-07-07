// The task for today is to replace @solana/web3.js and other bundles with gill. For sake of continuity, I will preserve the original code in comments while making use of gill by the side.
// I will appreciate PRs to help solve this problem. This program does all the steps from init to metadata to minting.

// Using gill only.
import { createSolanaClient, generateKeyPairSigner, SendAndConfirmTransactionWithSignersFunction, getExplorerLink, createTransaction, type KeyPairSigner, getMinimumBalanceForRentExemption, signTransactionMessageWithSigners, getSignatureFromTransaction, sendAndConfirmTransactionWithSignersFactory, signAndSendTransactionMessageWithSigners, sendAndConfirmTransactionFactory } from 'gill';
import { loadKeypairSignerFromFile } from "gill/node";
import { getCreateAccountInstruction, getCreateMetadataAccountV3Instruction, getTokenMetadataAddress } from 'gill/programs';
import { TOKEN_2022_PROGRAM_ADDRESS, getInitializeMintInstruction, getMintSize } from 'gill/programs/token';

//import { createMint } from '@solana/spl-token';
//import wallet from "..//../turbin3-wallet.json"
//import { Keypair, Connection, Commitment } from "@solana/web3.js";
//import { Keypair, sendAndConfirmTransaction } from '@solana/web3.js';


// using gill to allocate the necessary imports
const {rpc, rpcSubscriptions} = createSolanaClient({urlOrMoniker: "devnet"});
const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({rpc,rpcSubscriptions});
const tokenProgram = TOKEN_2022_PROGRAM_ADDRESS;
const space = getMintSize();


(async () => {
    try {
        
        const {value: latestBlockhash} = await rpc.getLatestBlockhash().send();

        const signer: KeyPairSigner = await loadKeypairSignerFromFile("../turbin3-wallet.json");
        const mint = await generateKeyPairSigner();
        const metadataAddress = await getTokenMetadataAddress(mint);

        const tx = createTransaction({
            feePayer: signer,
            version: "legacy",
            instructions: [
                getCreateAccountInstruction({
                    space,
                    lamports: getMinimumBalanceForRentExemption(space),
                    newAccount: mint,
                    payer: signer,
                    programAddress: tokenProgram,
                }),
                getInitializeMintInstruction({
                    mint: mint.address,
                    mintAuthority: signer.address,
                    freezeAuthority: signer.address,
                    decimals: 9,
                }), 
                getCreateMetadataAccountV3Instruction( {
                    collectionDetails: null,
                    isMutable: true,
                    updateAuthority: signer,
                    mint: mint.address,
                    metadata: metadataAddress,
                    mintAuthority: signer,
                    payer: signer,
                    data: {
                        sellerFeeBasisPoints: 5.5,
                        collection: null,
                        creators: null,
                        uses: null,
                        name: "Shrinath Aristotelian",
                        symbol:"SHA",
                        uri: "https://gateway.irys.xyz/G9Q2eJ5EgyoA6EMcSTdsYJnWeCDzx2TQWjrBQbmPCKr2",
                    },
                }),
            ],
            latestBlockhash
        });
        //  const sendAndConfirm = await sendAndConfirmTransactionWithSignersFactory({rpc,rpcSubscriptions});
        // console.log(sendAndConfirm);

        // signed the transactions successfully, but the problem is getting it to on-chain and 
        const signedTransaction = await signTransactionMessageWithSigners(tx);
        // const signature = getSignatureFromTransaction(signedTransaction);

        console.log("Mint address:", mint.address.toString());
console.log("Signer address:", signer.address.toString());
console.log("Metadata address:", metadataAddress.toString());
console.log("Rent-exempt lamports:", getMinimumBalanceForRentExemption(space));
console.log("Token program:", tokenProgram.toString());
        
        /*try {
            const signedMessage = await signTransactionMessageWithSigners(tx);
            await sendAndConfirmTransaction(signedMessage);
            console.log("\nCheck the Explorer for your TXN:", getExplorerLink({cluster: "devnet", transaction: getSignatureFromTransaction(signedMessage)}));
        }
            catch (error){
            console.log(`Unable to send and confirm the transaction. ${error}`)};
      const signedTx = await signTransactionMessageWithSigners(tx);
        const signatu = await sendAndConfirmTransaction(signedTx, );
         console.log(`This is the output ${signedTx}`);
        */
                
    }
    catch(err) {
        console.log(`\n Something went wrong. Oops!: ${err}`);
    }
})()



// The original spl_init continues here.

// Import our keypair from the wallet file
//const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
//const commitment: Commitment = "confirmed";
//const connection = new Connection("https://api.devnet.solana.com", commitment);

/* (async () => {
    try {
        // Start here
        const mint = await createMint(connection, keypair, keypair.publicKey, null, 6);
        console.log(`MInt Address: ${mint.toBase58()}`); 
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
*/
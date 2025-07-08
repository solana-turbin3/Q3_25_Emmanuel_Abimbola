/*import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import {createTransaction, createSolanaClient, signTransactionMessageWithSigners} from "gill"
import wallet from "../../turbin3-wallet.json"
import {
    buildCreateTokenTransaction,
    build
} from "gill/program/token"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
*/

import {buildTransferTokensTransaction, TOKEN_2022_PROGRAM_ADDRESS} from "gill/programs/token";
import {createSolanaClient, address, signTransactionMessageWithSigners, getExplorerLink, getSignatureFromTransaction} from "gill";
import {loadKeypairSignerFromFile} from "gill/node";
// import { simulateTransaction } from "@coral-xyz/anchor/dist/cjs/utils/rpc";
//import { globalAgent } from "http";
// import { signerIdentity } from "@metaplex-foundation/umi";

// To get more detailed logs
global.__GILL_DEBUG__ = true;
global.__GILL_DEBUG_LEVEL__ = "debug";
// We're going to import our keypair from the wallet file
// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));



//Create a Solana devnet connection
// const commitment = "confirmed"; // or "processed" or "finalized"
const {rpc, sendAndConfirmTransaction} = createSolanaClient({urlOrMoniker: "devnet",});

// Mint address
const mint = address("8JLE1Pi4yNDJ9LLe1TrpGqJJJDUg1WPGP3btz879YHCW");
const tokenProgram = TOKEN_2022_PROGRAM_ADDRESS; 
// const mint = new PublicKey("D1Ni3fZWoPA3B;

// Recipient address
const transferToDestination = address("3rLojDoVPUKBXnr2vybXJA3Lh3wKBxxDHx7fxJn8m3ZM");


(async () => {
    try {

        const signer = await loadKeypairSignerFromFile("../turbin3-wallet.json");
        const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

        // Get the token account of the fromWallet address, and if it does not exist, create it
        const transferTx = await buildTransferTokensTransaction ({
            feePayer: signer,
            version: "legacy",
            latestBlockhash,
            mint,
            // mint: tokenProgram,
            authority: signer,
            amount: 1,
            destination: transferToDestination,
            tokenProgram,
        });


        //console.log("Transaction to mint tokens:");
        //console.log(transferTx);

        const signedTransaction = await signTransactionMessageWithSigners(transferTx);
        try {console.log( "Sending transaction:",getExplorerLink({cluster: "devnet",transaction: getSignatureFromTransaction(signedTransaction),

        }),);
        await sendAndConfirmTransaction(signedTransaction);
        console.log("Transaction confirmed!"
        );} 
        catch (err) {
            console.error("Unable to send and confirm the transaction"); 
            console.error(err);
        }
        /*const signature =getSignatureFromTransaction(signedTransaction);

        console.log("\nExplorer Link for transferiing the tokens:");
        //const signature: string = await sendAndConfirmTransaction(signedTransaction);
        console.log(getExplorerLink({ cluster: "devnet", transaction: signature}));

        const success = await sendAndConfirmTransaction(signedTransaction);
        console.log("Successful", success); 
        */

        // Get the token account of the toWallet address, and if it does not exist, create it

        // Transfer the new token to the "toTokenAccount" we just created
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }

    

})();   
import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, PublicKey } from "@solana/web3.js"
import wallet from "../../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { transferOutOfEscrow } from "@metaplex-foundation/mpl-token-metadata";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("8JLE1Pi4yNDJ9LLe1TrpGqJJJDUg1WPGP3btz879YHCW");

// Recipient address
const to = new PublicKey("FzozXYEUfyuSPckCmyh5Y73b9KZe1ZjAXzfsuRCD3Ct");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenAcct = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            keypair.publicKey
        );
        
        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenAcct = await getOrCreateAssociatedTokenAccount(
            connection, 
            keypair, 
            mint, 
            to
        );
        


        // Transfer the new token to the "toTokenAccount" we just created
        // console.log(`Transferring 1 token from ${fromTokenAcct.address.toBase58()} to ${toTokenAcct.address.toBase58()}`);
        // Use the transfer function from @solana/spl-token
        // Note: The transfer function requires the amount to be in the smallest unit, so we multiply by LAMPORTS_PER_SOL

        const signature = await transfer( 
            connection, 
            keypair,  
            fromTokenAcct.address, 
            toTokenAcct.address, 
            keypair.publicKey, 
            1n);
        // const signature = await transferOutOfEscrow(connection, keypair, fromTokenAcct.address, toTokenAcct.address, keypair.publicKey, 1 * LAMPORTS_PER_SOL);

        console.log(`Transfer success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);

        
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
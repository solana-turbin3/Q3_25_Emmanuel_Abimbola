import {Connection, Keypair, LAMPORTS_PER_SOL} from "@solana/web3.js";
import wallet from "./dev-wallet.json" with {type : "json"}// Import the wallet JSON file, and specify the type as JSON

const keypair = Keypair.fromSecretKey(Uint8Array.from(wallet));

// Creating a Solana devnet connection.
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        // We gonna claim 2 devnet SOL tokens
        const txhash = await connection.requestAirdrop(
            keypair.publicKey,
            2 * LAMPORTS_PER_SOL
        );
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops! Something went wrong: ${e}`);
    }
})();
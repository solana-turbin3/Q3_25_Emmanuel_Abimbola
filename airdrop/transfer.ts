// I am enjoying this already. Voila!!!
import { Transaction, SystemProgram, LAMPORTS_PER_SOL, Connection, Keypair, sendAndConfirmTransaction, PublicKey } from '@solana/web3.js';
import wallet from './dev-wallet.json' with { type: 'json' }; // Import the wallet JSON file, not making the mistake I made earlier.

const from = Keypair.fromSecretKey(Uint8Array.from(wallet));
const to = new PublicKey('2QkJLTKTtYFHS6xir1TEXLSdajM7r1Djf96JogKnRGSR'); // Replace with the recipient's public key

const connection = new Connection('https://api.devnet.solana.com'); // this code inits a solana devnet connection

/*
// the following program is to transfer SOL from my wallet to another
(async () => {
    try {
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: LAMPORTS_PER_SOL / 100, // Transferring 0.01 SOL
            })
        );
        transaction.recentBlockhash = (
            await connection.getLatestBlockhash('confirmed')
        ).blockhash;
        transaction.feePayer = from.publicKey; // this block of code sets the fee payer to the sender's public key

        const signature = await sendAndConfirmTransaction(connection, transaction, [from]);
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (l) {
        console.error(`Oops! Something went wrong ${l}`); // I decided to use l instead of e, just for fun.
    }
})(); */

/// Now we will empty this wallet and close the account
(async () => {
    try {
        const balance = await connection.getBalance(from.publicKey);
        console.log(`Current balance: ${balance} lamports`);
        
        // Get minimum balance for rent exemption (for a basic account)
        const rentExemptBalance = await connection.getMinimumBalanceForRentExemption(0);
        console.log(`Minimum rent-exempt balance: ${rentExemptBalance} lamports`);
        
        // Reserve rent-exempt balance plus some buffer for transaction fees
        const reserveAmount = rentExemptBalance + 5000; // rent exemption + fee buffer
        const transferAmount = balance - reserveAmount;
        
        if (transferAmount <= 0) {
            console.log(`Cannot transfer. Need to reserve ${reserveAmount} lamports for rent exemption and fees.`);
            return;
        }
        
        console.log(`Transferring: ${transferAmount} lamports`);
        
        const actualTransaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to,
                lamports: transferAmount,
            })
        );
        actualTransaction.recentBlockhash = (
            await connection.getLatestBlockhash('confirmed')
        ).blockhash;
        actualTransaction.feePayer = from.publicKey;

        const signature = await sendAndConfirmTransaction(connection, actualTransaction, [from]);
        console.log(`Success! Transferred maximum possible amount. Check TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`)
        
        // Check remaining balance
        const remainingBalance = await connection.getBalance(from.publicKey);
        console.log(`Remaining balance: ${remainingBalance} lamports (${remainingBalance / LAMPORTS_PER_SOL} SOL)`);
        
    } catch (l) {
        console.error(`Oops! Something went wrong:`, l)
    }
})();
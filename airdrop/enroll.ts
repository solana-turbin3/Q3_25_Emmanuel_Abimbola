// the imports to be used in the code
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { Buffer } from "buffer";
import idl from "./programs/Turbin3_Prereq.json" with { type: "json" };
import wallet from "./personal-wallet.json" with { type: "json" }; // Import the personal wallet JSON file

// all the constants we will use in the code
const MPL_CORE_PROGRAM_ID = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"); // The MPL Core Program ID
const TURBIN3_PROGRAM_ID = new PublicKey(idl.metadata.address);

const connection = new Connection("https://api.devnet.solana.com", "confirmed"); // Connect to Solana Devnet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet)); // Load keypair from wallet JSON
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" }); // Create Anchor provider

const program = new Program(idl, TURBIN3_PROGRAM_ID, provider); // Create the program client

const account_seeds = [Buffer.from("prereqs"), keypair.publicKey.toBuffer()]; // PDA seeds for your prereq account
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId); // Derive PDA

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2"); // Replace with actual collection key
const mintTs = Keypair.generate(); // Generate a keypair for the mint token

// all the functions we will use in the code

// initialize the account with your GitHub handle
(async () => {
  try {
    const txHash = await program.methods
      .initialize("Neocryptoquant")
      .accounts({
        user: keypair.publicKey,
        account: account_key,
        systemProgram: SystemProgram.programId,
      })
      .signers([keypair])
      .rpc();

    console.log(
      `Success! Check the TX here: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
    );
  } catch (e) {
    console.error(`Oops! Initialization failed: ${e}`);
  }
})();

// submit token standard and mint
(async () => {
  try {
    const [authority] = PublicKey.findProgramAddressSync(
      [Buffer.from("collection"), mintCollection.toBuffer()],
      program.programId
    );

    const txhash = await program.methods
      .submitTs()
      .accounts({
        user: keypair.publicKey,
        account: account_key,
        mint: mintTs.publicKey,
        collection: mintCollection,
        authority,
        mplCoreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram: SystemProgram.programId, // ✅ Use `.programId`
      })
      .signers([keypair, mintTs])
      .rpc();

    console.log(
      `✅ Token standard submitted! TX: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
    );
  } catch (e) {
    console.error(`❌ submitTs failed: ${e}`);
  }
})();

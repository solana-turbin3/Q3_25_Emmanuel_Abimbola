import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { EscrowQ3 } from "../target/types/escrow_q3";
import { Keypair, SystemProgram, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { createMint, getOrCreateAssociatedTokenAccount, mintTo, getAccount } from "@solana/spl-token";

describe("escrow_q3", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.escrowQ3 as Program<EscrowQ3>;
  const provider = anchor.getProvider();

  // Keypairs
  const maker = Keypair.generate();
  const taker = Keypair.generate();

  // Test values
  const seed = new anchor.BN(123);
  const deposit = new anchor.BN(1000);
  const receive = new anchor.BN(500);

  let mintA: PublicKey;
  let mintB: PublicKey;
  let makerAtaA: PublicKey;
  let takerAtaA: PublicKey;

  it("Funds test accounts", async () => {
    // Fund both accounts with SOL for rent
    for (const kp of [maker, taker]) {
      await provider.connection.requestAirdrop(kp.publicKey, 2 * LAMPORTS_PER_SOL);
    }
  });

  it("Creates mints and ATAs", async () => {
    // Create two mints
    mintA = await createMint(
      provider.connection,
      maker, // payer
      maker.publicKey, // mint authority
      null, // freeze authority
      6 // decimals
    );
    mintB = await createMint(
      provider.connection,
      maker,
      maker.publicKey,
      null,
      6
    );

    // Create ATAs for maker and taker
    makerAtaA = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker,
      mintA,
      maker.publicKey
    )).address;

    takerAtaA = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      taker,
      mintA,
      taker.publicKey
    )).address;

    // Mint tokens to maker's ATA
    await mintTo(
      provider.connection,
      maker,
      mintA,
      makerAtaA,
      maker,
      1_000_000 // 1 token with 6 decimals
    );
  });

  it("Initializes escrow", async () => {
    // Derive PDA for escrow
    const [escrowPda, escrowBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    // Derive vault PDA (if needed, or use getOrCreateAssociatedTokenAccount for escrow)
    // For this example, let's assume vault is an ATA for escrow and mintA:
    const vault = (await getOrCreateAssociatedTokenAccount(
      provider.connection,
      maker,
      mintA,
      escrowPda,
      true // allow owner off curve
    )).address;

    const tx = await program.methods
      .initialize(seed, deposit, receive)
      .accounts({
        maker: maker.publicKey,
        taker: taker.publicKey,
        mintA,
        mintB,
        makerAtaA,
        escrow: escrowPda,
        vault,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([maker, taker])
      .rpc();

    console.log("Your transaction signature", tx);

    // Optionally, fetch and check escrow state
    const escrowAccount = await program.account.escrow.fetch(escrowPda);
    console.log("Escrow state:", escrowAccount);
  });
});
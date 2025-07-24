import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Amm } from "../target/types/amm";
// import wallet from "/home/ghostgamer/.config/solana/id.json"
import { PublicKey } from "@solana/web3.js"
// import { bytes } from "@coral-xyz/anchor/dist/cjs/utils";
import { expect } from "chai";
import { SendTransactionError } from "@solana/web3.js";

import { getAccount, createMint, getAssociatedTokenAddress, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID, transfer } from "@solana/spl-token";

describe("amm", () => {
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.amm as Program<Amm>;

  const wallet = provider.wallet

  const seed = new anchor.BN(Math.random() * 10_000)
  console.log(seed.toString())
  // const seed = new anchor.BN(866);
  const seed2 = new anchor.BN(Math.random() * 100_000)

  const fee = 30
  
  //const Zoro = new PublicKey(mintX)
  let Zoro: PublicKey;
  let G5: PublicKey;
  //const Zoro = new PublicKey("6QRY8Stw4VzgGSLeTavBNAcpVNHV2ArFnpg7GKQx5vrL")
  //const G5 = new PublicKey("HHL8VZCfzdUvPp6hHq2V1h7oWxZBFGgrkyT9SCTAJ3Nm")

  const max_zoro = new anchor.BN(50_000 * 1_000_000)
  const max_g5 = new anchor.BN(50_000 * 1_000_000)

  const [config] = PublicKey.findProgramAddressSync([
    Buffer.from("config"),
    seed.toArrayLike(Buffer, "le", 8)
  ],
    program.programId,
  );
  const [lp] = PublicKey.findProgramAddressSync([
    Buffer.from("lp"),
    config.toBuffer(),
  ],
    program.programId,
  );


  before (async () => {
      Zoro = await createMint(
        provider.connection,
        wallet.payer,
        wallet.publicKey,
        null,
        6
      );
      
      G5 = await createMint(
        provider.connection,
        wallet.payer,
        wallet.publicKey,
        null,
        6
      );
      const ZoroATA = await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, Zoro, wallet.publicKey);
      const G5ATA = await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, G5, wallet.publicKey);

      await mintTo(provider.connection, wallet.payer, Zoro, ZoroATA.address, wallet.publicKey, 1_000_000_000);
      await mintTo(provider.connection, wallet.payer, G5, G5ATA.address, wallet.publicKey, 1_000_000_000);
    });

    
  it("Is initialized with authority", async () => {
    const tx = await program.methods.initialize(seed, fee, wallet.publicKey)
    .accountsPartial({
      initializer: wallet.publicKey,
      mintX: Zoro,
      mintY: G5,
      mintLp: lp,
      config: config,
    }).rpc();

    const configAccount = await program.account.config.fetch(config);
    expect(configAccount.authority.equals(wallet.publicKey)).to.be.true;
  });
  

  // it("Is initialized with authority!", 
  //   before (async () => {
  //     Zoro = await createMint(
  //       provider.connection,
  //       wallet.payer,
  //       wallet.publicKey,
  //       null,
  //       6
  //     );
  //   )
    
  //   // Add your test here.
  //   const tx = await program.methods.initialize(seed, fee, wallet.publicKey)
  //     .accountsPartial({
  //       initializer: wallet.publicKey,
  //       mintX: Zoro,
  //       mintY: G5,
  //       mintLp: lp,
  //       config: config,
  //     })
  //     .rpc();
  //   const configAccount = await program.account.config.fetch(config);
  //   expect(configAccount.authority.equals(wallet.publicKey)).to.be.true;
  //   expect(configAccount.seed.toString()).to.equal(seed.toString());
  //   expect(configAccount.fee.toString()).to.equal(fee.toString());

  //   console.log("The transactio signature", tx);
  // });

  it("fails initialize with same seed!", async () => {
    try {

      const tx = await program.methods.initialize(seed, fee, wallet.publicKey)
        .accountsPartial({
          initializer: wallet.publicKey,
          mintX: Zoro,
          mintY: G5,
          mintLp: lp,
          config: config,
        })
        .rpc();
      throw new Error("Expected initialize to fail, but it succeeded");
    } catch (e) {
      expect(e).to.be.instanceOf(SendTransactionError);
      expect(e.message).to.include("already in use");
    }
  });

  //(async () => 
  console.log(anchor.web3.PublicKey);

  it("Deposit!", async () => {
    
    const userZoroATA = await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, Zoro, wallet.publicKey);
    const userG5ATA = await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, G5, wallet.publicKey);
    const userLpATA = await getOrCreateAssociatedTokenAccount(provider.connection, wallet.payer, lp, wallet.publicKey);

    await mintTo (
    provider.connection,
    wallet.payer,
    Zoro,
    userZoroATA.address,
    wallet.publicKey,
    500_000_000,
  )

   await mintTo (
    provider.connection,
    wallet.payer,
    G5,
    userG5ATA.address,
    wallet.publicKey, // my deposit function has been giving me issues. I will love some help with it over here.
    500_000_000,
  )
    const depositAmount = 50_000;
    await transfer (
      provider.connection,
      wallet.payer,
      userZoroATA.address,
      userLpATA.address,
      // wallet.publicKey,
      Zoro,
      depositAmount,
    )
        await transfer (
      provider.connection,
      wallet.payer,
      userG5ATA.address,
      userLpATA.address,
      //wallet.publicKey,
      G5,
      depositAmount,)

    const userZoroBefore = await getAccount(provider.connection, userZoroATA.address);
    const userG5Before = await getAccount(provider.connection, userG5ATA.address);
    const userLpBefore = await getAccount(provider.connection, userLpATA.address);
    const tx = await program.methods.deposit(new anchor.BN(depositAmount), max_zoro, max_g5)
      .accountsPartial({
        user: wallet.publicKey,
        mintX: Zoro,
        mintY: G5,
        mintLp: lp,
        config: config,
      }).signers([wallet.payer])
      .rpc();
    const userZoroAfter = await getAccount(provider.connection, userZoroATA.address);
    const userG5After = await getAccount(provider.connection, userG5ATA.address);
    const userLpAfter = await getAccount(provider.connection, userLpATA.address);

    expect(Number(userZoroAfter.amount)).to.be.lessThan(Number(userZoroBefore.amount));
    expect(Number(userG5After.amount)).to.be.lessThan(Number(userG5Before.amount));
    expect(Number(userLpAfter.amount)).to.be.greaterThan(Number(userLpBefore.amount));

  });
// Check what accounts your deposit instruction needs
console.log("Deposit instruction accounts:", program.idl.instructions.find(ix => ix.name === 'deposit').accounts);
  it("fails to deposit invalid amount!", async () => {
    const depositAmount = new anchor.BN(0);

    try {
      await program.methods
        .deposit(depositAmount, max_zoro, max_g5)
        .accountsPartial({
          user: wallet.publicKey,
          mintX: Zoro,
          mintY: G5,
          mintLp: lp,
          config: config,
        })
        .rpc();
      expect.fail("Deposit should have failed due to invalid amount");
    } catch (e) {
      expect(e).to.have.property("logs");
      expect(e.logs.join("\n")).to.include("Invalid amount.");
    }
  });

  it("Swapping Zoro to Gear5!", async () => {
    const userZoroATA = await getAssociatedTokenAddress(Zoro, wallet.publicKey);
    const userG5ATA = await getAssociatedTokenAddress(G5, wallet.publicKey);
    const vaultZoro = await getAssociatedTokenAddress(Zoro, config, true);
    const vaultG5 = await getAssociatedTokenAddress(G5, config, true);

    const userZoroBefore = await getAccount(provider.connection, userZoroATA);
    const userG5Before = await getAccount(provider.connection, userG5ATA);
    const vaultZoroBefore = await getAccount(provider.connection, vaultZoro);
    const vaultG5Before = await getAccount(provider.connection, vaultG5);

    const amount = new anchor.BN(50 * 1_000_000);
    const min = new anchor.BN(500);
    const is_x = true;

    const tx = await program.methods
      .swap(is_x , amount, min)
      .accountsPartial({
        user: wallet.publicKey,
        mintX: Zoro,
        mintY: G5,
        config: config,
      })
      .rpc();

    const userZoroAfter = await getAccount(provider.connection, userZoroATA);
    const userG5After = await getAccount(provider.connection, userG5ATA);
    const vaultZoroAfter = await getAccount(provider.connection, vaultZoro);
    const vaultG5After = await getAccount(provider.connection, vaultG5);

    expect(Number(userZoroAfter.amount)).to.be.lessThan(Number(userZoroBefore.amount));
    expect(Number(userG5After.amount)).to.be.greaterThan(Number(userG5Before.amount));
    expect(Number(vaultZoroAfter.amount)).to.be.greaterThan(Number(vaultZoroBefore.amount));
    expect(Number(vaultG5After.amount)).to.be.lessThan(Number(vaultG5Before.amount));
  });

  it("Swapping Gear5 to Zoro!", async () => {
    const userZoroATA = await getAssociatedTokenAddress(Zoro, wallet.publicKey);
    const userG5ATA = await getAssociatedTokenAddress(G5, wallet.publicKey);
    const vaultZoro = await getAssociatedTokenAddress(Zoro, config, true);
    const vaultG5 = await getAssociatedTokenAddress(G5, config, true);

    const userZoroBefore = await getAccount(provider.connection, userZoroATA);
    const userG5Before = await getAccount(provider.connection, userG5ATA);
    const vaultZoroBefore = await getAccount(provider.connection, vaultZoro);
    const vaultG5Before = await getAccount(provider.connection, vaultG5);

    const amountIn = new anchor.BN(50 * 1_000_000);
    const slippage = new anchor.BN(500);

    const tx = await program.methods
      .swap(false, amountIn, slippage)
      .accountsPartial({
        user: wallet.publicKey,
        mintX: Zoro,
        mintY: G5,
        config: config,
      })
      .rpc();

    const userZoroAfter = await getAccount(provider.connection, userZoroATA);
    const userG5After = await getAccount(provider.connection, userG5ATA);
    const vaultZoroAfter = await getAccount(provider.connection, vaultZoro);
    const vaultG5After = await getAccount(provider.connection, vaultG5);

    expect(Number(userG5After.amount)).to.be.lessThan(Number(userG5Before.amount));
    expect(Number(userZoroAfter.amount)).to.be.greaterThan(Number(userZoroBefore.amount));
    expect(Number(vaultG5After.amount)).to.be.greaterThan(Number(vaultG5Before.amount));
    expect(Number(vaultZoroAfter.amount)).to.be.lessThan(Number(vaultZoroBefore.amount));
  });

  

});
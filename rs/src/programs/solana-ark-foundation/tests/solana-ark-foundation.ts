import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VeterinarySystem } from "../target/types/veterinary_system";
import { assert } from "chai";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";

describe("solana-ark-foundation (Devnet)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .VeterinarySystem as Program<VeterinarySystem>;

  let treasuryPda: PublicKey; // Global Treasury PDA
  let adminPda: PublicKey; // Global Admin PDA

  before(async () => {
    // Derive and store the Treasury PDA and Admin PDA
    treasuryPda = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    )[0];

    adminPda = PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    )[0];
  });

  it("Initializes the admin PDA on Devnet", async () => {
    try {
      const adminAccount = await program.account.adminAccount.fetch(adminPda);
      console.log("Admin PDA already exists:", adminAccount);
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log("Admin PDA not found, initializing...");
        const tx = await program.methods
          .initializeAdmin()
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();

        console.log("Transaction Signature:", tx);
      } else {
        throw err;
      }
    }
  });

  it("Initializes the Treasury PDA", async () => {
    try {
      const treasuryAccount = await program.account.treasuryAccount.fetch(
        treasuryPda
      );
      console.log("Treasury PDA already exists:", treasuryAccount);
  
      // If the Treasury PDA exists, check its balance
      if (treasuryAccount.balance.toNumber() !== 0) {
        console.log(
          "Treasury balance is already non-zero:",
          treasuryAccount.balance.toString()
        );
        return; // Skip initialization if balance is non-zero
      }
    } catch (err) {
      if (err.message.includes("Account does not exist")) {
        console.log("Treasury PDA not found, initializing...");
        const tx = await program.methods
          .initializeTreasury()
          .accounts({
            payer: provider.wallet.publicKey,
          })
          .rpc();
  
        console.log("Transaction Signature:", tx);
      } else {
        throw err;
      }
    }
  
    // Fetch the Treasury PDA again to verify its state
    const treasuryAccount = await program.account.treasuryAccount.fetch(
      treasuryPda
    );
    assert.strictEqual(
      treasuryAccount.balance.toString(),
      "0",
      "Treasury balance should be zero"
    );
    console.log(
      "Treasury successfully initialized with PDA:",
      treasuryPda.toBase58()
    );
  });

  it("Adds a new veterinary cabinet", async () => {
    const [cabinetPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("cabinet"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );
  
    const [treasuryPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("treasury")],
      program.programId
    );
  
    const [adminPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("admin")],
      program.programId
    );
  
    const cabinetDetails = {
      info: Array.from(
        new Uint8Array(Buffer.from("Paw Perfect Veterinary Clinic".padEnd(32)))
      ),
    };
  
    const payer = provider.wallet.publicKey;
    const joinFeeLamports = 800_000; // 0.8 SOL in lamports
  
    // Airdrop lamports to payer
    const txHash = await provider.connection.requestAirdrop(
      payer,
      joinFeeLamports
    );
    await provider.connection.confirmTransaction(txHash, "confirmed");
  
    try {
      const tx = await program.methods
        .addVeterinaryCabinet(cabinetDetails.info)
        .accounts({
          cabinet: cabinetPda,           // Cabinet PDA
          treasury: treasuryPda,         // Treasury PDA
          adminPda: adminPda,            // Admin PDA
          admin: provider.wallet.publicKey, // Admin signer
          payer: provider.wallet.publicKey, // Payer signer
          systemProgram: SystemProgram.programId, // System program
        })
        .rpc();
  
      console.log("Transaction Signature:", tx);
    } catch (err) {
      console.error("Transaction failed:", err);
      if ("tx" in err) {
        const logs = await provider.connection.getConfirmedTransaction(
          err.tx,
          "confirmed"
        );
        console.error("Transaction Logs:", logs.meta?.logMessages || []);
      } else {
        console.error("Error Logs Unavailable. Raw Error:", err);
      }
      throw err;
    }
  });
});
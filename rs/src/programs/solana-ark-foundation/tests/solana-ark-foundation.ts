import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VeterinarySystem } from "../target/types/veterinary_system";
import { assert } from "chai";
import { PublicKey, Keypair } from "@solana/web3.js";

describe("solana-ark-foundation (Devnet)", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
  
    const program = anchor.workspace.VeterinarySystem as Program<VeterinarySystem>;
  
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
        const treasuryAccount = await program.account.treasuryAccount.fetch(treasuryPda);
        console.log("Treasury PDA already exists:", treasuryAccount);
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
  
      const treasuryAccount = await program.account.treasuryAccount.fetch(treasuryPda);
      assert.strictEqual(
        treasuryAccount.balance.toString(),
        "0",
        "Treasury balance should be zero"
      );
      console.log("Treasury successfully initialized with PDA:", treasuryPda.toBase58());
    });
  
    it("Adds a new veterinary cabinet", async () => {
      const [cabinetPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("cabinet"), provider.wallet.publicKey.toBuffer()],
        program.programId
      );
  
      const cabinetDetails = {
        info: Array.from(new Uint8Array(Buffer.from("Paw Perfect Veterinary Clinic".padEnd(32)))),
      };
  
      const payer = provider.wallet.publicKey;
      const joinFeeLamports = 800_000; // 0.8 SOL in lamports
      const txHash = await provider.connection.requestAirdrop(payer, joinFeeLamports);
      await provider.connection.confirmTransaction(txHash, "confirmed");
      console.log("Payer funded with lamports:", joinFeeLamports);
  
      const initialTreasuryBalance = await provider.connection.getBalance(treasuryPda);
  
      try {
        const tx = await program.methods
          .addVeterinaryCabinet(cabinetDetails.info)
          .accounts({
            cabinet: cabinetPDA,
            treasury: treasuryPda,
            admin: adminPda,
            payer,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
  
        console.log("Transaction Signature:", tx);
      } catch (err) {
        console.error("Transaction failed:", err);
        if ("tx" in err) {
          const logs = await provider.connection.getConfirmedTransaction(err.tx, "confirmed");
          console.error("Transaction Logs:", logs.meta?.logMessages || []);
        } else {
          console.error("Error Logs Unavailable. Raw Error:", err);
        }
        throw err;
      }
  
      const cabinetAccount = await program.account.veterinaryCabinet.fetch(cabinetPDA);
      assert.strictEqual(
        cabinetAccount.wallet.toBase58(),
        payer.toBase58(),
        "Cabinet wallet mismatch"
      );
      assert.strictEqual(
        Buffer.from(cabinetAccount.info).toString().trim(),
        "Paw Perfect Veterinary Clinic",
        "Cabinet info mismatch"
      );
  
      const updatedTreasuryBalance = await provider.connection.getBalance(treasuryPda);
      assert.strictEqual(
        updatedTreasuryBalance,
        initialTreasuryBalance + joinFeeLamports,
        "Treasury lamports balance should match the updated balance after the JOIN_FEE"
      );
  
      console.log("Cabinet successfully added with PDA:", cabinetPDA.toBase58());
    });
  });

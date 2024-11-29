import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VeterinarySystem } from "../target/types/veterinary_system";

describe("solana-ark-foundation (Devnet)", () => {
  // Configure the client to use Devnet
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VeterinarySystem as Program<VeterinarySystem>;

  it("Checks if the program is initialized on Devnet", async () => {
    // Derive the admin PDA
    const [adminPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("admin")], // ADMIN_SEED
      program.programId
    );

    // Try to fetch the admin account
    try {
      const adminAccount = await program.account.adminAccount.fetch(adminPda);
      console.log("Program is already initialized with admin PDA:", adminPda.toBase58());
      console.log("Admin Account Data:", adminAccount);
    } catch (error) {
      console.log("Program is not initialized. Proceeding with initialization...");

      // Initialize the program
      const tx = await program.methods
        .initializeAdmin()
        .accounts({
          payer: provider.wallet.publicKey,
        })
        .rpc();
      console.log("Initialization transaction signature:", tx);
    }
  });
});

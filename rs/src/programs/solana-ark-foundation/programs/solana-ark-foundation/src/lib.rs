use anchor_lang::prelude::*;

declare_id!("9sZpn4qSY2Xh5MEUbkFWxcJJaMwVePunGpxTNEtHkrtk");

#[program]
pub mod solana_ark_foundation {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

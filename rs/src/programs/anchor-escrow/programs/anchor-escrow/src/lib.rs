use anchor_lang::prelude::*;

declare_id!("4W2xEwJ7K4Detthv2Qc6fZRV9rjM7RxfnKXVR5okaP9S");

pub mod state;
pub use sate::*;

pub mod instruction;
pub use instruction::*;

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Make>, seed:u64, receive:u64 ) -> Result<()> {
        ctx.accoutns.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}

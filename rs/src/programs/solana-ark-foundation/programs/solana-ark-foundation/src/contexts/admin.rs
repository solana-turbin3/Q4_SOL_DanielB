use anchor_lang::prelude::*;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init,
        seeds = [b"admin"],
        bump,
        payer = payer,
        space = 8 + 32 // Adjust for the AdminAccount size (8 for discriminator, 32 for key)
    )]
    pub admin_pda: Account<'info, AdminAccount>, // Admin PDA Account

    #[account(mut)]
    pub payer: Signer<'info>, // Payer initializes the account

    pub system_program: Program<'info, System>, // System program for account initialization
}
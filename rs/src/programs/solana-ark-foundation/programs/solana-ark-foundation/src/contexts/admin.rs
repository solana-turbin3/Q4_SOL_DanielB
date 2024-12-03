use anchor_lang::prelude::*;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 64, // Adjust for AdminAccount size
        seeds = [b"admin"],
        bump
    )]
    pub admin_pda: Account<'info, AdminAccount>,

    #[account(mut)]
    pub payer: Signer<'info>, // Wallet paying for the transaction

    pub system_program: Program<'info, System>,
}
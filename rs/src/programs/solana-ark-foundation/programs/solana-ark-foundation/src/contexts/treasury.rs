use anchor_lang::prelude::*;
use crate::entities::TreasuryAccount;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(
        init,
        payer = payer, // Payer funds the initialization
        seeds = [b"treasury"], // Seed for Treasury PDA based on admin key
        bump,
        space = 8 + 256 // Account discriminator + TreasuryAccount fields
    )]
    pub treasury: Account<'info, TreasuryAccount>,

    #[account(
        seeds = [b"admin"], // Seed for Admin PDA
        bump,
        has_one = admin // Ensure the admin matches the Admin PDA
    )]
    pub admin_pda: Account<'info, AdminAccount>,

    #[account(mut)]
    pub admin: Signer<'info>, // Admin wallet signs for initialization

    #[account(mut)]
    pub payer: Signer<'info>, // Payer funds the account creation

    pub system_program: Program<'info, System>, // System program for lamport transfers
}


#[derive(Accounts)]
pub struct WithdrawFromTreasury<'info> {
    #[account(
        mut,
        seeds = [b"treasury"], // Seed for Treasury PDA
        bump,
        has_one = admin // Ensure only the admin can withdraw
    )]
    pub treasury: Account<'info, TreasuryAccount>,

    #[account(
        seeds = [b"admin"], // Seed for Admin PDA
        bump
    )]
    pub admin_pda: Account<'info, AdminAccount>,

    #[account(mut)]
    pub admin: Signer<'info>, // Admin wallet signs the transaction

    /// CHECK: Destination account for funds (not verified, can be any wallet)
    #[account(mut)]
    pub destination: AccountInfo<'info>,

    pub system_program: Program<'info, System>, // System program for lamport transfers
}

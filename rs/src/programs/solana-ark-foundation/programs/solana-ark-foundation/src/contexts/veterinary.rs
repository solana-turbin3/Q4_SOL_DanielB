use anchor_lang::prelude::*;
use crate::entities::VeterinaryCabinet;
use crate::entities::TreasuryAccount;
use crate::entities::AdminAccount;

#[derive(Accounts)]
pub struct AddVeterinaryCabinet<'info> {
    #[account(
        init,
        payer = payer, // Payer funds the creation of the cabinet
        seeds = [b"cabinet"], // Seed for Veterinary Cabinet PDA
        bump,
        space = 8 + 256 // Account discriminator (8 bytes) + VeterinaryCabinet fields
    )]
    pub cabinet: Account<'info, VeterinaryCabinet>, // Veterinary Cabinet PDA

    #[account(mut)]
    pub payer: Signer<'info>, // User paying the JOIN_FEE

    #[account(
        mut,
        seeds = [b"treasury", admin.key().as_ref()], // Seed for Treasury PDA
        bump
    )]
    pub treasury: Account<'info, TreasuryAccount>, // Treasury PDA to receive the JOIN_FEE

    #[account(
        seeds = [b"admin"], // Seed for Admin PDA
        bump
    )]
    pub admin_pda: Account<'info, AdminAccount>, // Admin PDA for verification

    #[account(mut)]
    pub admin: Signer<'info>, // Admin wallet signs the transaction

    pub system_program: Program<'info, System>, // System program for transferring lamports
}

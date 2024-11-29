use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::entities::VeterinaryCabinet;

#[derive(Accounts)]
pub struct MintVeterinaryCabinetNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, // Separate payer for transaction fees

    // Verify the caller is a cabinet member by linking to the wallet field in VeterinaryCabinet
    #[account(mut, has_one = wallet)]
    pub cabinet: Account<'info, VeterinaryCabinet>,

    /// Wallet signer for validation
    pub wallet: Signer<'info>, // Add this line

    /// CHECK: This is not initialized; used as a PDA for master edition
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,

    /// CHECK: Admin PDA for signing and paying transactions
    #[account(seeds = [b"admin_pda"], bump)]
    pub admin_pda: UncheckedAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>, // Mint account for the NFT

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>, // Associated Token Account for mint

    /// CHECK: Metadata account (to be created)
    #[account(mut)]
    pub metadata_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,          // SPL Token program
    pub system_program: Program<'info, System>,        // System program for creating accounts
}

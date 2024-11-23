use anchor_lang::prelude::*;
// use anchor_spl::token::{Mint, TokenAccount, Token};
use anchor_spl::token::Token;
use crate::entities::VeterinaryCabinet;

#[derive(Accounts)]
pub struct MintVeterinaryCabinetNFT<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>, // Veterinary Cabinet PDA
    #[account(mut)]
    pub payer: Signer<'info>, // Wallet initiating the mint
    #[account(init, payer = payer, space = 82, seeds = [b"mint", cabinet.key().as_ref()], bump)]
    pub mint: AccountInfo<'info>, // NFT mint account
    #[account(init, payer = payer, space = 165, seeds = [b"token", cabinet.key().as_ref()], bump)]
    pub token_account: AccountInfo<'info>, // Token account for NFT
    #[account(init, payer = payer, space = 200, seeds = [b"metadata", mint.key().as_ref()], bump)]
    pub metadata: AccountInfo<'info>, // Metadata account for NFT
    pub token_program: Program<'info, Token>, // Token program
    pub system_program: Program<'info, System>, // System program
    pub rent: Sysvar<'info, Rent>, // Rent system
}

use anchor_lang::prelude::*;
use crate::entities::VeterinaryCabinet;

#[derive(Accounts)]
pub struct AddVeterinaryCabinet<'info> {
    #[account(init, payer = payer, space = 8 + 320)] // Adjust space for VeterinaryCabinet struct
    pub cabinet: Account<'info, VeterinaryCabinet>,
    #[account(mut)]
    pub payer: Signer<'info>, // User paying the fee
    /// CHECK: Safe because this account is explicitly program-controlled
    #[account(mut)]
    pub fee_account: AccountInfo<'info>, // Program's fee account
    pub system_program: Program<'info, System>,
}

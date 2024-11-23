use anchor_lang::prelude::*;
use crate::entities::{ Owner, VeterinaryCabinet};

#[derive(Accounts)]
pub struct AddOwner<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>,
    #[account(init, payer = payer, space = 8 + 108)] // Adjust space for maximum size
    pub owner: Account<'info, Owner>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

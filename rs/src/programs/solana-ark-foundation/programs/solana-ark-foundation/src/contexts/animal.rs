use anchor_lang::prelude::*;
use crate::entities::{Animal, Owner, VeterinaryCabinet};

#[derive(Accounts)]
pub struct AddAnimal<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>,
    #[account(mut)]
    pub owner: Account<'info, Owner>,
    #[account(init, payer = payer, space = 8 + 328)]
    pub animal: Account<'info, Animal>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(mut)]
    pub animal: Account<'info, Animal>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

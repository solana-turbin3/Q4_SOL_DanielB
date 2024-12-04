use anchor_lang::prelude::*;
use crate::entities::{Animal, Owner, VeterinaryCabinet};

#[derive(Accounts)]
pub struct AddAnimal<'info> {
    #[account(mut)]
    pub cabinet: Account<'info, VeterinaryCabinet>,

    #[account(mut)]
    pub owner: Account<'info, Owner>,

    #[account(
        init_if_needed,
        payer = payer,
        //should include additional context (e.g., the owner ID or cabinet ID) for uniqueness
        seeds = [b"animal".as_ref(), owner.key().as_ref(), cabinet.key().as_ref()],  // Seed for VeterinaryCabinet PDA
        bump,
        space = 8 + 256 // Account discriminator + VeterinaryCabinet fields
    )]
    pub animal: Account<'info, Animal>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct TransferOwnership<'info> {
//     #[account(mut)]
//     pub animal: Account<'info, Animal>,
//     #[account(mut)]
//     pub payer: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

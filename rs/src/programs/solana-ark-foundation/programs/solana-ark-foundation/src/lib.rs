use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("9sZpn4qSY2Xh5MEUbkFWxcJJaMwVePunGpxTNEtHkrtk");

pub mod errors;
pub mod entities;
pub mod contexts; // Include the contexts module
pub mod instructions;

// use crate::errors::ErrorCode;
// use crate::entities::{VeterinaryCabinet, Owner, Animal}; // Import structs if needed
use crate::contexts::{AddVeterinaryCabinet, AddAnimal, AddOwner}; // Ensure these are correctly imported.
// use crate::instructions::{add_veterinary_cabinet, add_owner, add_animal, transfer_animal_ownership}; // Ensure these are correctly imported.

const JOIN_FEE: u64 = 800_000; // 0.8 Lamports

#[program]
pub mod veterinary_system {
    use super::*;

    /// Add a new Veterinary Cabinet to the system
    pub fn add_veterinary_cabinet(
        ctx: Context<AddVeterinaryCabinet>,
        name: String,
        id: String,
        phone: String,
        country: String,
        town: String,
        postcode: String,
        address: String,
        expire_date: i64,
    ) -> Result<()> {
        // Delegate logic to instructions module
        instructions::add_veterinary_cabinet(ctx, name, id, phone, country, town, postcode, address, expire_date)
    }

    /// Add a new Animal Owner
    pub fn add_animal_owner(
        ctx: Context<AddOwner>,
        owner_name: String,
        animal_id: String,
    ) -> Result<()> {
        // Delegate logic to instructions module
        instructions::add_owner(ctx, owner_name, animal_id)
    }

    /// Add a new Animal
    pub fn add_animal(
        ctx: Context<AddAnimal>,
        metadata: String,
        owner_id: String,
    ) -> Result<()> {
        // Delegate logic to instructions module
        instructions::add_animal(ctx, metadata, owner_id)
    }
}
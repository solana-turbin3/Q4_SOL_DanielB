use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

declare_id!("CeXpwgqj6qKpT5fBvpgUU9z6Xryt5nEyMqWvhnm3HkRK");

mod errors;
mod entities;
mod contexts; // Include the contexts module
mod instructions;
mod utils;

// use crate::errors::ErrorCode;
// use crate::entities::{VeterinaryCabinet, Owner, Animal}; // Import structs if needed
use crate::contexts::*; // Ensure these are correctly imported.
use crate::utils::*;
// use crate::instructions::{add_veterinary_cabinet, add_owner, add_animal, transfer_animal_ownership}; // Ensure these are correctly imported.

#[program]
pub mod veterinary_system {
    use super::*;
    use crate::constants::ADMIN_SEED;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        let (admin_pda, _bump) = Pubkey::find_program_address(&[ADMIN_SEED], ctx.program_id);
    
        // Assign the derived `admin_pda` to the account.
        instructions::initialize_admin(ctx, admin_pda)
    }
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

    /// Transfer ownership of an Animal
    pub fn transfer_animal_ownership(
        ctx: Context<TransferOwnership>,
        new_owner_id: String,
        veterinary_cabinet_id: String,
    ) -> Result<()> {
        // Delegate logic to instructions module
        // mihg
        instructions::transfer_animal_ownership(ctx, new_owner_id,  veterinary_cabinet_id)
    }

    pub fn mint_veterinary_cabinet_nft(
        ctx: Context<MintVeterinaryCabinetNFT>,
        metadata_uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        instructions::mint_veterinary_cabinet_nft(ctx, metadata_uri, name, symbol)
    }
}

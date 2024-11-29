/// Add a new Animal Owner
use anchor_lang::prelude::*;
use crate::contexts::owner::AddOwner;
use crate::errors::ErrorCode;

pub fn add_owner(
    ctx: Context<AddOwner>,
    owner_name: String,
    veterinary_cabinet_id: String,
) -> Result<()> {
    let owner = &mut ctx.accounts.owner;

    // Ensure the caller is a recognized veterinary cabinet
    if ctx.accounts.cabinet.wallet != ctx.accounts.payer.key() {
        return err!(ErrorCode::UnauthorizedAccess);
    }

    // Initialize the owner PDA
    owner.name = owner_name;
    owner.veterinary_cabinet_id = veterinary_cabinet_id;
    owner.wallet = ctx.accounts.payer.key();

    Ok(())
}
use anchor_lang::prelude::*;
use crate::contexts::animal::{AddAnimal,TransferOwnership};
use crate::errors::ErrorCode;

 /// Add a new Animal
 pub fn add_animal(
    ctx: Context<AddAnimal>,
    metadata: String,
    owner_id: String,
) -> Result<()> {
//     let animal = &mut ctx.accounts.animal;

//     // Ensure the caller is a recognized animal owner
//     if ctx.accounts.owner.wallet != ctx.accounts.payer.key() {
//         return err!(ErrorCode::UnauthorizedAccess);
//     }

//     // Initialize the animal PDA
//     animal.metadata = metadata;
//     animal.owner_id = owner_id;
//     animal.cabinet_id = ctx.accounts.cabinet.id.clone();

    Ok(())
}

pub fn transfer_animal_ownership(
    ctx: Context<TransferOwnership>,
    new_owner_id: String,
    veterinary_cabinet_id: String,
) -> Result<()> {
    // let animal = &mut ctx.accounts.animal;

    // // Ensure the caller is a recognized veterinary cabinet
    // let cabinet_id = veterinary_cabinet_id.clone(); // Fix shadowing and cloning
    // if cabinet_id != animal.cabinet_id {
    //     return err!(ErrorCode::UnauthorizedAccess);
    // }

    // // Update the owner ID for the animal PDA
    // animal.owner_id = new_owner_id;

    Ok(())
}

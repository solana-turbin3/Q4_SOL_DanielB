use anchor_lang::prelude::*;
use crate::contexts::veterinary::AddVeterinaryCabinet;
use crate::errors::ErrorCode;

pub const JOIN_FEE: u64 = 800_000;

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
    let cabinet = &mut ctx.accounts.cabinet;
    let payer = &ctx.accounts.payer;
    let fee_account = &mut ctx.accounts.fee_account;

    // Ensure the user has paid the required fee
    if **payer.lamports.borrow() < JOIN_FEE {
        return err!(ErrorCode::InsufficientFee);
    }

    // Transfer lamports to the fee account
    **payer.lamports.borrow_mut() -= JOIN_FEE;
    **fee_account.lamports.borrow_mut() += JOIN_FEE;

    // Initialize the cabinet PDA
    cabinet.name = name;
    cabinet.id = id;
    cabinet.phone = phone;
    cabinet.country = country;
    cabinet.town = town;
    cabinet.postcode = postcode;
    cabinet.address = address;
    cabinet.wallet = payer.key();
    cabinet.expire_date = expire_date;

    // Mint NFT for the cabinet (NFT minting logic placeholder)
    // ...

    Ok(())
}    
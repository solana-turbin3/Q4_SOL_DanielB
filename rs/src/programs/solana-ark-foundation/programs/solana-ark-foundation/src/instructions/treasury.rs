use anchor_lang::prelude::*;
use crate::contexts::treasury::{InitializeTreasury, WithdrawFromTreasury};

pub fn initialize_treasury(ctx: Context<InitializeTreasury>) -> Result<()> {
    // Derive the expected treasury PDA using admin's public key
    let (expected_treasury_pda, _bump) =
        Pubkey::find_program_address(&[b"treasury"], ctx.program_id);

    // Ensure the derived PDA matches the provided treasury account
    if ctx.accounts.treasury.key() != expected_treasury_pda {
        return Err(error!(crate::errors::ErrorCode::InvalidTreasuryAccount));
    }

    // Ensure this function is not called again if the Treasury PDA already exists
    if ctx.accounts.treasury.balance != 0 {
        return Err(error!(crate::errors::ErrorCode::TreasuryAlreadyInitialized));
    }

    // Initialize the treasury account
    let treasury = &mut ctx.accounts.treasury;
    treasury.balance = 0; // Start with zero balance
    treasury.admin = ctx.accounts.admin_pda.key(); // Link Treasury to Admin PDA

    Ok(())
}


pub fn withdraw_from_treasury(
    ctx: Context<WithdrawFromTreasury>,
    amount: u64,
) -> Result<()> {
    // Ensure the caller is the admin
    if ctx.accounts.admin_pda.admin != ctx.accounts.admin.key() {
        return Err(error!(crate::errors::ErrorCode::UnauthorizedAccess));
    }

    // Derive the expected treasury PDA
    let (expected_treasury_pda, _bump) =
        Pubkey::find_program_address(&[b"treasury"], ctx.program_id);

    // Ensure the derived PDA matches the treasury account
    if ctx.accounts.treasury.key() != expected_treasury_pda {
        return Err(error!(crate::errors::ErrorCode::InvalidTreasuryAccount));
    }

    let treasury = &mut ctx.accounts.treasury;
    let destination = &mut ctx.accounts.destination;

    // Ensure sufficient funds in the treasury
    if treasury.balance < amount {
        return Err(error!(crate::errors::ErrorCode::InsufficientFunds));
    }

    // Perform the lamport transfer
    **treasury.to_account_info().lamports.borrow_mut() -= amount;
    **destination.lamports.borrow_mut() += amount;

    // Update the treasury's custom balance
    treasury.balance -= amount;

    Ok(())
}



use anchor_lang::prelude::*;

#[account]
pub struct AdminAccount {
    pub key: Pubkey, // Store the admin's public key
}
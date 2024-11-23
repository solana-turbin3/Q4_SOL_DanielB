use anchor_lang::prelude::*;

#[account]
pub struct Owner {
    pub name: String,       // 4 bytes + name length
    pub animal_id: String,  // 4 bytes + animal ID length
    pub wallet: Pubkey,     // 32 bytes
}

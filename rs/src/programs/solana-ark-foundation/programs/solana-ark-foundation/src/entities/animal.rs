use anchor_lang::prelude::*;

#[account]
pub struct Animal {
    pub metadata: String,  // 4 bytes + metadata length
    pub owner_id: String,  // 4 bytes + owner ID length
    pub cabinet_id: String, // 4 bytes + cabinet ID length
}
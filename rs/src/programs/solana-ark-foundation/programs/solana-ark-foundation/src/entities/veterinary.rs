use anchor_lang::prelude::*;

#[account]
pub struct VeterinaryCabinet {
    pub name: String,         // 4 bytes + name length
    pub id: String,           // 4 bytes + id length
    pub phone: String,        // 4 bytes + phone length
    pub country: String,      // 4 bytes + country length
    pub town: String,         // 4 bytes + town length
    pub postcode: String,     // 4 bytes + postcode length
    pub address: String,      // 4 bytes + address length
    pub wallet: Pubkey,       // 32 bytes
    pub expire_date: i64,     // 8 bytes
}

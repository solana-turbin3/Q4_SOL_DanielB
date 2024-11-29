pub mod mint_nft;
pub mod animal;
pub mod owner;
pub mod veterinary;
pub mod admin;

// Re-export context structs for program use
pub use animal::*;
pub use owner::*;
pub use veterinary::*;
pub use admin::*;
pub use mint_nft::*;

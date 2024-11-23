pub mod mint_nft;
pub mod animal;
pub mod owner;
pub mod veterinary;

// Re-export context structs for program use
pub use animal::{AddAnimal, TransferOwnership};
pub use owner::AddOwner;
pub use veterinary::AddVeterinaryCabinet;
pub use mint_nft::MintVeterinaryCabinetNFT;

pub mod veterinary;
pub mod animal;
pub mod owner;

// Re-export functions
pub use veterinary::add_veterinary_cabinet;
pub use animal::{add_animal, transfer_animal_ownership};
pub use owner::add_owner;

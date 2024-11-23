use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized: Caller wallet does not match the cabinet owner.")]
    UnauthorizedAccess,
    #[msg("Invalid metadata: Metadata URI is required.")]
    InvalidMetadata,
    #[msg("Insufficient fee for joining the program.")]
    InsufficientFee,
}

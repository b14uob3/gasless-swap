use crate::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Signer has no authority.")]
    Unauthorized,
}

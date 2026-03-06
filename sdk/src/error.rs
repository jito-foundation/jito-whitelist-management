use solana_program_error::ProgramError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WhitelistManagementError {
    #[error("ArithmeticOverflow")]
    ArithmeticOverflow = 1000,

    #[error("InvalidAdmin")]
    InvalidAdmin = 1001,

    #[error("AdminSelfRemoval")]
    AdminSelfRemoval = 1002,

    #[error("ListFull")]
    ListFull = 1003,
}

impl From<WhitelistManagementError> for ProgramError {
    fn from(e: WhitelistManagementError) -> Self {
        Self::Custom(e as u32)
    }
}

impl From<WhitelistManagementError> for u64 {
    fn from(e: WhitelistManagementError) -> Self {
        e as Self
    }
}

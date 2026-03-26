use borsh::{BorshDeserialize, BorshSerialize};
use shank::ShankInstruction;

#[derive(Debug, BorshSerialize, BorshDeserialize, ShankInstruction)]
pub enum JitoWhitelistManagementInstruction {
    /// Initializes the global configuration
    #[account(0, writable, signer, name = "payer")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "initial_admin")]
    #[account(3, name = "system_program")]
    InitializeWhitelist,

    /// Add admin
    #[account(0, signer, name = "admin")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "new_admin")]
    AddAdmin,

    /// Remove Admin
    #[account(0, signer, name = "admin")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "admin_to_remove")]
    RemoveAdmin,

    // Add To Whitelist
    #[account(0, signer, name = "admin")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "signer_to_add")]
    AddToWhitelist,

    // Remove From Whitelist
    #[account(0, signer, name = "admin")]
    #[account(1, writable, name = "whitelist")]
    #[account(2, name = "signer_to_remove")]
    RemoveFromWhitelist,
}

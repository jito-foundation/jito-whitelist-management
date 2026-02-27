use clap::{Subcommand, ValueEnum};
use solana_pubkey::Pubkey;

/// Network type for subsidy schedule
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum NetworkArg {
    Mainnet,
    Testnet,
}

/// The CLI handler for the bam_boost program
#[derive(Subcommand)]
pub enum WhitelistManagementCommands {
    /// Initialize, get, and set the whitelist management struct
    Whitelist {
        #[command(subcommand)]
        action: WhitelistManagementActions,
    },
}

/// The actions that can be performed on the bam_boost config
#[derive(Subcommand)]
pub enum WhitelistManagementActions {
    /// Get the whitelist
    Get {
        /// Base
        #[arg(long)]
        base: Pubkey,
    },

    /// Initialize the whitelist
    Initialize {
        /// Base keypair used to derive the whitelist PDA
        #[arg(long)]
        base: String,

        /// Initial Admin
        #[arg(long)]
        initial_admin: Pubkey,
    },

    /// Add Admin
    AddAdmin {
        /// Base keypair used to derive the whitelist PDA
        #[arg(long)]
        base: Pubkey,

        /// New Admin
        #[arg(long)]
        new_admin: Pubkey,
    },

    /// Remove Admin
    RemoveAdmin {
        /// Base keypair used to derive the whitelist PDA
        #[arg(long)]
        base: Pubkey,

        /// Admin To Remove
        #[arg(long)]
        admin_to_remove: Pubkey,
    },

    /// Add To Whitelist
    AddToWhitelist {
        /// Base keypair used to derive the whitelist PDA
        #[arg(long)]
        base: Pubkey,

        /// Signer To Add
        #[arg(long)]
        signer_to_add: Pubkey,
    },

    /// Remove From Whitelist
    RemoveFromWhitelist {
        /// Base keypair used to derive the whitelist PDA
        #[arg(long)]
        base: Pubkey,

        /// Signer To Remove
        #[arg(long)]
        signer_to_remove: Pubkey,
    },
}

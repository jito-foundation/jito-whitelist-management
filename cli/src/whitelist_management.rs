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
    Get,

    /// Initialize the whitelist
    Initialize {
        /// Initial Admin
        #[arg(long)]
        initial_admin: Pubkey,
    },

    /// Add Admin
    AddAdmin {
        /// Current admin pubkey. If omitted, defaults to the configured signer.
        /// Pass explicitly when using --print-tx without a keypair file.
        #[arg(long)]
        admin: Option<Pubkey>,

        /// New Admin
        #[arg(long)]
        new_admin: Pubkey,
    },

    /// Remove Admin
    RemoveAdmin {
        /// Current admin pubkey. If omitted, defaults to the configured signer.
        /// Pass explicitly when using --print-tx without a keypair file.
        #[arg(long)]
        admin: Option<Pubkey>,

        /// Admin To Remove
        #[arg(long)]
        admin_to_remove: Pubkey,
    },

    /// Add To Whitelist
    AddToWhitelist {
        /// Current admin pubkey. If omitted, defaults to the configured signer.
        /// Pass explicitly when using --print-tx without a keypair file.
        #[arg(long)]
        admin: Option<Pubkey>,

        /// Signer To Add
        #[arg(long)]
        signer_to_add: Pubkey,
    },

    /// Remove From Whitelist
    RemoveFromWhitelist {
        /// Current admin pubkey. If omitted, defaults to the configured signer.
        /// Pass explicitly when using --print-tx without a keypair file.
        #[arg(long)]
        admin: Option<Pubkey>,

        /// Signer To Remove
        #[arg(long)]
        signer_to_remove: Pubkey,
    },
}

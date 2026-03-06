use jito_bytemuck::AccountDeserialize;
use jito_whitelist_management_client::instructions::{
    AddAdminBuilder, AddToWhitelistBuilder, InitializeWhitelistBuilder, RemoveAdminBuilder,
    RemoveFromWhitelistBuilder,
};
use jito_whitelist_management_core::whitelist::Whitelist;
use solana_pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_signer::{signers::Signers, Signer};
use solana_transaction::{Instruction, Transaction};

use crate::{
    cli_config::CliConfig,
    whitelist_management::{WhitelistManagementActions, WhitelistManagementCommands},
};

#[allow(dead_code)]
pub struct WhitelistManagementCliHandler {
    /// The configuration of CLI
    cli_config: CliConfig,

    /// The Pubkey of the Jito Whitelist Management Program
    jito_whitelist_management_program_id: Pubkey,

    /// This will print out the raw TX instead of running it
    print_tx: bool,

    /// This will print out the account information in JSON format
    print_json: bool,

    /// This will print out the account information in JSON format with reserved space
    print_json_with_reserves: bool,
}

impl WhitelistManagementCliHandler {
    pub const fn new(
        cli_config: CliConfig,
        jito_whitelist_management_program_id: Pubkey,
        print_tx: bool,
        print_json: bool,
        print_json_with_reserves: bool,
    ) -> Self {
        Self {
            cli_config,
            jito_whitelist_management_program_id,
            print_tx,
            print_json,
            print_json_with_reserves,
        }
    }

    pub async fn handle(&self, action: WhitelistManagementCommands) -> anyhow::Result<()> {
        match action {
            WhitelistManagementCommands::Whitelist {
                action: WhitelistManagementActions::Get,
            } => self.get_whitelist().await,
            WhitelistManagementCommands::Whitelist {
                action: WhitelistManagementActions::Initialize { initial_admin },
            } => self.initialize_whitelist(initial_admin).await,
            WhitelistManagementCommands::Whitelist {
                action: WhitelistManagementActions::AddAdmin { new_admin },
            } => self.add_admin(new_admin).await,
            WhitelistManagementCommands::Whitelist {
                action: WhitelistManagementActions::RemoveAdmin { admin_to_remove },
            } => self.remove_admin(admin_to_remove).await,
            WhitelistManagementCommands::Whitelist {
                action: WhitelistManagementActions::AddToWhitelist { signer_to_add },
            } => self.add_to_whitelist(signer_to_add).await,
            WhitelistManagementCommands::Whitelist {
                action: WhitelistManagementActions::RemoveFromWhitelist { signer_to_remove },
            } => self.remove_from_whitelist(signer_to_remove).await,
        }
    }

    fn whitelist_address(&self) -> Pubkey {
        let program_id = self.jito_whitelist_management_program_id;
        Pubkey::find_program_address(&[b"whitelist"], &program_id).0
    }

    async fn get_whitelist(&self) -> anyhow::Result<()> {
        let jito_whitelist_management_address = self.whitelist_address();
        let account = self
            .get_account::<Whitelist>(&Pubkey::new_from_array(
                jito_whitelist_management_address.to_bytes(),
            ))
            .await?;

        println!("Whitelist Account: {}", jito_whitelist_management_address);
        println!("Bump: {}", account.bump);

        println!("\nAdmins:");
        for (i, admin) in account.admins.iter().enumerate() {
            if !Whitelist::is_empty_address(admin) {
                println!("  [{}] {}", i, admin);
            }
        }

        println!("\nWhitelisted Signers:");
        for (i, signer) in account.whitelist.iter().enumerate() {
            if !Whitelist::is_empty_address(signer) {
                println!("  [{}] {}", i, signer);
            }
        }

        Ok(())
    }

    async fn initialize_whitelist(&self, initial_admin: Pubkey) -> anyhow::Result<()> {
        let whitelist_address = self.whitelist_address();
        // let signer = self
        //     .cli_config
        //     .signer
        //     .clone()
        //     .ok_or_else(|| anyhow::anyhow!("signer is required"))?;

        let mut ix_builder = InitializeWhitelistBuilder::new();
        ix_builder
            .payer(self.cli_config.signer.pubkey())
            .whitelist(whitelist_address)
            .initial_admin(initial_admin);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.jito_whitelist_management_program_id;

        log::info!("Initializing Whitelist parameters: {ix_builder:?}",);

        self.process_transaction(
            &[ix],
            &self.cli_config.signer.pubkey(),
            std::slice::from_ref(&self.cli_config.signer),
        )
        .await?;

        if !self.print_tx {
            let account = self.get_account::<Whitelist>(&whitelist_address).await?;
            log::info!("{account:?}");
        }

        Ok(())
    }

    async fn add_admin(&self, new_admin: Pubkey) -> anyhow::Result<()> {
        let whitelist_address = self.whitelist_address();

        let mut ix_builder = AddAdminBuilder::new();
        ix_builder
            .admin(self.cli_config.signer.pubkey())
            .whitelist(whitelist_address)
            .new_admin(new_admin);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.jito_whitelist_management_program_id;

        log::info!("Adding new admin parameters: {ix_builder:?}",);

        self.process_transaction(
            &[ix],
            &self.cli_config.signer.pubkey(),
            std::slice::from_ref(&self.cli_config.signer),
        )
        .await?;

        if !self.print_tx {
            let account = self.get_account::<Whitelist>(&whitelist_address).await?;
            log::info!("{account:?}");
        }

        Ok(())
    }

    async fn remove_admin(&self, admin_to_remove: Pubkey) -> anyhow::Result<()> {
        let whitelist_address = self.whitelist_address();

        let mut ix_builder = RemoveAdminBuilder::new();
        ix_builder
            .admin(self.cli_config.signer.pubkey())
            .whitelist(whitelist_address)
            .admin_to_remove(admin_to_remove);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.jito_whitelist_management_program_id;

        log::info!("Removing admin parameters: {ix_builder:?}",);

        self.process_transaction(
            &[ix],
            &self.cli_config.signer.pubkey(),
            std::slice::from_ref(&self.cli_config.signer),
        )
        .await?;

        if !self.print_tx {
            let account = self.get_account::<Whitelist>(&whitelist_address).await?;
            log::info!("{account:?}");
        }

        Ok(())
    }

    async fn add_to_whitelist(&self, signer_to_add: Pubkey) -> anyhow::Result<()> {
        let whitelist_address = self.whitelist_address();

        let mut ix_builder = AddToWhitelistBuilder::new();
        ix_builder
            .admin(self.cli_config.signer.pubkey())
            .whitelist(whitelist_address)
            .signer_to_add(signer_to_add);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.jito_whitelist_management_program_id;

        log::info!("Adding to whitelist parameters: {ix_builder:?}",);

        self.process_transaction(
            &[ix],
            &self.cli_config.signer.pubkey(),
            std::slice::from_ref(&self.cli_config.signer),
        )
        .await?;

        if !self.print_tx {
            let account = self.get_account::<Whitelist>(&whitelist_address).await?;
            log::info!("{account:?}");
        }

        Ok(())
    }

    async fn remove_from_whitelist(&self, signer_to_remove: Pubkey) -> anyhow::Result<()> {
        let whitelist_address = self.whitelist_address();

        let mut ix_builder = RemoveFromWhitelistBuilder::new();
        ix_builder
            .admin(self.cli_config.signer.pubkey())
            .whitelist(whitelist_address)
            .signer_to_remove(signer_to_remove);
        let mut ix = ix_builder.instruction();
        ix.program_id = self.jito_whitelist_management_program_id;

        log::info!("Removing from whitelist parameters: {ix_builder:?}",);

        self.process_transaction(
            &[ix],
            &self.cli_config.signer.pubkey(),
            std::slice::from_ref(&self.cli_config.signer),
        )
        .await?;

        if !self.print_tx {
            let account = self.get_account::<Whitelist>(&whitelist_address).await?;
            log::info!("{account:?}");
        }

        Ok(())
    }

    /// Creates a new RPC client using the configuration from the CLI handler.
    ///
    /// This method constructs an RPC client with the URL and commitment level specified in the
    /// CLI configuration. The client can be used to communicate with a Solana node for
    /// submitting transactions, querying account data, and other RPC operations.
    fn get_rpc_client(&self) -> RpcClient {
        RpcClient::new_with_commitment(self.cli_config.rpc_url.clone(), self.cli_config.commitment)
    }

    /// Fetches and deserializes an account
    ///
    /// This method retrieves account data using the configured RPC client,
    /// then deserializes it into the specified account type using Borsh deserialization.
    async fn get_account<T: AccountDeserialize>(
        &self,
        account_pubkey: &Pubkey,
    ) -> anyhow::Result<T> {
        let rpc_client = self.get_rpc_client();

        let account = rpc_client.get_account(account_pubkey).await?;
        let account = T::try_from_slice_unchecked(&account.data)?;

        Ok(*account)
    }

    // fn convert_instruction(
    //     &self,
    //     ix: Instruction,
    // ) -> anchor_lang::prelude::instruction::Instruction {
    //     anchor_lang::prelude::instruction::Instruction {
    //         program_id: anchor_lang::prelude::Pubkey::new_from_array(ix.program_id.to_bytes()),
    //         accounts: ix
    //             .accounts
    //             .into_iter()
    //             .map(|acc| anchor_lang::prelude::AccountMeta {
    //                 pubkey: anchor_lang::prelude::Pubkey::new_from_array(acc.pubkey.to_bytes()),
    //                 is_signer: acc.is_signer,
    //                 is_writable: acc.is_writable,
    //             })
    //             .collect(),
    //         data: ix.data,
    //     }
    // }

    // fn convert_anchor_instruction(
    //     &self,
    //     ix: anchor_lang::prelude::instruction::Instruction,
    // ) -> Instruction {
    //     Instruction {
    //         program_id: Pubkey::new_from_array(ix.program_id.to_bytes()),
    //         accounts: ix
    //             .accounts
    //             .into_iter()
    //             .map(|acc| AccountMeta {
    //                 pubkey: Pubkey::new_from_array(acc.pubkey.to_bytes()),
    //                 is_signer: acc.is_signer,
    //                 is_writable: acc.is_writable,
    //             })
    //             .collect(),
    //         data: ix.data,
    //     }
    // }

    /// Processes a transaction by either printing it as Base58 or sending it.
    ///
    /// This method handles the logic for processing a set of instructions as a transaction.
    /// If `print_tx` is enabled in the CLI handler (helpful for running commands in Squads), it will print the transaction in Base58 format
    /// without sending it. Otherwise, it will submit and confirm the transaction.
    async fn process_transaction<T>(
        &self,
        ixs: &[Instruction],
        payer: &Pubkey,
        signers: &T,
    ) -> anyhow::Result<()>
    where
        T: Signers + ?Sized,
    {
        let rpc_client = self.get_rpc_client();

        let blockhash = rpc_client.get_latest_blockhash().await?;
        let tx = Transaction::new_signed_with_payer(ixs, Some(payer), signers, blockhash);
        let result = rpc_client.send_and_confirm_transaction(&tx).await?;

        log::info!("Transaction confirmed: {:?}", result);

        Ok(())
    }

    // #[allow(dead_code)]
    // fn print_base58_tx(&self, ixs: &[Instruction]) {
    //     ixs.iter().for_each(|ix| {
    //         log::info!("\n------ IX ------\n");

    //         println!("{}\n", ix.program_id);

    //         ix.accounts.iter().for_each(|account| {
    //             let pubkey = format!("{}", account.pubkey);
    //             let writable = if account.is_writable { "W" } else { "" };
    //             let signer = if account.is_signer { "S" } else { "" };

    //             println!("{:<44} {:>2} {:>1}", pubkey, writable, signer);
    //         });

    //         println!("\n");

    //         let base58_string = bs58::encode(&ix.data).into_string();
    //         println!("{}\n", base58_string);
    //     });
    // }
}

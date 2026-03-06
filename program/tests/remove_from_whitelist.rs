mod fixtures;

#[cfg(test)]
mod tests {
    use jito_whitelist_management_client::errors::JitoWhitelistManagementError;
    use jito_whitelist_management_core::whitelist::EMPTY_ADDRESS;
    use solana_keypair::{Keypair, Signer};
    use solana_transaction::InstructionError;

    use crate::fixtures::{assert_ix_error, TestBuilder};

    #[tokio::test]
    async fn test_remove_from_whitelist_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await
            .unwrap();

        let new_signer = Keypair::new();
        fixture.transfer(&new_signer.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_add_to_whitelist(&admin, new_signer.pubkey())
            .await
            .unwrap();

        whitelist_management_program_client
            .do_remove_from_whitelist(&admin, new_signer.pubkey())
            .await
            .unwrap();

        let whitelist = whitelist_management_program_client
            .get_whitelist()
            .await
            .unwrap();

        assert_eq!(whitelist.admins[0], admin.pubkey());
        assert_eq!(whitelist.whitelist[0], EMPTY_ADDRESS);
    }

    #[tokio::test]
    async fn test_remove_from_whitelist_bad_admin_fails() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await
            .unwrap();

        let bad_admin = Keypair::new();
        fixture.transfer(&bad_admin.pubkey(), 1.0).await.unwrap();

        let new_admin = Keypair::new();
        fixture.transfer(&new_admin.pubkey(), 1.0).await.unwrap();

        let transaction_error = whitelist_management_program_client
            .do_remove_from_whitelist(&bad_admin, new_admin.pubkey())
            .await;

        assert_ix_error(
            transaction_error,
            InstructionError::Custom(JitoWhitelistManagementError::InvalidAdmin as u32),
        );
    }
}

mod fixtures;

#[cfg(test)]
mod tests {
    use jito_whitelist_management_core::whitelist::EMPTY_ADDRESS;
    use solana_keypair::{Keypair, Signer};
    use solana_transaction::InstructionError;

    use crate::fixtures::{assert_ix_error, TestBuilder};

    #[tokio::test]
    async fn test_remove_admin_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await
            .unwrap();

        let new_admin = Keypair::new();
        fixture.transfer(&new_admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_add_admin(&admin, new_admin.pubkey())
            .await
            .unwrap();

        whitelist_management_program_client
            .do_remove_admin(&admin, new_admin.pubkey())
            .await
            .unwrap();

        let whitelist = whitelist_management_program_client
            .get_whitelist()
            .await
            .unwrap();

        assert_eq!(whitelist.admins[0], admin.pubkey());
        assert_eq!(whitelist.admins[1], EMPTY_ADDRESS);
    }

    #[tokio::test]
    async fn test_remove_admin_bad_admin_fails() {
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
            .do_remove_admin(&bad_admin, new_admin.pubkey())
            .await;

        assert_ix_error(transaction_error, InstructionError::InvalidAccountData);
    }

    #[tokio::test]
    async fn test_remove_admin_self_remove_fails() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await
            .unwrap();

        let new_admin = Keypair::new();
        fixture.transfer(&new_admin.pubkey(), 1.0).await.unwrap();

        let transaction_error = whitelist_management_program_client
            .do_remove_admin(&admin, admin.pubkey())
            .await;

        assert_ix_error(transaction_error, InstructionError::InvalidAccountData);
    }
}

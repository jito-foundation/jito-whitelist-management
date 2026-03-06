mod fixtures;

#[cfg(test)]
mod tests {
    use jito_whitelist_management_core::whitelist::EMPTY_ADDRESS;
    use solana_keypair::{Keypair, Signer};
    use solana_pubkey::Pubkey;
    use solana_transaction::InstructionError;

    use crate::fixtures::{assert_ix_error, TestBuilder};

    #[tokio::test]
    async fn test_initialize_whitelist_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await
            .unwrap();

        let whitelist = whitelist_management_program_client
            .get_whitelist()
            .await
            .unwrap();

        assert_eq!(whitelist.whitelist.len(), 64);

        for pubkey in whitelist.whitelist.iter() {
            assert_eq!(*pubkey, EMPTY_ADDRESS);
        }

        assert_eq!(whitelist.admins.len(), 8);
        assert_eq!(whitelist.admins[0], admin.pubkey());

        for admin in whitelist.admins.iter().skip(1) {
            assert_eq!(*admin, EMPTY_ADDRESS);
        }
    }

    #[tokio::test]
    async fn test_initialize_whitelist_double_init_fails() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await
            .unwrap();

        fixture.warp_slot_incremental(1).await.unwrap();

        let transaction_error = whitelist_management_program_client
            .do_initialize_whitelist(admin.pubkey())
            .await;

        assert_ix_error(transaction_error, InstructionError::InvalidAccountOwner);
    }

    #[tokio::test]
    async fn test_initialize_config_bad_pda_fails() {
        let fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let bad_whitelist = Pubkey::new_unique();
        let initial_admin = Pubkey::new_unique();

        let transaction_error = whitelist_management_program_client
            .initialize_whitelist(bad_whitelist, initial_admin)
            .await;

        assert_ix_error(transaction_error, InstructionError::InvalidAccountData);
    }
}

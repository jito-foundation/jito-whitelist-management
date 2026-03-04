mod fixtures;

#[cfg(test)]
mod tests {
    use solana_keypair::{Keypair, Signer};
    use solana_transaction::InstructionError;

    use crate::fixtures::{assert_ix_error, TestBuilder};

    #[tokio::test]
    async fn test_set_stake_tracking_ok() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        let base = Keypair::new();

        whitelist_management_program_client
            .do_initialize_whitelist(&base, admin.pubkey())
            .await
            .unwrap();

        let new_signer = Keypair::new();
        fixture.transfer(&new_signer.pubkey(), 1.0).await.unwrap();

        whitelist_management_program_client
            .do_add_to_whitelist(&admin, &base, new_signer.pubkey())
            .await
            .unwrap();

        let total_stake_deposited = 10;
        let total_stake_withdrawn = 20;
        let total_withdrawal_fees = 30;

        whitelist_management_program_client
            .set_stake_tracking(
                &new_signer,
                base.pubkey(),
                total_stake_deposited,
                total_stake_withdrawn,
                total_withdrawal_fees,
            )
            .await
            .unwrap();

        let whitelist = whitelist_management_program_client
            .get_whitelist(&base.pubkey())
            .await
            .unwrap();

        assert_eq!(whitelist.total_stake_deposited(), total_stake_deposited);
        assert_eq!(whitelist.total_stake_withdrawn(), total_stake_withdrawn);
        assert_eq!(whitelist.total_withdrawal_fees(), total_withdrawal_fees);
    }

    #[tokio::test]
    async fn test_set_stake_tracking_bad_signer_fails() {
        let mut fixture = TestBuilder::new().await;
        let mut whitelist_management_program_client = fixture.whitelist_management_program_client();

        let admin = Keypair::new();
        fixture.transfer(&admin.pubkey(), 1.0).await.unwrap();

        let base = Keypair::new();

        whitelist_management_program_client
            .do_initialize_whitelist(&base, admin.pubkey())
            .await
            .unwrap();

        let bad_admin = Keypair::new();
        fixture.transfer(&bad_admin.pubkey(), 1.0).await.unwrap();

        let new_admin = Keypair::new();
        fixture.transfer(&new_admin.pubkey(), 1.0).await.unwrap();

        let total_stake_deposited = 10;
        let total_stake_withdrawn = 20;
        let total_withdrawal_fees = 30;

        let transaction_error = whitelist_management_program_client
            .set_stake_tracking(
                &bad_admin,
                base.pubkey(),
                total_stake_deposited,
                total_stake_withdrawn,
                total_withdrawal_fees,
            )
            .await;

        assert_ix_error(transaction_error, InstructionError::InvalidAccountData);
    }
}

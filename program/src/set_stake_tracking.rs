use jito_bytemuck::AccountDeserialize;
use jito_whitelist_management_core::whitelist::Whitelist;
use solana_account_info::AccountInfo;
use solana_program_error::{ProgramError, ProgramResult};
use solana_program_log::log;
use solana_pubkey::Pubkey;

pub fn process_set_stake_tracking(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    total_stake_deposited: u64,
    total_stake_withdrawn: u64,
    total_withdrawal_fees: u64,
) -> ProgramResult {
    let [whitelist_signer_info, whitelist_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !whitelist_signer_info.is_signer {
        log("Whitelist signer is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !whitelist_signer_info.is_writable {
        log("Whitelist signer is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    {
        let whitelist_data = whitelist_info.data.borrow();
        let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;
        Whitelist::load(program_id, whitelist_info, &whitelist.base, true)?;
        whitelist.check_whitelist_signer(whitelist_signer_info.key)?;
    }

    let mut whitelist_data = whitelist_info.data.borrow_mut();
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;
    whitelist.set_total_stake_deposited(total_stake_deposited);
    whitelist.set_total_stake_withdrawn(total_stake_withdrawn);
    whitelist.set_total_withdrawal_fees(total_withdrawal_fees);

    Ok(())
}

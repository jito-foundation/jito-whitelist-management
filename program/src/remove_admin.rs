use jito_bytemuck::AccountDeserialize;
use jito_whitelist_management_core::whitelist::Whitelist;
use solana_account_info::AccountInfo;
use solana_program_error::{ProgramError, ProgramResult};
use solana_program_log::log;
use solana_pubkey::Pubkey;

pub fn process_remove_admin(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let [admin_info, whitelist_info, admin_to_remove_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !admin_info.is_signer {
        log("Admin is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !admin_info.is_writable {
        log("Admin is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    {
        let whitelist_data = whitelist_info.data.borrow();
        let whitelist = Whitelist::try_from_slice_unchecked(&whitelist_data)?;
        Whitelist::load(program_id, whitelist_info, &whitelist.base, true)?;
        whitelist.check_admin(admin_info.key)?;
        whitelist.check_admin(admin_to_remove_info.key)?;
    }

    let mut whitelist_data = whitelist_info.data.borrow_mut();
    let whitelist = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;
    whitelist.remove_admin(admin_info.key, admin_to_remove_info.key)?;

    Ok(())
}

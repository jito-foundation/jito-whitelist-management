use jito_bytemuck::{AccountDeserialize, Discriminator};
use jito_whitelist_management_core::whitelist::{Whitelist, EMPTY_ADDRESS};
use jito_whitelist_management_sdk::error::WhitelistManagementError;
use solana_account_info::AccountInfo;
use solana_cpi::{invoke, invoke_signed};
use solana_program_error::{ProgramError, ProgramResult};
use solana_program_log::log;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use solana_sysvar::Sysvar;

pub fn process_initialize_whitelist(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    let [payer_info, whitelist_info, initial_admin_info, system_program_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer_info.is_signer {
        log("Payer is not a signer");
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !payer_info.is_writable {
        log("Payer is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if whitelist_info
        .owner
        .ne(&solana_system_interface::program::id())
    {
        log("Whitelist is not owned by the system program");
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !whitelist_info.data_is_empty() {
        log("Whitelist data is not empty");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if !whitelist_info.is_writable {
        log("Whitelist is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if system_program_info
        .key
        .ne(&solana_system_interface::program::id())
    {
        log("System Program is not the system program");
        return Err(ProgramError::IncorrectProgramId);
    }

    log("Initializing whitelist");
    let (whitelist_pubkey, whitelist_bump, mut whitelist_seeds) =
        Whitelist::find_program_address(program_id);
    whitelist_seeds.push(vec![whitelist_bump]);
    if whitelist_info.key.ne(&whitelist_pubkey) {
        log!("Whitelist account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    let rent = Rent::get()?;
    let space = 8_u64
        .checked_add(size_of::<Whitelist>() as u64)
        .ok_or(WhitelistManagementError::ArithmeticOverflow)?;

    let current_lamports = **whitelist_info.try_borrow_lamports()?;
    if current_lamports == 0 {
        // If there are no lamports in the account, we create it with create_account
        invoke_signed(
            &solana_system_interface::instruction::create_account(
                payer_info.key,
                whitelist_info.key,
                rent.minimum_balance(space as usize),
                space,
                &crate::id(),
            ),
            &[
                payer_info.clone(),
                whitelist_info.clone(),
                system_program_info.clone(),
            ],
            &[whitelist_seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice()],
        )?;
    } else {
        // Someone can transfer lamports to accounts before they're initialized.
        // In that case, create_account won't work. Instead, transfer the deficit,
        // allocate the required space, and assign ownership to the program.
        let required_lamports = rent
            .minimum_balance(space as usize)
            .saturating_sub(current_lamports);
        if required_lamports > 0 {
            invoke(
                &solana_system_interface::instruction::transfer(
                    payer_info.key,
                    whitelist_info.key,
                    required_lamports,
                ),
                &[
                    payer_info.clone(),
                    whitelist_info.clone(),
                    system_program_info.clone(),
                ],
            )?;
        }

        // Allocate space
        invoke_signed(
            &solana_system_interface::instruction::allocate(whitelist_info.key, space),
            &[whitelist_info.clone(), system_program_info.clone()],
            &[whitelist_seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice()],
        )?;

        // Assign to this program
        invoke_signed(
            &solana_system_interface::instruction::assign(whitelist_info.key, &crate::id()),
            &[whitelist_info.clone(), system_program_info.clone()],
            &[whitelist_seeds
                .iter()
                .map(|seed| seed.as_slice())
                .collect::<Vec<&[u8]>>()
                .as_slice()],
        )?;
    }

    let mut whitelist_data = whitelist_info.data.borrow_mut();
    whitelist_data[0] = Whitelist::DISCRIMINATOR;
    let whitelist_acc = Whitelist::try_from_slice_unchecked_mut(&mut whitelist_data)?;

    whitelist_acc.set_admins([EMPTY_ADDRESS; 8]);
    whitelist_acc.admins[0] = *initial_admin_info.key;
    whitelist_acc.set_whitelist([EMPTY_ADDRESS; 64]);
    whitelist_acc.set_bump(whitelist_bump);

    Ok(())
}

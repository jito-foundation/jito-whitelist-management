use borsh::BorshDeserialize;
use jito_whitelist_management_sdk::instruction::JitoWhitelistManagementInstruction;
use solana_account_info::AccountInfo;
use solana_address::declare_id;
use solana_program_error::{ProgramError, ProgramResult};
use solana_program_log::log;
use solana_pubkey::Pubkey;

mod add_admin;
mod add_to_whitelist;
mod initialize_whitelist;
mod remove_admin;
mod remove_from_whitelist;
mod set_stake_tracking;

declare_id!("Wh1tea995dSzf9q4bmUCPM8s6URjT1HWMrp771bLW7G");

solana_program_entrypoint::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = JitoWhitelistManagementInstruction::try_from_slice(instruction_data)
        .map_err(|_e| ProgramError::InvalidAccountData)?;

    match instruction {
        JitoWhitelistManagementInstruction::InitializeWhitelist => {
            log!("Instruction: InitializeWhitelist");
            initialize_whitelist::process_initialize_whitelist(program_id, accounts)
        }
        JitoWhitelistManagementInstruction::AddAdmin => {
            log!("Instruction: AddAdmin");
            add_admin::process_add_admin(program_id, accounts)
        }
        JitoWhitelistManagementInstruction::RemoveAdmin => {
            log!("Instruction: RemoveAdmin");
            remove_admin::process_remove_admin(program_id, accounts)
        }
        JitoWhitelistManagementInstruction::AddToWhitelist => {
            log!("Instruction: AddToWhitelist");
            add_to_whitelist::process_add_to_whitelist(program_id, accounts)
        }
        JitoWhitelistManagementInstruction::RemoveFromWhitelist => {
            log!("Instruction: RemoveFromWhitelist");
            remove_from_whitelist::process_remove_from_whitelist(program_id, accounts)
        }
        JitoWhitelistManagementInstruction::SetStakeTracking {
            total_stake_deposited,
            total_stake_withdrawn,
            total_withdrawal_fees,
        } => {
            log!("Instruction: SetStakeTracking");
            set_stake_tracking::process_set_stake_tracking(
                program_id,
                accounts,
                total_stake_deposited,
                total_stake_withdrawn,
                total_withdrawal_fees,
            )
        }
    }
}

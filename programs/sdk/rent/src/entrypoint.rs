use {
    solana_account_info::AccountInfo,
    solana_program_entrypoint::{entrypoint, ProgramResult},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_rent::Rent,
    solana_sysvar::Sysvar,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent::get()?;
    if account.lamports() < rent.minimum_balance(account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    Ok(())
}

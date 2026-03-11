use pinocchio::{
    entrypoint,
    error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    AccountView, Address, ProgramResult,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent::get()?;
    if account.lamports() < rent.try_minimum_balance(account.data_len())? {
        return Err(ProgramError::AccountNotRentExempt);
    }

    Ok(())
}

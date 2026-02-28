use {
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
    solana_program_log::log,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    log!("lamports={}", &[account.lamports(), account.lamports()]);

    Ok(())
}

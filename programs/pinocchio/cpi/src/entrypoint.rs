use {
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
    pinocchio_system::instructions::CreateAccount,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [from, to, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    CreateAccount {
        from,
        to,
        lamports: 1_000_000_000,
        space: 10,
        owner: program_id,
    }
    .invoke()
}

use {
    pinocchio::{
        error::ProgramError, no_allocator, nostd_panic_handler, program_entrypoint, AccountView,
        Address, ProgramResult,
    },
    pinocchio_system::instructions::CreateAccount,
};

// Declares the entrypoint of the program.
program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

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

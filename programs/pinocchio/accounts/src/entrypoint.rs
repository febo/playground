use pinocchio::{entrypoint, AccountView, Address, ProgramResult};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}

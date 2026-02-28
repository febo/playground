use pinocchio::{
    no_allocator, nostd_panic_handler, program_entrypoint, AccountView, Address, ProgramResult,
};

// Declares the entrypoint of the program.
program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}

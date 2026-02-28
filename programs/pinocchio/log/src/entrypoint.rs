use {
    pinocchio::{
        error::ProgramError, no_allocator, nostd_panic_handler, program_entrypoint, AccountView,
        Address, ProgramResult,
    },
    solana_program_log::log,
};

// Declares the entrypoint of the program.
program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

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

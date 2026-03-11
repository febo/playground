use {
    solana_account_info::AccountInfo,
    solana_program_entrypoint::{entrypoint, ProgramResult},
    solana_pubkey::Pubkey,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}

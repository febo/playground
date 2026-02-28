use {
    solana_account_info::AccountInfo,
    solana_cpi::invoke,
    solana_program_entrypoint::{entrypoint, ProgramResult},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_system_interface::instruction::create_account,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [from, to, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let create_ix = create_account(from.key, to.key, 1_000_000_000, 10, program_id);
    let create_accounts = [from.clone(), to.clone()];

    invoke(&create_ix, &create_accounts)
}

use {
    pinocchio::{
        entrypoint::inline::ProgramInput, error::ProgramError, execute, inline_program_entrypoint,
        AccountView, Address, ProgramResult,
    },
    pinocchio_system::instructions::CreateAccount,
};

// Declares the entrypoint of the program.
inline_program_entrypoint!(process_instruction);

pub fn process_instruction(input: ProgramInput) -> ProgramResult {
    match input.data.first() {
        Some(&0) => execute!((3, input) => create),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

/// Instruction processor.
pub fn create(
    program_id: &Address,
    accounts: &mut [AccountView],
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

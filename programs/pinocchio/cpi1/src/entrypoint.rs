use pinocchio::{
    cpi::invoke,
    entrypoint,
    error::ProgramError,
    instruction::{InstructionAccount, InstructionView},
    AccountView, Address, ProgramResult,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
#[inline(never)]
pub fn process_instruction(
    program_id: &Address,
    accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [from, to, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let instruction_accounts: [InstructionAccount; 2] = [
        InstructionAccount::writable_signer(from.address()),
        InstructionAccount::writable_signer(to.address()),
    ];

    // instruction data
    // - [0..4  ]: instruction discriminator
    // - [4..12 ]: lamports
    // - [12..20]: account space
    // - [20..52]: owner address
    let mut instruction_data = [0; 52];
    // create account instruction has a '0' discriminator
    instruction_data[4..12].copy_from_slice(&1_000_000_000u64.to_le_bytes());
    instruction_data[12..20].copy_from_slice(&10u64.to_le_bytes());
    instruction_data[20..52].copy_from_slice(program_id.as_ref());

    let instruction = InstructionView {
        program_id: &pinocchio_system::ID,
        accounts: &instruction_accounts,
        data: &instruction_data,
    };

    invoke(&instruction, &[&from, &to])
}

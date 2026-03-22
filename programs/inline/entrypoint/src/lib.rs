use pinocchio::{entrypoint::inline::ProgramInput, inline_program_entrypoint, ProgramResult};

inline_program_entrypoint!(process_instruction);

pub fn process_instruction(_input: ProgramInput) -> ProgramResult {
    core::hint::black_box(Ok(()))
}

#![no_std]

use pinocchio::{
    account::AccountView, no_allocator, nostd_panic_handler, program_entrypoint, Address,
    ProgramResult,
};

program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

fn read_u64_unaligned(data: &[u8]) -> u64 {
    if data.len() < 9 {
        return 1;
    }
    unsafe { core::ptr::read_unaligned(data.as_ptr().add(1) as *const u64) }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let v = read_u64_unaligned(instruction_data);
    core::hint::black_box(v);
    core::hint::black_box(Ok(()))
}

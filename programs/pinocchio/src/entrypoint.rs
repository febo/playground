use core::{mem::size_of, ptr::with_exposed_provenance_mut, slice::from_raw_parts};
use pinocchio::{AccountView, Address, ProgramResult};

/// `assert_eq(core::mem::align_of::<u128>(), 8)` is true for BPF but not
/// for some host machines.
const BPF_ALIGN_OF_U128: usize = 8;

/// Align a pointer to the BPF alignment of [`u128`].
macro_rules! align_pointer {
    ($ptr:ident) => {
        // Integer-to-pointer cast: first compute the aligned address as a `usize`,
        // since this is more CU-efficient than using `ptr::align_offset()` or the
        // strict provenance API (e.g., `ptr::with_addr()`). Then cast the result
        // back to a pointer. The resulting pointer is guaranteed to be valid
        // because it follows the layout serialized by the runtime.
        with_exposed_provenance_mut(
            ($ptr.expose_provenance() + (BPF_ALIGN_OF_U128 - 1)) & !(BPF_ALIGN_OF_U128 - 1),
        )
    };
}

#[no_mangle]
pub unsafe extern "C" fn entrypoint(program_input: *mut u8, instruction_data: *mut u8) -> u64 {
    // First 8-bytes of program_input contains the number of accounts.
    let accounts = program_input as *mut u64;

    // The 8-bytes before the instruction data contains the length of
    // the instruction data, even if the instruction data is empty.
    let ix_data_len = *(instruction_data.sub(size_of::<u64>()) as *mut u64) as usize;

    // The program_id is located right after the instruction data.
    let program_id = &*(instruction_data.add(ix_data_len) as *const Address);

    // The slice of account pointers is located right after the program_id.
    let slice_ptr = instruction_data.add(ix_data_len + size_of::<Address>());
    let accounts = from_raw_parts(
        align_pointer!(slice_ptr) as *const AccountView,
        *accounts as usize,
    );

    // The instruction data slice.
    let instruction_data = from_raw_parts(instruction_data, ix_data_len);

    match process_instruction(program_id, accounts, instruction_data) {
        Ok(_) => 0,
        Err(e) => e.into(),
    }
}

pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    core::hint::black_box(Ok(()))
}

//use pinocchio::error::ProgramError;
use pinocchio::{entrypoint, AccountView, Address, ProgramResult};
//use pinocchio_system::instructions::CreateAccount;
//use solana_program_log::log;

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    // (1) run_accounts
    core::hint::black_box(Ok(()))
    // end of 1

    // (2) run_cpi
    /*
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
    */
    // end of 2

    // (3) run_log
    /*
    let [account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    log!("lamports={}", &[account.lamports(), account.lamports()]);

    Ok(())
    */
    // end of 3
}

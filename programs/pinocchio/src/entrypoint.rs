use pinocchio::{account_info::AccountInfo, entrypoint, pubkey::Pubkey, ProgramResult};
//use pinocchio::program_error::ProgramError;
//use pinocchio_log::log;
//use pinocchio_system::instructions::CreateAccount;

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _nstruction_data: &[u8],
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

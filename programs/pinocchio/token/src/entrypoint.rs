use {
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
    pinocchio_token::instructions::{
        Batch, BatchState, CloseAccount, InitializeAccount3, InitializeMint2, IntoBatch, MintTo,
        Transfer,
    },
};

/// The number of accounts for the batch instruction.
const MAX_ACCOUNTS_LEN: usize = InitializeMint2::ACCOUNTS_LEN
    + InitializeAccount3::ACCOUNTS_LEN
    + InitializeAccount3::ACCOUNTS_LEN
    + MintTo::MAX_ACCOUNTS_LEN
    + Transfer::MAX_ACCOUNTS_LEN
    + CloseAccount::MAX_ACCOUNTS_LEN;

/// The length of the instruction data for the batch instruction.
const MAX_DATA_LEN: usize = Batch::header_data_len(6)
    + InitializeMint2::MAX_DATA_LEN
    + InitializeAccount3::DATA_LEN
    + InitializeAccount3::DATA_LEN
    + MintTo::DATA_LEN
    + Transfer::DATA_LEN
    + CloseAccount::DATA_LEN;

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    accounts: &mut [AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let [mint1, account1, account2, mint_authority, freeze_authority, owner1, owner2, _token_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mut batch_state = BatchState::new(MAX_ACCOUNTS_LEN, MAX_DATA_LEN);
    let mut batch = batch_state.as_batch()?;

    InitializeMint2::new(
        mint1,
        9,
        mint_authority.address(),
        Some(freeze_authority.address()),
    )
    .into_batch(&mut batch)?;

    InitializeAccount3::new(account1, mint1, owner1.address()).into_batch(&mut batch)?;

    InitializeAccount3::new(account2, mint1, owner2.address()).into_batch(&mut batch)?;

    MintTo::new(mint1, account1, mint_authority, 1000).into_batch(&mut batch)?;

    Transfer::new(account1, account2, owner1, 1000).into_batch(&mut batch)?;

    CloseAccount::new(account1, owner1, owner1).into_batch(&mut batch)?;

    batch.invoke()
}

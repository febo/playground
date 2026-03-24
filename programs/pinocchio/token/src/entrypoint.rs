use {
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
    pinocchio_token::instructions::{
        Batch, CloseAccount, InitializeAccount3, InitializeMint2, IntoBatch, MintTo, Transfer,
    },
};

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

    let mut batch = Batch::default();

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

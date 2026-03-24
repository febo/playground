use {
    pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult},
    pinocchio_token::instructions::{
        Batch, CloseAccount, InitializeAccount3, InitializeMint2, MintTo, Transfer,
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

    let initialize_mint1 = InitializeMint2 {
        mint: mint1,
        mint_authority: mint_authority.address(),
        decimals: 9,
        freeze_authority: Some(freeze_authority.address()),
    };

    let initialize_account1 = InitializeAccount3 {
        account: account1,
        mint: mint1,
        owner: owner1.address(),
    };

    let initialize_account2 = InitializeAccount3 {
        account: account2,
        mint: mint1,
        owner: owner2.address(),
    };

    let mint_to = MintTo::new(mint1, account1, mint_authority, 1000);

    let transfer = Transfer::new(account1, account2, owner1, 1000);

    let close_account = CloseAccount::new(account1, owner1, owner1);

    batch.push(&initialize_mint1)?;
    batch.push(&initialize_account1)?;
    batch.push(&initialize_account2)?;
    batch.push(&mint_to)?;
    batch.push(&transfer)?;
    batch.push(&close_account)?;

    batch.invoke()
}

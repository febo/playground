use {
    mollusk_svm::{program::keyed_account_for_system_program, Mollusk},
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    solana_account::Account,
    solana_instruction::{AccountMeta, Instruction},
    solana_pubkey::Pubkey,
};

/// System program ID, used for creating accounts.
const SYSTEM_PROGRAM: Pubkey = Pubkey::new_from_array([0; 32]);

/// Base lamports for accounts, used to ensure accounts are rent-exempt.
pub const BASE_LAMPORTS: u64 = 2_000_000_000u64;

/// Create a new Mollusk instance for the given program ID and name.
pub fn setup(program_id: &Pubkey, name: &'static str) -> Mollusk {
    std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    solana_logger::setup();

    Mollusk::new(program_id, name)
}

/// Generate a set of unique public keys.
pub fn generate_pubkeys(count: usize) -> Vec<Pubkey> {
    let mut keys = Vec::with_capacity(count);
    for _ in 0..count {
        keys.push(Pubkey::new_unique());
    }
    keys
}

/// Generates the instruction data and accounts to execute a program.
pub fn entrypoint_instruction(
    program_id: Pubkey,
    expected: u64,
) -> (Instruction, Vec<(Pubkey, Account)>) {
    let mut keys = generate_pubkeys(expected as usize);

    let mut accounts = Vec::with_capacity(keys.len());
    let mut account_metas = Vec::with_capacity(keys.len());

    for _ in 0..keys.len() {
        let key = keys.pop().unwrap();
        accounts.push((key, Account::new(BASE_LAMPORTS, 0, &SYSTEM_PROGRAM)));
        account_metas.push(AccountMeta::new_readonly(key, false));
    }

    (
        Instruction {
            program_id,
            accounts: account_metas,
            data: vec![],
        },
        accounts,
    )
}

/// Generates the instruction data and accounts to execute a program that
/// performs a cross-program invocation (CPI).
pub fn cpi_instruction(program_id: Pubkey) -> (Instruction, Vec<(Pubkey, Account)>) {
    let keys = generate_pubkeys(2);
    let (system_program_id, system_program_account) = keyed_account_for_system_program();

    let accounts = vec![
        (keys[0], Account::new(BASE_LAMPORTS, 0, &SYSTEM_PROGRAM)),
        (keys[1], Account::new(0, 0, &SYSTEM_PROGRAM)),
        (system_program_id, system_program_account),
    ];

    let account_metas = vec![
        AccountMeta::new(keys[0], true),
        AccountMeta::new(keys[1], true),
        AccountMeta::new_readonly(system_program_id, false),
    ];

    (
        Instruction {
            program_id,
            accounts: account_metas,
            data: vec![],
        },
        accounts,
    )
}

/// Generates the instruction data and accounts to execute a program that
/// logs the lamports of an account.
pub fn log_instruction(program_id: Pubkey) -> (Instruction, Vec<(Pubkey, Account)>) {
    let key = generate_pubkeys(1).pop().unwrap();
    let accounts = vec![(key, Account::new(BASE_LAMPORTS, 0, &SYSTEM_PROGRAM))];

    let account_metas = vec![AccountMeta::new_readonly(key, false)];

    (
        Instruction {
            program_id,
            accounts: account_metas,
            data: vec![],
        },
        accounts,
    )
}

macro_rules! generate_entrypoint_bench {
    ( $bencher:ident, $program_id:ident, $expected:expr ) => {
        let (instruction, accounts) = entrypoint_instruction(*$program_id, $expected);
        let name = format!("Account ({})", $expected);

        $bencher = $bencher.bench((
            &name,
            &instruction,
            &accounts,
        ));
    };

    ( $bencher:ident, $program_id:ident, $( $expected:expr ),+ $(,)? ) => {
        $(
            generate_entrypoint_bench!($bencher, $program_id, $expected);
        )+
    };
}

pub fn run_accounts(program_id: &Pubkey, name: &'static str) {
    let mollusk = setup(program_id, name);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    generate_entrypoint_bench!(bencher, program_id, 0, 1, 2, 3, 4, 8, 16, 32, 64);

    // Run the benchmarks.

    bencher.execute();
}

pub fn run_cpi(program_id: &Pubkey, name: &'static str) {
    let mollusk = setup(program_id, name);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    // CPI to system program.

    let (instruction, accounts) = cpi_instruction(*program_id);
    bencher = bencher.bench(("system_program::create", &instruction, &accounts));

    bencher.execute();
}

pub fn run_log(program_id: &Pubkey, name: &'static str) {
    let mollusk = setup(program_id, name);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    // Log lamports of an account.

    let (instruction, accounts) = log_instruction(*program_id);
    bencher = bencher.bench(("log", &instruction, &accounts));

    bencher.execute();
}

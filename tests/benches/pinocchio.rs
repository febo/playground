#![feature(test)]

mod setup;

extern crate mollusk_svm;
extern crate mollusk_svm_bencher;
extern crate solana_account;
extern crate solana_instruction;
extern crate solana_pubkey;
extern crate test;

use solana_pubkey::Pubkey;

const PROGRAM_ID: Pubkey = Pubkey::new_from_array([2; 32]);

#[cfg(test)]
mod pinocchio {

    use {super::*, test::Bencher};

    #[bench]
    fn entrypoint(_bencher: &mut Bencher) {
        setup::run_accounts(&PROGRAM_ID, "libpinocchio_accounts");
    }

    #[bench]
    fn cpi(_bencher: &mut Bencher) {
        setup::run_cpi(&PROGRAM_ID, "libpinocchio_cpi");
    }

    #[bench]
    fn log(_bencher: &mut Bencher) {
        setup::run_log(&PROGRAM_ID, "libpinocchio_log");
    }  
    
    #[bench]
    fn input_vec(_bencher: &mut Bencher) {
        setup::run_input_vec(&PROGRAM_ID, "libpinocchio_unalignment");
    }
}

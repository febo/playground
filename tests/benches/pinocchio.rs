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
        setup::run_entrypoint(&PROGRAM_ID, "pinocchio_entrypoint");
    }

    #[bench]
    fn cpi(_bencher: &mut Bencher) {
        setup::run_cpi(&PROGRAM_ID, "pinocchio_cpi");
    }

    #[bench]
    fn log(_bencher: &mut Bencher) {
        setup::run_log(&PROGRAM_ID, "pinocchio_log");
    }

    #[bench]
    fn rent(_bencher: &mut Bencher) {
        setup::run_rent(&PROGRAM_ID, "pinocchio_rent");
    }

    #[bench]
    fn token(_bencher: &mut Bencher) {
        setup::run_token(&PROGRAM_ID, "pinocchio_token");
    }
}

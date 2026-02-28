//! Simple Solana SDK program.

use solana_pubkey::Pubkey;

mod entrypoint;

pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array([2; 32]);

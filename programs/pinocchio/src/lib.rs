//! Simple pinocchio program.

use pinocchio::Address;

mod entrypoint;

pub const PROGRAM_ID: Address = Address::new_from_array([2; 32]);

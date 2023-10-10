#![no_std]

mod storage;

use soroban_sdk::{contract, contractimpl};
use interface::ContractAInterface;
use crate::storage::*;

#[contract]
pub struct ContractA;

#[contractimpl]
impl ContractAInterface for ContractA {
    fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}

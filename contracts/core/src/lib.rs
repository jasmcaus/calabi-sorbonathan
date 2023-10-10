#![no_std]
#![allow(unused)]

mod storage;

use soroban_sdk::{contract, contractimpl};
use interface::ContractAInterface;
use crate::storage::*;

#[contract]
pub struct LendingPool;

#[contractimpl]
impl LendingPool {
    fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}

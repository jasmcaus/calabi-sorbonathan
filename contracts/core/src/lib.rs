#![no_std]
#![allow(unused)]

mod storage;

use crate::storage::*;
use interface::ContractAInterface;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct LendingPool;

#[contractimpl]
impl LendingPool {
    fn add(x: u32, y: u32) -> u32 {
        x.checked_add(y).expect("no overflow")
    }
}

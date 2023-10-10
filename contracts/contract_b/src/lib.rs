#![no_std]

use soroban_sdk::{contract, contractimpl, Address, Env};
use interface::ContractAClient;

#[contract]
pub struct ContractB;

#[contractimpl]
impl ContractB {
    pub fn add_with(env: Env, contract: Address, x: u32, y: u32) -> u32 {
        let client = ContractAClient::new(&env, &contract);
        client.add(&x, &y)
    }
}

mod test;

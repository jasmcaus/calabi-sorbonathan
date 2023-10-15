#![no_std]
#![allow(unused)]

mod storage;

use soroban_sdk::{contract, contractimpl, Env, Address, token};
use crate::storage::*;
use interface::lendingpool::ILendingPool;

#[contract]
pub struct LendingPool;

#[contractimpl]
impl ILendingPool for LendingPool {
    fn create_lending_offer(
        env: Env,
        from: Address,
        principle_amount: u128,
        principle_asset: Address,
        collateral_asset: Address,
        days_to_maturity: u128,
        interest_rate: u32,
        days_to_expire: u32
    ) {
        from.require_auth();

        // Extract tokens
        token::Client::new(&env, &principle_asset).transfer(&from, &env.current_contract_address(), &(principle_amount as i128));
        
        

    }
}

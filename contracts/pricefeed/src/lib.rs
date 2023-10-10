#![allow(unused)]
#![no_std]

mod storage;

use soroban_sdk::{contract, contractimpl, Address, Env};
use interface::ContractAClient;

use crate::storage::*;

#[contract]
pub struct PriceFeed;

#[contractimpl]
impl PriceFeed {
    pub fn add_pricefeed(env: Env, asset: Address, feed: Address) {
        let key = StorageKey::FeedAddresses(asset);

        env.storage().persistent().set(&key, &feed);
    }

    pub fn get_pricefeed(env: Env, asset: Address) -> Address {
        let key = StorageKey::FeedAddresses(asset);

        env.storage().persistent().get(&key).unwrap() 
    }

    pub fn get_latest_price(env: Env, asset: Address) -> (u64, u32) {
        let feed = Self::get_pricefeed(env, asset);

        (1_000_000, 6)
    }

    pub fn add_with(env: Env, contract: Address, x: u32, y: u32) -> u32 {
        let client = ContractAClient::new(&env, &contract);
        client.add(&x, &y)
    }
}
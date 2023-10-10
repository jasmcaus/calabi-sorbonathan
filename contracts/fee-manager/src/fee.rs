#![allow(unused)]
use soroban_sdk::{Address, Env, contracttype, contract, contractimpl};

#[derive(Clone)]
#[repr(u32)]
#[contracttype]
enum StorageKey {
    Balance(Address)
}

#[contract]
pub struct FeeManager;

#[contractimpl]
impl FeeManager {
    fn get_balance(env: &Env, asset: Address) -> i128 {
        let key = StorageKey::Balance(asset);

        env.storage().persistent().get(&key).unwrap_or(0)
    }

    fn update_balance(env: Env, asset: Address, new_amount: i128) {
        let key = StorageKey::Balance(asset.clone());

        env.storage().persistent().set(&key, &new_amount);
    }

    pub fn credit(env: Env, asset: Address, amount: i128) {
        let existing_balance = Self::get_balance(&env, asset.clone());

        Self::update_balance(env, asset.clone(), amount + existing_balance);
    }

    pub fn debit(env: Env, asset: Address, amount: i128) {
        let existing_balance = Self::get_balance(&env, asset.clone());

        if existing_balance < amount  {
            panic!("Insufficient Collateral");
        }

        Self::update_balance(env, asset, existing_balance - amount);
    }

    pub fn balance_of(env: Env, asset: Address) -> i128 {
        Self::get_balance(&env, asset)
    }

    pub fn fee_percentage(env: Env) -> u32 {
        5 // 2%
    }
}
use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[repr(u32)]
#[contracttype]
pub enum StorageKey {
    FeedAddresses(Address),
    TokenA,
}
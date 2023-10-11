use soroban_sdk::contracttype;

#[derive(Clone, Copy)]
#[repr(u32)]
#[contracttype]
pub enum StorageKey {
    TokenA,
}

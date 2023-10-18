use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone, Copy)]
#[contracttype]
pub struct InterestRateInfo {
    pub last_block: u128,
    pub fee_to_protocol_rate: u128,
    pub latest_timestamp: u128,
    pub rate_per_sec: u128,
    pub optimal_utilization: u128,
    pub base_rate: u128,
    pub slope_1: u128,
    pub slope_2: u128,
}

#[derive(Clone, Copy)]
#[contracttype]
pub struct InterestRateParams {
    pub fee_to_protocol_rate: u128,
    pub optimal_utilization: u128,
    pub base_rate: u128,
    pub slope_1: u128,
    pub slope_2: u128,
}

#[derive(Clone, Copy)]
#[contracttype]
pub struct Vault {
    pub amount: u128,
    pub shares: u128
}

#[derive(Clone, Copy)]
#[contracttype]
pub struct AssetVault {
    pub total_asset: Vault,
    pub total_borrow: Vault,
    pub interest_rate_info: InterestRateInfo
}

#[derive(Clone)]
#[contracttype]
pub struct CollateralSharesKey {
    user: Address,
    asset: Address
}

#[derive(Clone)]
#[repr(u32)]
#[contracttype]
pub enum StorageKey {
    TokenA,
    Vaults(Address),
    CollateralShares(CollateralSharesKey),
    BorrowShares(Address),
}


pub fn __get_vault(env: &Env, asset: &Address) -> AssetVault {
    let key = StorageKey::Vaults(asset.clone());

    if let Some(vault) = env.storage().persistent().get(&key) {
        vault 
    } else {
        let v = Vault { amount: 0, shares: 0 };
        let i = InterestRateInfo {
            last_block: 0,
            fee_to_protocol_rate: 0,
            latest_timestamp: 0,
            rate_per_sec: 0,
            optimal_utilization: 0,
            base_rate: 0,
            slope_1: 0,
            slope_2: 0,
        };
        AssetVault { total_asset: v, total_borrow: v, interest_rate_info: i }
    }
}

pub fn __set_vault(env: &Env, asset: &Address, asset_vault: &AssetVault) {
    let key = StorageKey::Vaults(asset.clone());

    env.storage().persistent().set(&key, asset_vault);
}


pub fn __get_collateral_shares(env: &Env, user: &Address, asset: &Address) -> u128 {
    let key = StorageKey::CollateralShares(CollateralSharesKey { user: user.clone(), asset: asset.clone() });

    if let Some(shares) = env.storage().persistent().get(&key) {
        shares 
    } else {
        0
    }
}

pub fn __set_collateral_shares(env: &Env, user: &Address, asset: &Address, new_shares: u128) {
    let key = StorageKey::CollateralShares(CollateralSharesKey { user: user.clone(), asset: asset.clone() });

    env.storage().persistent().set(&key, &new_shares);
}
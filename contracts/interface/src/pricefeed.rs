use soroban_sdk::{contractclient, Address, Env};

#[contractclient(name = "PriceFeed")]
pub trait IPriceFeed {
    fn add_pricefeed(env: Env, asset: Address, feed: Address);
    fn get_pricefeed(env: Env, asset: Address) -> Address;
    fn get_latest_price(env: Env, asset: Address) -> (u128, u32);
}

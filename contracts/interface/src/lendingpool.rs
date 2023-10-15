use soroban_sdk::{contractclient, Env, Address};

#[contractclient(name = "LendingPool")]
pub trait ILendingPool {
    fn create_lending_offer(
        env: Env,
        from: Address,
        principle_amount: u128,
        principle_asset: Address,
        collateral_asset: Address,
        days_to_maturity: u128,
        interest_rate: u32,
        days_to_expire: u32
    );
}
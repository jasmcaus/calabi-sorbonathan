use soroban_sdk::{contractclient, Env, Address};

#[contractclient(name = "LendingPool")]
pub trait ILendingPool {
    fn supply(
        env: Env,
        asset: Address,
        amount: u128,
        from: Address,
    );
}
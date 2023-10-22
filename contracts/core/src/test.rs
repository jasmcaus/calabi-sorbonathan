#![cfg(test)]
extern crate std;

use super::*;
use crate::assertions::*;
use crate::core::LendingPool;
use interface::pricefeed::*;
use soroban_sdk::{
    contract, contractimpl, symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env,
};
use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

mod pricefeed_core {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/pricefeed.wasm");
}

use pricefeed_core::*;

fn create_token_contract<'a>(
    env: &Env,
    admin: &Address,
) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let contract_address = env.register_stellar_asset_contract(admin.clone());

    (
        TokenClient::new(env, &contract_address),
        TokenAdminClient::new(env, &contract_address),
    )
}

fn create_pricefeed(env: &Env) {
    let pricefeed =
        PriceFeedClient::new(env, &env.register_contract(None, PriceFeed {}));
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let a = Address::random(&env);
    let token_admin = Address::random(&env);

    let (token, token_admin) = create_token_contract(&env, &token_admin);
    token_admin.mint(&a, &1000);
    assert_eq!(token.balance(&a), 1000);

    // Supply
}

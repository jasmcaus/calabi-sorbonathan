#![no_std]
#![allow(unused)]

mod constants;
mod storage;

use core::panic;

use crate::constants::*;
use crate::storage::*;
use interface::lendingpool::ILendingPool;
use soroban_sdk::{contract, contractimpl, token, Address, Env};

#[contract]
pub struct LendingPool;

#[contractimpl]
impl ILendingPool for LendingPool {
    fn supply(env: Env, asset: Address, amount: u128, from: Address) {
        from.require_auth();
        __accrue_interest(&env, &asset);

        // Extract tokens
        token::Client::new(&env, &asset).transfer(
            &from,
            &env.current_contract_address(),
            &(amount as i128),
        );

        let mut asset_vault = __get_vault(&env, &asset);

        let shares = __to_shares(asset_vault.total_asset, amount, false);

        asset_vault.total_asset.shares += shares;
        asset_vault.total_asset.amount += amount;

        let mut new_shares = __get_collateral_shares(&env, &from, &asset);
        new_shares += shares;
        __set_collateral_shares(&env, &from, &asset, new_shares)
    }

    fn borrow(env: Env, asset: Address, amount: u128, from: Address) {
        from.require_auth();
        __accrue_interest(&env, &asset);

        // Extract tokens
        token::Client::new(&env, &asset).transfer(
            &from,
            &env.current_contract_address(),
            &(amount as i128),
        );

        let mut asset_vault = __get_vault(&env, &asset);

        let shares = __to_shares(asset_vault.total_borrow, amount, false);

        asset_vault.total_borrow.shares += shares;
        asset_vault.total_borrow.amount += amount;

        let mut new_shares = __get_borrow_shares(&env, &from, &asset);
        new_shares += shares;
        __set_borrow_shares(&env, &from, &asset, new_shares)

        // if __health_factor(&from) <=  MIN_HEALTH_FACTOR {
        //     panic!("Borrow not allowed (health factor too low)");
        // }
    }
}

/// Returns:
/// _interest_earned
/// _fees_amount
/// _fees_share
/// new_rate
fn __accrue_interest(env: &Env, asset: &Address) -> (u128, u128, u128, u64) {
    let mut asset_vault = __get_vault(&env, &asset);

    let mut _interest_earned: u128 = 0;
    let mut _fees_amount: u128 = 0;
    let mut _fees_share: u128 = 0;
    let mut new_rate: u128 = 0;

    if asset_vault.total_asset.amount == 0 {
        return (0, 0, 0, 0);
    }

    let timestamp = env.ledger().timestamp() as u128;

    let mut current_rate_info = asset_vault.interest_rate_info;
    if current_rate_info.latest_timestamp == timestamp {
        new_rate = current_rate_info.rate_per_sec;
        return (
            _interest_earned,
            _fees_amount,
            _fees_share,
            new_rate.try_into().unwrap(),
        );
    }

    // No borrows, no interest
    if asset_vault.total_borrow.shares == 0 {
        current_rate_info.latest_timestamp = timestamp;
        current_rate_info.last_block = timestamp;
        asset_vault.interest_rate_info = current_rate_info;
    } else {
        let time_delta = timestamp - current_rate_info.last_block;

        let utilization =
            (asset_vault.total_borrow.amount * RATE_PRECISION) / asset_vault.total_asset.amount;

        new_rate = __calculate_interest_rate(current_rate_info.clone(), utilization);

        current_rate_info.rate_per_sec = new_rate;
        current_rate_info.latest_timestamp = env.ledger().timestamp() as u128;
        current_rate_info.last_block = env.ledger().timestamp() as u128;

        // Calculate interest accured
        _interest_earned =
            (time_delta * asset_vault.total_borrow.amount * current_rate_info.rate_per_sec)
                / (RATE_PRECISION * BLOCKS_PER_YEAR);

        // Accumulate interest and fees
        asset_vault.total_borrow.amount += _interest_earned;
        asset_vault.total_asset.amount += _interest_earned;
        asset_vault.interest_rate_info = current_rate_info;

        if (current_rate_info.fee_to_protocol_rate > 0) {
            _fees_amount = (_interest_earned * current_rate_info.fee_to_protocol_rate) / PRECISION;

            _fees_share = (_fees_amount * asset_vault.total_asset.shares)
                / (asset_vault.total_asset.amount - _fees_amount);

            asset_vault.total_asset.shares += _fees_share;

            // give fee shares to this contract
            // userCollateralShares[address(this)][token] += _fees_share;
        }
    }

    __set_vault(&env, &asset, &asset_vault);

    (
        _interest_earned,
        _fees_amount,
        _fees_share,
        new_rate.try_into().unwrap(),
    )
}

fn __calculate_interest_rate(interest_rate_info: InterestRateInfo, utilization: u128) -> u128 {
    let mut new_rate_per_sec = 0 as u128;

    let optimal_utilization = interest_rate_info.optimal_utilization;
    let base_rate = interest_rate_info.base_rate;
    let slope_1 = interest_rate_info.slope_1;
    let slope_2 = interest_rate_info.slope_2;

    if utilization <= optimal_utilization {
        let rate = (utilization * slope_1) / optimal_utilization;
        new_rate_per_sec = base_rate + rate;
    } else {
        let utilization_delta = utilization - optimal_utilization;
        let excess_utilization_rate =
            (utilization_delta * RATE_PRECISION) / (RATE_PRECISION - optimal_utilization);
        new_rate_per_sec =
            base_rate + slope_1 + (excess_utilization_rate * slope_2) / RATE_PRECISION;
    }

    new_rate_per_sec
}

fn __to_shares(vault: Vault, amount: u128, round_up: bool) -> u128 {
    let mut shares = 0 as u128;

    if (vault.amount == 0) {
        shares = amount;
    } else {
        shares = (amount * vault.shares) / vault.amount;
        if round_up && (shares * vault.amount) / vault.shares < amount {
            shares = shares + 1;
        }
    }

    shares
}

fn __to_amount(vault: Vault, shares: u128, round_up: bool) -> u128 {
    let mut amount = 0 as u128;

    if vault.shares == 0 {
        amount = shares;
    } else {
        amount = (shares * vault.amount) / vault.shares;
        if round_up && (amount * vault.shares) / vault.amount < shares {
            amount = amount + 1;
        }
    }

    amount
}

fn __health_factor(user: &Address) {
    !todo!("Health factor todo")
}

fn __get_user_data(user: &Address) {
    !todo!("Get user data todo")
}

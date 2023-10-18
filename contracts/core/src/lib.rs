#![no_std]
#![allow(unused)]

mod constants;
mod storage;

use core::panic;

use crate::constants::*;
use crate::storage::*;
use interface::lendingpool::ILendingPool;
use interface::pricefeed::PriceFeed;
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
        __set_vault(&env, &asset, &asset_vault);

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
        __set_vault(&env, &asset, &asset_vault);

        let mut new_shares = __get_borrow_shares(&env, &from, &asset);
        new_shares += shares;
        __set_borrow_shares(&env, &from, &asset, new_shares)

        // if __health_factor(&from) <=  MIN_HEALTH_FACTOR {
        //     panic!("Borrow not allowed (health factor too low)");
        // }
    }

    fn repay(env: Env, asset: Address, mut amount: u128, from: Address) {
        from.require_auth();
        __accrue_interest(&env, &asset);

        let mut asset_vault = __get_vault(&env, &asset);

        let user_borrow_share = __get_borrow_shares(&env, &from, &asset);
        let mut shares = __to_shares(asset_vault.total_borrow, amount, false);

        if shares > user_borrow_share {
            shares = user_borrow_share;
            amount = __to_amount(asset_vault.total_borrow, shares, false);
        }

        // Extract tokens
        token::Client::new(&env, &asset).transfer(
            &env.current_contract_address(),
            &from,
            &(amount as i128),
        );

        asset_vault.total_borrow.shares -= shares;
        asset_vault.total_borrow.amount -= amount;
        __set_vault(&env, &asset, &asset_vault);

        let mut new_shares = user_borrow_share - shares;
        __set_borrow_shares(&env, &from, &asset, new_shares)
    }

    fn liquidate(
        env: Env,
        account: Address,
        collateral: Address,
        borrow_token: Address,
        mut amount_to_liquidate: u128,
        from: Address,
    ) {
        from.require_auth();

        // if __health_factor(&from) >=  MIN_HEALTH_FACTOR {
        //     panic!("Borrow is solvant");
        // }

        let collateral_shares = __get_collateral_shares(&env, &account, &collateral);
        let borrow_shares = __get_borrow_shares(&env, &account, &borrow_token);

        if collateral_shares == 0 || borrow_shares == 0 {
            panic!("Invalid liquidation");
        }

        let mut borrow_vault = __get_vault(&env, &borrow_token);
        let total_borrow_amount = __to_amount(borrow_vault.total_borrow, borrow_shares, false);
        let max_borrow_amount_to_liquidate =
            (total_borrow_amount * LIQUIDATION_CLOSE_FACTOR) / PRECISION;
        amount_to_liquidate = if amount_to_liquidate > max_borrow_amount_to_liquidate {
            max_borrow_amount_to_liquidate
        } else {
            amount_to_liquidate
        };

        let mut collateral_vault = __get_vault(&env, &collateral);

        let mut collateral_amount_to_liquidate = 0 as u128;
        let mut liquidation_reward = 0;

        {
            let user_total_collateral_amount =
                __to_amount(collateral_vault.total_asset, collateral_shares, false);

            let collateral_price = __get_asset_price(&env, &collateral);
            let borrow_token_price = __get_asset_price(&env, &borrow_token);

            collateral_amount_to_liquidate =
                (amount_to_liquidate * borrow_token_price) / collateral_price;
            let max_liquidation_reward =
                (collateral_amount_to_liquidate * LIQUIDATION_REWARD) / PRECISION;

            if collateral_amount_to_liquidate > user_total_collateral_amount {
                collateral_amount_to_liquidate = user_total_collateral_amount;
                amount_to_liquidate =
                    (user_total_collateral_amount * collateral_price) / borrow_token_price;
            } else {
                let collateral_balance_after =
                    user_total_collateral_amount - collateral_amount_to_liquidate;
                liquidation_reward = if max_liquidation_reward > collateral_balance_after {
                    collateral_balance_after
                } else {
                    max_liquidation_reward
                }
            }

            // Update borrow vault
            let repaid_borrow_shares =
                __to_shares(borrow_vault.total_borrow, amount_to_liquidate, false);

            borrow_vault.total_borrow.shares -= repaid_borrow_shares;
            borrow_vault.total_borrow.amount -= amount_to_liquidate;

            // Update collateral vault
            let liquidated_collateral_shares = __to_shares(
                collateral_vault.total_asset,
                collateral_amount_to_liquidate + liquidation_reward,
                false,
            );

            collateral_vault.total_asset.shares -= liquidated_collateral_shares;
            collateral_vault.total_asset.amount -=
                collateral_amount_to_liquidate + liquidation_reward;

            __set_borrow_shares(
                &env,
                &account,
                &borrow_token,
                (borrow_shares - repaid_borrow_shares),
            );
            __set_collateral_shares(
                &env,
                &account,
                &collateral,
                (collateral_shares - liquidated_collateral_shares),
            );
        }

        // Repay borrowed amount
        token::Client::new(&env, &borrow_token).transfer(
            &from,
            &env.current_contract_address(),
            &(amount_to_liquidate as i128),
        );

        // Repay collateral and liquidation reward to liquidator
        token::Client::new(&env, &collateral).transfer(
            &env.current_contract_address(),
            &from,
            &((collateral_amount_to_liquidate + liquidation_reward) as i128),
        );

        // Save vaults
        __set_vault(&env, &collateral, &collateral_vault);
        __set_vault(&env, &borrow_token, &borrow_vault);
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

fn __get_asset_price(env: &Env, asset: &Address) -> u128 {
    let pricefeed = __get_pricefeed(&env, &asset);

    let client = PriceFeed::new(env, &pricefeed);

    let result = client.get_latest_price(asset);

    result.0 / result.1 as u128
}

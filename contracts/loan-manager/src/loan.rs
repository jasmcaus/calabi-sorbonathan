#![allow(unused)]
use soroban_sdk::{Address, Env, contracttype, contract, contractimpl};

use crate::storage::*;
use crate::assertions::*;

const DUST_AMOUNT: u128 = 100;

#[contract]
pub struct LoanManager;

#[contractimpl]
impl LoanManager {
    pub fn create_loan(
        env: Env,
        is_lending_offer: bool, // if false, it's borrowing offer
        borrower: Address,
        lender: Address,
        principle_asset: Address,
        collateral_asset: Address,
        collateral_amount: u128,
        principle_amount: u128,
        collateral_price: u128,
        interest_rate: u32,
        unclaimed_borrowed_principle: u128,
    ) -> u32 {
        __increment_loan_id(&env);
        let loan_id = __get_loan_id(&env);

        require(lender != borrower, "Invalid lender/borrower");

        if is_lending_offer {

        } else {

        }

        let now = env.ledger().timestamp();
        let duration = (24 * 60 * 60) * 1;
        let maturity_date = now + duration;

        let loan = Loan {
            exists: true,
            offer_id: loan_id,
            state: LoanState::ACTIVE,
            borrower,
            lender,
            principle_asset,
            collateral_asset,
            initial_principle: principle_amount,
            current_principle: principle_amount,
            initial_collateral: collateral_amount,
            current_collateral: collateral_amount,
            collateral_price,
            interest_rate,
            start_date: now as u128,
            maturity_date: maturity_date as u128,
            
            num_installments_paid: 0,

            unclaimed_principle: 0,
            unclaimed_collateral: 0,
            unclaimed_default_collateral: 0,
            unclaimed_borrowed_principle: 0,
            total_interest_paid: 0,
            repaid_on: 0,
        };

        __set_loan(&env, loan_id, &loan);

        loan_id
    }

    pub fn repay_loan(
        env: Env,
        loan_id: u32,
        interest_paid: u128, 
        principle_paid: u128, 
        collateral_received: u128
    ) {
        let mut loan = __get_loan(&env, loan_id);
        require(loan.exists, "Loan doesn't exist lol");

        loan.num_installments_paid += 1;
        loan.total_interest_paid += interest_paid;
        loan.unclaimed_principle += principle_paid + interest_paid;
        loan.unclaimed_collateral += collateral_received;
        loan.current_principle -= principle_paid;
        loan.current_collateral -= collateral_received;

        if loan.current_principle <= DUST_AMOUNT {
            loan.state = LoanState::REPAID;
        }

        __set_loan(&env, loan_id, &loan);
    }


    pub fn get_loan(env: Env, loan_id: u32) -> Loan {
        __get_loan(&env, loan_id)
    }

}
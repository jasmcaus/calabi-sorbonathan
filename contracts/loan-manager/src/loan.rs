#![allow(unused)]
use soroban_sdk::{Address, Env, contracttype, contract, contractimpl};

use crate::storage::*;
use crate::assertions::*;

#[contract]
pub struct LoanManager;

#[contractimpl]
impl LoanManager {
    pub fn create_loan(
        env: Env,
        is_lending_offer: bool, // if false, it's borrowing offer
        borrower: Address,
        lender: Address,
        principal_asset: Address,
        collateral_asset: Address,
        collateral_amount: i128,
        principal_amount: i128,
        collateral_price: i128,
        interest_rate: u32,
        unclaimed_borrowed_principal: i128,
    ) -> u32 {
        increment_loan_id(&env);
        let loan_id = get_loan_id(&env);

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
            principal_asset,
            collateral_asset,
            initial_principal: principal_amount,
            current_principal: principal_amount,
            initial_collateral: collateral_amount,
            current_collateral: collateral_amount,
            collateral_price,
            interest_rate,
            start_date: now as i128,
            maturity_date: maturity_date as i128,
            
            num_installments_paid: 0,

            unclaimed_principal: 0,
            unclaimed_collateral: 0,
            unclaimed_default_collateral: 0,
            unclaimed_borrowed_principal: 0,
            total_interest_paid: 0,
            repaid_on: 0,
        };

        set_loan(&env, loan_id, loan);

        loan_id
    }
}
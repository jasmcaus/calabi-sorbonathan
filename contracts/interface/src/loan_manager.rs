#![allow(unused)]
use soroban_sdk::{contract, contractimpl, contracttype, contractclient, Address, Env};


#[derive(Clone, Eq, PartialEq)]
#[contracttype]
pub enum LoanState {
    ACTIVE,
    REPAID,
    ACTIVEDEFAULTED,
    REPAIDDEFAULTED,
}


#[derive(Clone)]
#[contracttype]
pub struct Loan {
    pub exists: bool,

    pub offer_id: u32,
    pub state: LoanState,

    pub borrower: Address,
    pub lender: Address,

    pub principle_asset: Address,
    pub collateral_asset: Address,

    pub initial_principle: u128,
    pub current_principle: u128,
    pub initial_collateral: u128,
    pub current_collateral: u128,

    // worth of collateral in USD
    pub collateral_price: u128,

    // loan interest_rate rate per seconds
    pub interest_rate: u32,
    pub start_date: u128,
    pub maturity_date: u128,

    pub num_installments_paid: u32,

    // represents principle + interest_rate was paid payback by the borrower tha lender has not claimed
    pub unclaimed_principle: u128,
    // represents collateral amount was unlocked that the borrower has noted
    pub unclaimed_collateral: u128,
    // represents collateral amount retrieved from a borrower when default that the lender has not claimed
    pub unclaimed_default_collateral: u128,
    // represents principle amount the borrower has not claimed
    pub unclaimed_borrowed_principle: u128,
    // represents total interest_rate paid by borrower
    pub total_interest_paid: u128,
    // seconds of full/installment repaid loan
    pub repaid_on: u128,
}


#[contractclient(name = "LoanManager")]
pub trait ILoanManager {
    fn create_loan(
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
    ) -> u32;

    fn repay_loan(
        env: Env,
        loan_id: u32,
        interest_paid: u128,
        principle_paid: u128,
        collateral_received: u128,
    );

    fn claim_principle(env: Env, loan_id: u32, user: Address) -> (u128, u32);

    fn claim_collateral(env: Env, loan_id: u32, user: Address) -> (u128, u32);

    fn claim_borrowed_principle(env: Env, loan_id: u32, user: Address) -> (u128, u32);

    fn liquidate_loan(
        env: Env,
        loan_id: u32,
        principle_paid: u128,
        collateral_received: u128,
        collateral_paid: u128,
    );

    fn get_loan(env: Env, loan_id: u32) -> Loan;
}
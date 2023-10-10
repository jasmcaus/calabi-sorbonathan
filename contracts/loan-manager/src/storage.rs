#![allow(unused)]
use soroban_sdk::{Address, Env, contracttype, contract, contractimpl};

#[contracttype]
pub enum LoanState {
    ACTIVE,
    REPAID,
    ACTIVEDEFAULTED,
    REPAIDDEFAULTED
}


#[contracttype]
pub struct Loan {
    pub exists: bool,

    pub offer_id: u32,
    pub state: LoanState,

    pub borrower: Address,
    pub lender: Address,

    pub principal_asset: Address,
    pub collateral_asset: Address,

    pub initial_principal: u128,
    pub current_principal: u128,
    pub initial_collateral: u128,
    pub current_collateral: u128,

    // worth of collateral in USD
    pub collateral_price: u128, 

    // loan interest_rate rate per seconds
    pub interest_rate: u32,
    pub start_date: u128,
    pub maturity_date: u128,

    pub num_installments_paid: u32,

    // represents principal + interest_rate was paid payback by the borrower tha lender has not claimed
    pub unclaimed_principal: u128,
    // represents collateral amount was unlocked that the borrower has noted
    pub unclaimed_collateral: u128,
    // represents collateral amount retrieved from a borrower when default that the lender has not claimed
    pub unclaimed_default_collateral: u128,
    // represents principal amount the borrower has not claimed
    pub unclaimed_borrowed_principal: u128,
    // represents total interest_rate paid by borrower
    pub total_interest_paid: u128,
    // seconds of full/installment repaid loan
    pub repaid_on: u128,
}

#[derive(Clone)]
#[repr(u32)]
#[contracttype]
enum StorageKey {
    LoanId,
    Loans(u32),
}


pub fn __get_loan_id(env: &Env) -> u32 {
    let key = StorageKey::LoanId;

    if let Some(id) = env.storage().persistent().get(&key) {
        id 
    } else {
        0
    }
}

pub fn __increment_loan_id(env: &Env) {
    let key = StorageKey::LoanId;

    let loan_id = __get_loan_id(&env);

    env.storage().persistent().set(&key, &(loan_id + 1));
}


pub fn __get_loan(env: &Env, loan_id: u32) -> Loan {
    let key = StorageKey::Loans(loan_id);

    env.storage().persistent().get(&key).unwrap()
}

pub fn __set_loan(env: &Env, loan_id: u32, loan: Loan) {
    let key = StorageKey::Loans(loan_id);

    env.storage().persistent().set(&key, &loan);
}
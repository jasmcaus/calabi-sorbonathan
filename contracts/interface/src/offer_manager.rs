#![allow(unused)]
use soroban_sdk::{contract, contractimpl, contracttype, contractclient, Address, Env};


#[derive(Clone, Eq, PartialEq)]
#[contracttype]
pub enum OfferType {
    LENDINGOFFER,
    BORROWINGOFFER,
}

#[derive(Clone, Eq, PartialEq)]
#[contracttype]
pub enum OfferState {
    DEFAULT,
    CANCELLED,
}


#[derive(Clone)]
#[contracttype]
pub struct Offer {
    pub exists: bool,

    pub offer_id: u32,
    pub state: OfferState,
    pub offer_type: OfferType,

    pub principle_asset: Address,
    pub collateral_asset: Address,

    pub initial_principle: u128,
    pub current_principle: u128,

    pub initial_collateral: u128,
    pub current_collateral: u128,

    // loan interest_rate rate per seconds
    pub interest_rate: u32,
    pub days_to_maturity: u32,
    pub expiration_date: u128,
    pub created_at: u128,

    pub creator: Address,
}


#[contractclient(name = "OfferManager")]
pub trait IOfferManager {
    fn create_offer(
        env: Env,
        principle_asset: Address,
        principle_amount: u128,
        interest_rate: u32,
        days_to_maturity: u32,
        days_to_expire: u32,
        collateral_asset: Address,
        lender: Address,
    ) -> u32;

    // fn create_loan(
    //     env: Env,
    //     is_lending_offer: bool, // if false, it's borrowing offer
    //     borrower: Address,
    //     lender: Address,
    //     principle_asset: Address,
    //     collateral_asset: Address,
    //     collateral_amount: u128,
    //     principle_amount: u128,
    //     collateral_price: u128,
    //     interest_rate: u32,
    //     unclaimed_borrowed_principle: u128,
    // ) -> u32;

    // fn repay_loan(
    //     env: Env,
    //     loan_id: u32,
    //     interest_paid: u128,
    //     principle_paid: u128,
    //     collateral_received: u128,
    // );

    // fn claim_principle(env: Env, loan_id: u32, user: Address) -> (u128, u32);

    // fn claim_collateral(env: Env, loan_id: u32, user: Address) -> (u128, u32);

    // fn claim_borrowed_principle(env: Env, loan_id: u32, user: Address) -> (u128, u32);

    // fn liquidate_loan(
    //     env: Env,
    //     loan_id: u32,
    //     principle_paid: u128,
    //     collateral_received: u128,
    //     collateral_paid: u128,
    // );

    // fn get_loan(env: Env, loan_id: u32) -> Offer;
}
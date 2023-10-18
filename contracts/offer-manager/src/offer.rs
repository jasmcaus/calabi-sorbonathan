#![allow(unused)]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

use crate::assertions::*;
use crate::storage::*;
use interface::offer_manager::*;

const DUST_AMOUNT: u128 = 100;

#[contract]
pub struct OfferManager;

#[contractimpl]
impl IOfferManager for OfferManager {
    fn create_offer(
        env: Env,
        principle_asset: Address,
        principle_amount: u128,
        interest_rate: u32,
        days_to_maturity: u32,
        days_to_expire: u32,
        collateral_asset: Address,
        lender: Address,
    ) -> u32 {
        lender.require_auth();
        __increment_offer_id(&env);
        let offer_id = __get_offer_id(&env);

        let created_at = env.ledger().timestamp() as u128;
        let duration = (days_to_expire + 1 * (24 * 60 * 60)) as u64;
        let expires_at = created_at + duration as u128;

        let offer = Offer {
            exists: true,
            offer_id,
            state: OfferState::DEFAULT,
            offer_type: OfferType::LENDINGOFFER,
            principle_asset,
            collateral_asset,
            initial_principle: principle_amount,
            current_principle: principle_amount,
            initial_collateral: 0,
            current_collateral: 0,
            created_at,
            interest_rate,
            days_to_maturity,
            expiration_date: expires_at,
            creator: lender,
        };

        return offer_id;
    }

    //     require(lender != borrower, "Invalid lender/borrower");

    //     if is_lending_offer {
    //         require(
    //             !__has_borrowed(&env, offer_id, borrower.clone()),
    //             "Err: already borrowed",
    //         );
    //         __set_has_borrowed(&env, offer_id, borrower.clone(), true)
    //     } else {
    //         require(
    //             !__has_borrowed(&env, offer_id, lender.clone()),
    //             "Err: already borrowed",
    //         );
    //         __set_has_borrowed(&env, offer_id, lender.clone(), true)
    //     }

    //     let now = env.ledger().timestamp();
    //     let duration = (24 * 60 * 60) * 1;
    //     let maturity_date = now + duration;

    //     let offer = Loan {
    //         exists: true,
    //         offer_id: offer_id,
    //         state: LoanState::ACTIVE,
    //         borrower,
    //         lender,
    //         principle_asset,
    //         collateral_asset,
    //         initial_principle: principle_amount,
    //         current_principle: principle_amount,
    //         initial_collateral: collateral_amount,
    //         current_collateral: collateral_amount,
    //         collateral_price,
    //         interest_rate,
    //         start_date: now as u128,
    //         maturity_date: maturity_date as u128,

    //         num_installments_paid: 0,

    //         unclaimed_principle: 0,
    //         unclaimed_collateral: 0,
    //         unclaimed_default_collateral: 0,
    //         unclaimed_borrowed_principle: 0,
    //         total_interest_paid: 0,
    //         repaid_on: 0,
    //     };

    //     __set_offer(&env, offer_id, offer);

    //     offer_id
    // }

    // fn repay_offer(
    //     env: Env,
    //     offer_id: u32,
    //     interest_paid: u128,
    //     principle_paid: u128,
    //     collateral_received: u128,
    // ) {
    //     let mut offer = __get_offer(&env, offer_id);
    //     require(offer.exists, "Loan doesn't exist lol");

    //     offer.num_installments_paid += 1;
    //     offer.total_interest_paid += interest_paid;
    //     offer.unclaimed_principle += principle_paid + interest_paid;
    //     offer.unclaimed_collateral += collateral_received;
    //     offer.current_principle -= principle_paid;
    //     offer.current_collateral -= collateral_received;

    //     if offer.current_principle <= DUST_AMOUNT {
    //         offer.state = LoanState::REPAID;
    //     }

    //     __set_offer(&env, offer_id, offer);
    // }

    // fn claim_principle(env: Env, offer_id: u32, user: Address) -> (u128, u32) {
    //     let mut offer = __get_offer(&env, offer_id);
    //     require(offer.exists, "Loan doesn't exist lol");

    //     require(offer.lender == user, "Err: not lender");
    //     require(offer.unclaimed_principle > 0, "Err: zero balance");

    //     let amount = offer.unclaimed_principle;
    //     let offer_id = offer.offer_id;
    //     offer.unclaimed_principle = 0;

    //     __set_offer(&env, offer_id, offer.clone());

    //     (amount, offer_id)
    // }

    // fn claim_collateral(env: Env, offer_id: u32, user: Address) -> (u128, u32) {
    //     let mut offer = __get_offer(&env, offer_id);
    //     require(offer.exists, "Loan doesn't exist lol");

    //     require(offer.borrower == user, "Err: not borrower");
    //     require(offer.unclaimed_collateral > 0, "Err: zero balance");

    //     let amount = offer.unclaimed_collateral;
    //     let offer_id = offer.offer_id;
    //     offer.unclaimed_collateral = 0;

    //     __set_offer(&env, offer_id, offer.clone());

    //     (amount, offer_id)
    // }

    // fn claim_borrowed_principle(env: Env, offer_id: u32, user: Address) -> (u128, u32) {
    //     let mut offer = __get_offer(&env, offer_id);
    //     require(offer.exists, "Loan doesn't exist lol");

    //     require(offer.borrower == user, "Err: not borrower");
    //     require(offer.unclaimed_borrowed_principle > 0, "Err: zero balance");

    //     let amount = offer.unclaimed_borrowed_principle;
    //     let offer_id = offer.offer_id;
    //     offer.unclaimed_borrowed_principle = 0;

    //     __set_offer(&env, offer_id, offer.clone());

    //     (amount, offer_id)
    // }

    // fn liquidate_offer(
    //     env: Env,
    //     offer_id: u32,
    //     principle_paid: u128,
    //     collateral_received: u128,
    //     collateral_paid: u128,
    // ) {
    //     let mut offer = __get_offer(&env, offer_id);
    //     require(offer.state == LoanState::ACTIVE, "Loan not active");

    //     let now = env.ledger().timestamp() as u128;
    //     let default_date = offer.maturity_date * 1;

    //     require(default_date >= now, "Err: offer not matured");

    //     offer.current_principle -= principle_paid;
    //     offer.current_collateral -= collateral_received;
    //     offer.unclaimed_default_collateral -= collateral_paid;

    //     if offer.current_collateral > 0 {
    //         offer.unclaimed_collateral = offer.current_collateral;
    //         offer.current_collateral = 0;
    //     }

    //     if offer.current_principle <= 10 {
    //         offer.state = LoanState::REPAIDDEFAULTED;
    //     } else {
    //         offer.state = LoanState::ACTIVEDEFAULTED;
    //     }
    // }

    // fn get_offer(env: Env, offer_id: u32) -> Offer {
    //     __get_offer(&env, offer_id)
    // }
}

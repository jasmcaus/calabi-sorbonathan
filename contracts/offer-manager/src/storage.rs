#![allow(unused)]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};
use interface::offer_manager::{Offer, OfferState, OfferType};

#[derive(Clone)]
#[repr(u32)]
#[contracttype]
enum StorageKey {
    OfferId,
    Offers(u32),
}


pub fn __get_offer_id(env: &Env) -> u32 {
    let key = StorageKey::OfferId;

    if let Some(id) = env.storage().persistent().get(&key) {
        id
    } else {
        0
    }
}

pub fn __increment_offer_id(env: &Env) {
    let key = StorageKey::OfferId;

    let offer_id = __get_offer_id(&env);

    env.storage().persistent().set(&key, &(offer_id + 1));
}

pub fn __get_offer(env: &Env, offer_id: u32) -> Offer {
    let key = StorageKey::Offers(offer_id);

    if let Some(loan) = env.storage().persistent().get(&key) {
        loan
    } else {
        panic!("Offer doesn't exist");
    }
}

pub fn __set_offer(env: &Env, offer_id: u32, loan: Offer) {
    let key = StorageKey::Offers(offer_id);

    env.storage().persistent().set(&key, &loan);
}
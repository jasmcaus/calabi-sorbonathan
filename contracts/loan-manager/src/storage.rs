#![allow(unused)]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};
use interface::loan_manager::{Loan, LoanState};

#[derive(Clone)]
#[contracttype]
pub struct HasBorrowedStorageKey {
    offer_id: u32,
    user: Address,
}

#[derive(Clone)]
#[repr(u32)]
#[contracttype]
enum StorageKey {
    LoanId,
    Loans(u32),
    HasBorrowed(HasBorrowedStorageKey),
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

    if let Some(loan) = env.storage().persistent().get(&key) {
        loan
    } else {
        panic!("Loan doesn't exist");
    }
}

pub fn __set_loan(env: &Env, loan_id: u32, loan: Loan) {
    let key = StorageKey::Loans(loan_id);

    env.storage().persistent().set(&key, &loan);
}

pub fn __has_borrowed(env: &Env, offer_id: u32, user: Address) -> bool {
    let key = StorageKey::HasBorrowed(HasBorrowedStorageKey { offer_id, user });

    if let Some(has_borrowed) = env.storage().persistent().get(&key) {
        has_borrowed
    } else {
        false
    }
}

pub fn __set_has_borrowed(env: &Env, offer_id: u32, user: Address, has_borrowed: bool) {
    let key = StorageKey::HasBorrowed(HasBorrowedStorageKey { offer_id, user });

    env.storage().persistent().set(&key, &has_borrowed);
}

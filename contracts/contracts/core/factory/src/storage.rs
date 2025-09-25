use soroban_sdk::{Address, Env, Vec, Symbol, symbol_short};

const ESCROW_COUNT: Symbol = symbol_short!("ESC_CNT");
const ESCROW_LIST: Symbol = symbol_short!("ESC_LIST");

pub fn add_escrow_to_storage(env: &Env, escrow_address: Address) {
    // Add escrow to index
    let mut escrows: Vec<Address> = env.storage().instance().get(&ESCROW_LIST).unwrap_or(Vec::new(env));
    escrows.push_back(escrow_address);
    env.storage().instance().set(&ESCROW_LIST, &escrows);

    // Update count
    let count = escrows.len();
    env.storage().instance().set(&ESCROW_COUNT, &count);
}

pub fn get_escrow_count(env: &Env) -> u32 {
    env.storage().instance().get(&ESCROW_COUNT).unwrap_or(0)
}

pub fn get_all_escrows(env: &Env) -> Vec<Address> {
    env.storage().instance().get(&ESCROW_LIST).unwrap_or(Vec::new(env))
}

pub fn get_escrow_by_index(env: &Env, index: u32) -> Option<Address> {
    let escrows: Vec<Address> = env.storage().instance().get(&ESCROW_LIST).unwrap_or(Vec::new(env));
    if index < escrows.len() {
        Some(escrows.get(index).unwrap())
    } else {
        None
    }
}
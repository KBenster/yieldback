#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use crate::token;

#[contract]
pub struct PrincipalToken;

#[contractimpl]
impl PrincipalToken {
    pub fn initialize(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
        escrow_contract: Address,
        maturity_date: u64,
    ) {
        token::initialize(&env, admin, decimal, name, symbol, escrow_contract, maturity_date);
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        token::allowance(&env, from, spender)
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        token::approve(&env, from, spender, amount, expiration_ledger);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        token::balance(&env, id)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        token::transfer(&env, from, to, amount);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        token::transfer_from(&env, spender, from, to, amount);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        token::burn(&env, from, amount);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        token::burn_from(&env, spender, from, amount);
    }

    pub fn decimals(env: Env) -> u32 {
        token::decimals(&env)
    }

    pub fn name(env: Env) -> String {
        token::name(&env)
    }

    pub fn symbol(env: Env) -> String {
        token::symbol(&env)
    }

    pub fn total_supply(env: Env) -> i128 {
        token::total_supply(&env)
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        token::mint(&env, to, amount);
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        token::set_admin(&env, new_admin);
    }

    pub fn admin(env: Env) -> Address {
        token::admin(&env)
    }

    pub fn authorized(env: Env, id: Address) -> bool {
        token::authorized(&env, id)
    }

    pub fn set_authorized(env: Env, id: Address, authorize: bool) {
        token::set_authorized(&env, id, authorize);
    }

    pub fn clawback(env: Env, from: Address, amount: i128) {
        token::clawback(&env, from, amount);
    }

    // Bond-specific functions
    pub fn get_escrow_contract(env: Env) -> Address {
        token::get_escrow_contract(&env)
    }

    pub fn get_maturity_date(env: Env) -> u64 {
        token::get_maturity_date(&env)
    }
}
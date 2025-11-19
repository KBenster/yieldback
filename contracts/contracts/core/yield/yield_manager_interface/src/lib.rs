#![no_std]

use soroban_sdk::{contractclient, Address, BytesN, Env};

/// Trait defining the interface for the Yield Manager contract.
/// This trait is used to generate the YieldManagerClient for type-safe cross-contract calls.
#[contractclient(name = "YieldManagerClient")]
pub trait YieldManagerTrait {
    fn __constructor(
        env: Env,
        admin: Address,
        vault: Address,
        maturity: u64,
        pt_wasm_hash: BytesN<32>,
        yt_wasm_hash: BytesN<32>,
    );

    fn get_vault(env: Env) -> Address;
    fn get_principal_token(env: Env) -> Address;
    fn get_yield_token(env: Env) -> Address;
    fn get_maturity(env: Env) -> u64;
    fn get_exchange_rate(env: Env) -> i128;
    fn get_exchange_rate_at_expiry(env: Env) -> Option<i128>;
    fn set_exchange_rate_at_expiry(env: Env);
    fn deposit(env: Env, from: Address, shares_amount: i128);
    fn distribute_yield(env: Env, to: Address, shares_amount: i128);
    fn redeem_principal(env: Env, from: Address, pt_amount: i128);
}

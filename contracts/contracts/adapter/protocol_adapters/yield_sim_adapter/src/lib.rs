#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, IntoVal};
use adapter_trait::YieldAdapter;

#[contracttype]
pub enum DataKey {
    YieldPool,
    Token,
}

#[contract]
pub struct YieldSimAdapter;

#[contractimpl]
impl YieldAdapter for YieldSimAdapter {
    fn __constructor(env: Env, yield_protocol: Address, token: Address) {
        if env.storage().instance().has(&DataKey::YieldPool) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::YieldPool, &yield_protocol);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    fn deposit(env: Env, depositor: Address, amount: i128) {
        depositor.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let yield_pool_address: Address = env.storage().instance()
            .get(&DataKey::YieldPool)
            .expect("Not initialized");

        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from depositor (escrow) to adapter
        token.transfer(&depositor, &env.current_contract_address(), &amount);

        // Approve the yield pool to spend tokens
        token.approve(&env.current_contract_address(), &yield_pool_address, &amount, &(env.ledger().sequence() + 1000));

        // Forward the deposit to the yield pool
        env.invoke_contract::<()>(
            &yield_pool_address,
            &soroban_sdk::symbol_short!("deposit"),
            (env.current_contract_address(), amount).into_val(&env)
        );
    }

    fn get_yield_protocol(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::YieldPool)
            .expect("Not initialized")
    }

    fn get_token(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized")
    }
}

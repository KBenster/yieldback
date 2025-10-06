#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, IntoVal};
use adapter_trait::YieldAdapter;

#[contracttype]
pub enum DataKey {
    Escrow,
    YieldPool,
    Token,
}

#[contract]
pub struct YieldSimAdapter;

#[contractimpl]
impl YieldAdapter for YieldSimAdapter {
    fn __constructor(env: Env, escrow: Address, yield_protocol: Address, token: Address) {
        if env.storage().instance().has(&DataKey::YieldPool) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Escrow, &escrow);
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
        //TODO: this may not be necessary
        token.approve(&env.current_contract_address(), &yield_pool_address, &amount, &(env.ledger().sequence() + 1000));

        // Forward the deposit to the yield pool
        env.invoke_contract::<()>(
            &yield_pool_address,
            &soroban_sdk::symbol_short!("deposit"),
            (env.current_contract_address(), amount).into_val(&env)
        );
    }

    fn withdraw(env: Env, amount: i128) {
        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let yield_pool_address: Address = env.storage().instance()
            .get(&DataKey::YieldPool)
            .expect("Not initialized");

        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let escrow_address: Address = env.storage().instance()
            .get(&DataKey::Escrow)
            .expect("Not initialized");

        let adapter_address = env.current_contract_address();

        // Withdraw from the yield pool to the adapter
        env.invoke_contract::<Address>(
            &yield_pool_address,
            &soroban_sdk::symbol_short!("withdraw"),
            (adapter_address.clone(), amount).into_val(&env)
        );

        // Transfer tokens from adapter to the escrow contract
        let token = token::Client::new(&env, &token_address);
        token.transfer(&adapter_address, &escrow_address, &amount);
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

    fn get_assets(env: Env) -> i128 {
        let yield_pool_address: Address = env.storage().instance()
            .get(&DataKey::YieldPool)
            .expect("Not initialized");

        // Query the balance of this adapter contract in the yield pool
        env.invoke_contract::<i128>(
            &yield_pool_address,
            &soroban_sdk::symbol_short!("balance"),
            (env.current_contract_address(),).into_val(&env)
        )
    }
}

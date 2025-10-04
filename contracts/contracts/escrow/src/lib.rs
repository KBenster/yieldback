#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, IntoVal, String, BytesN};

#[contracttype]
pub enum DataKey {
    BlendPool,
    Token,
    SYToken,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn __constructor(env: Env, blend_pool: Address, token: Address, sy_wasm_hash: BytesN<32>, admin: Address, name: String, symbol: String) {
        env.storage().instance().set(&DataKey::BlendPool, &blend_pool);
        env.storage().instance().set(&DataKey::Token, &token);

        // Deploy StandardizedYield token contract
        let sy_token_address = env.deployer().with_current_contract(b"sy_token")
            .deploy_v2(sy_wasm_hash, (admin.clone(), name, symbol).into_val(&env));

        env.storage().instance().set(&DataKey::SYToken, &sy_token_address);
    }

    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let blend_pool_address: Address = env.storage().instance()
            .get(&DataKey::BlendPool)
            .expect("Not initialized");

        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from user to escrow contract
        token.transfer(&user, &env.current_contract_address(), &amount);

        // Transfer tokens from escrow to blend pool
        token.transfer(&env.current_contract_address(), &blend_pool_address, &amount);

        // Call blend pool's deposit function
        env.invoke_contract::<()>(
            &blend_pool_address,
            &soroban_sdk::symbol_short!("deposit"),
            (user, amount).into_val(&env)
        );
    }
}
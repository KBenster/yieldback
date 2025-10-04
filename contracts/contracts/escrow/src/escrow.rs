use soroban_sdk::{contract, contractimpl, contracttype, token, Address, BytesN, Env, IntoVal};
use crate::token_deployment;

#[contracttype]
pub enum DataKey {
    BlendPool,
    Token,
    SYToken,
    SYWasmHash,
    MaturityDate,
}

pub trait Escrow {
    fn deploy_sy_token(env: Env, admin: Address, name: soroban_sdk::String, symbol: soroban_sdk::String) -> Address;
    fn deposit(env: Env, user: Address, amount: i128);
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn __constructor(env: Env, blend_pool: Address, token: Address, sy_wasm_hash: BytesN<32>, maturity_date: u64) {
        env.storage().instance().set(&DataKey::BlendPool, &blend_pool);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::SYWasmHash, &sy_wasm_hash);
        env.storage().instance().set(&DataKey::MaturityDate, &maturity_date);
    }
}

#[contractimpl]
impl Escrow for EscrowContract {
    fn deploy_sy_token(env: Env, admin: Address, name: soroban_sdk::String, symbol: soroban_sdk::String) -> Address {
        token_deployment::deploy_sy_token(&env, admin, name, symbol)
    }

    fn deposit(env: Env, user: Address, amount: i128) {
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

        let sy_token_address: Address = env.storage().instance()
            .get(&DataKey::SYToken)
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
            (user.clone(), amount).into_val(&env)
        );

        // syAmount = assetAmount / exchangeRate
        let exchange_rate: i128 = 1; // At the start, 1 SY = 1 asset. As yield accrues, the exchange rate increases proportionally.
        let sy_amount = amount / exchange_rate;

        // Mint SY tokens to the escrow contract
        env.invoke_contract::<()>(
            &sy_token_address,
            &soroban_sdk::symbol_short!("mint"),
            (env.current_contract_address(), sy_amount).into_val(&env)
        );
    }
}

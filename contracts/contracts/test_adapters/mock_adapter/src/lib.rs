#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    Escrow,
    YieldProtocol,
    Token,
    TotalAssets,
}

/// Mock adapter contract for testing
/// Implements the YieldAdapter trait but doesn't actually interact with a real yield protocol
/// Instead, it just tracks assets and allows simulating yield for testing purposes
#[contract]
pub struct MockAdapter;

#[contractimpl]
impl MockAdapter {
    /// Initialize the adapter with the escrow contract, yield protocol address and token address
    pub fn __constructor(env: Env, escrow: Address, yield_protocol: Address, token: Address) {
        env.storage().instance().set(&DataKey::Escrow, &escrow);
        env.storage().instance().set(&DataKey::YieldProtocol, &yield_protocol);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::TotalAssets, &0i128);
    }

    /// Receive funds from escrow and track them as assets
    pub fn deposit(env: Env, depositor: Address, amount: i128) {
        depositor.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from depositor to adapter
        token.transfer(&depositor, &env.current_contract_address(), &amount);

        // Update total assets
        let current_assets: i128 = env.storage().instance().get(&DataKey::TotalAssets).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalAssets, &(current_assets + amount));
    }

    /// Withdraw funds from the adapter to the escrow contract
    pub fn withdraw(env: Env, amount: i128) {
        let escrow: Address = env.storage().instance().get(&DataKey::Escrow).unwrap();
        escrow.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from adapter to escrow
        token.transfer(&env.current_contract_address(), &escrow, &amount);

        // Update total assets
        let current_assets: i128 = env.storage().instance().get(&DataKey::TotalAssets).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalAssets, &(current_assets - amount));
    }

    /// Get the configured yield protocol address
    pub fn get_yield_protocol(env: Env) -> Address {
        env.storage().instance().get(&DataKey::YieldProtocol).unwrap()
    }

    /// Get the configured token address
    pub fn get_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Token).unwrap()
    }

    /// Get the total assets managed by this adapter
    pub fn get_assets(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::TotalAssets).unwrap_or(0)
    }

    // ============ Test Helper Functions ============
    // These functions are only used in tests to simulate yield generation

    /// Manually set the total assets (for testing purposes)
    pub fn set_total_assets(env: Env, amount: i128) {
        env.storage().instance().set(&DataKey::TotalAssets, &amount);
    }

    /// Simulate yield generation by increasing total assets without transferring tokens
    /// This mimics what would happen if the underlying yield protocol generated returns
    pub fn simulate_yield(env: Env, yield_amount: i128) {
        let current_assets: i128 = env.storage().instance().get(&DataKey::TotalAssets).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalAssets, &(current_assets + yield_amount));
    }
}
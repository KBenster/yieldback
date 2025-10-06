#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    Token,
    YieldRate,      // Annual percentage yield in basis points (e.g., 500 = 5%)
    LastAccrual,    // Last time yield was accrued
}

#[contracttype]
#[derive(Clone)]
pub struct Position {
    pub principal: i128,
    pub last_update: u64,
}

/// Mock yield protocol that simulates a lending/staking protocol
/// Accepts deposits, tracks positions, and pays out yield based on a configurable APY
#[contract]
pub struct MockYieldProtocol;

#[contractimpl]
impl MockYieldProtocol {
    /// Initialize the protocol with a token address and annual yield rate
    ///
    /// # Arguments
    /// * `token` - The token address this protocol accepts
    /// * `annual_yield_bps` - Annual yield in basis points (e.g., 500 = 5% APY)
    /// * `funder` - Address that will provide initial funding
    /// * `initial_reserve` - Amount of tokens to transfer from funder as initial reserves
    pub fn __constructor(env: Env, token: Address, annual_yield_bps: u32, funder: Address, initial_reserve: i128) {
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::YieldRate, &annual_yield_bps);
        env.storage().instance().set(&DataKey::LastAccrual, &env.ledger().timestamp());

        // Fund the protocol with initial reserves if amount > 0
        if initial_reserve > 0 {
            funder.require_auth();
            let token_client = token::Client::new(&env, &token);
            token_client.transfer(&funder, &env.current_contract_address(), &initial_reserve);
        }
    }

    /// Deposit tokens into the yield protocol
    /// Creates or updates a position for the depositor
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from depositor to protocol
        token.transfer(&from, &env.current_contract_address(), &amount);

        // Get or create position
        let mut position: Position = env
            .storage()
            .persistent()
            .get(&from)
            .unwrap_or(Position {
                principal: 0,
                last_update: env.ledger().timestamp(),
            });

        // Accrue any pending yield before updating
        let accrued = Self::calculate_yield(
            env.clone(),
            position.principal,
            position.last_update,
            env.ledger().timestamp(),
        );

        // Update position
        position.principal = position.principal + amount + accrued;
        position.last_update = env.ledger().timestamp();

        env.storage().persistent().set(&from, &position);
    }

    /// Withdraw tokens from the protocol (including accrued yield)
    pub fn withdraw(env: Env, to: Address, amount: i128) {
        to.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Get position
        let mut position: Position = env
            .storage()
            .persistent()
            .get(&to)
            .unwrap_or_else(|| panic!("no position found"));

        // Calculate accrued yield
        let accrued = Self::calculate_yield(
            env.clone(),
            position.principal,
            position.last_update,
            env.ledger().timestamp(),
        );

        let total_balance = position.principal + accrued;

        if total_balance < amount {
            panic!("insufficient balance");
        }

        // Transfer tokens to user
        token.transfer(&env.current_contract_address(), &to, &amount);

        // Update position
        position.principal = total_balance - amount;
        position.last_update = env.ledger().timestamp();

        if position.principal > 0 {
            env.storage().persistent().set(&to, &position);
        } else {
            env.storage().persistent().remove(&to);
        }
    }

    /// Get the balance of a depositor (principal + accrued yield)
    pub fn balance(env: Env, account: Address) -> i128 {
        let position: Position = env
            .storage()
            .persistent()
            .get(&account)
            .unwrap_or(Position {
                principal: 0,
                last_update: env.ledger().timestamp(),
            });

        let accrued = Self::calculate_yield(
            env.clone(),
            position.principal,
            position.last_update,
            env.ledger().timestamp(),
        );

        position.principal + accrued
    }

    /// Get just the principal (original deposit amount, excluding yield)
    pub fn principal(env: Env, account: Address) -> i128 {
        let position: Position = env
            .storage()
            .persistent()
            .get(&account)
            .unwrap_or(Position {
                principal: 0,
                last_update: env.ledger().timestamp(),
            });

        position.principal
    }

    /// Get the accrued yield for an account
    pub fn accrued_yield(env: Env, account: Address) -> i128 {
        let position: Position = env
            .storage()
            .persistent()
            .get(&account)
            .unwrap_or(Position {
                principal: 0,
                last_update: env.ledger().timestamp(),
            });

        Self::calculate_yield(
            env.clone(),
            position.principal,
            position.last_update,
            env.ledger().timestamp(),
        )
    }

    /// Get the token address
    pub fn get_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Token).unwrap()
    }

    /// Get the annual yield rate in basis points
    pub fn get_yield_rate(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::YieldRate).unwrap()
    }

    /// Calculate yield for a given principal over a time period
    /// Uses simple interest: yield = principal * rate * time / (365 days * 10000)
    fn calculate_yield(env: Env, principal: i128, start_time: u64, end_time: u64) -> i128 {
        if principal == 0 || end_time <= start_time {
            return 0;
        }

        let rate: u32 = env.storage().instance().get(&DataKey::YieldRate).unwrap_or(0);
        let time_elapsed = (end_time - start_time) as i128;
        let seconds_per_year: i128 = 365 * 24 * 60 * 60;

        // yield = principal * rate * time / (seconds_per_year * 10000)
        // Note: 10000 converts basis points to percentage
        let yield_amount = (principal * (rate as i128) * time_elapsed)
            / (seconds_per_year * 10000);

        yield_amount
    }

    // ============ Admin/Test Helper Functions ============

    /// Update the yield rate (for testing different scenarios)
    pub fn set_yield_rate(env: Env, new_rate_bps: u32) {
        env.storage().instance().set(&DataKey::YieldRate, &new_rate_bps);
    }

    /// Get the total value locked in the protocol
    pub fn total_assets(env: Env) -> i128 {
        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token = token::Client::new(&env, &token_address);
        token.balance(&env.current_contract_address())
    }

    /// Fund the protocol with additional tokens (to ensure it can pay yield)
    /// This simulates the protocol having reserves or earning external revenue
    pub fn fund_protocol(env: Env, funder: Address, amount: i128) {
        funder.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token = token::Client::new(&env, &token_address);

        token.transfer(&funder, &env.current_contract_address(), &amount);
    }
}

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone)]
pub struct Deposit {
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Token,
    InterestRate, // Annual interest rate in basis points (e.g., 500 = 5%)
    Deposit(Address),
}

#[contract]
pub struct BlendPoolSimulator;

#[contractimpl]
impl BlendPoolSimulator {
    /// Initialize the pool with a token address and annual interest rate
    /// interest_rate_bps: interest rate in basis points (e.g., 500 = 5% APY)
    /// Note: The pool contract should be set as the admin of the token to mint interest
    pub fn __constructor(env: Env, token: Address, interest_rate_bps: u32) {
        if env.storage().instance().has(&DataKey::Token) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::InterestRate, &interest_rate_bps);
    }

    /// Deposit funds into the pool
    pub fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from user to contract
        token.transfer(&user, &env.current_contract_address(), &amount);

        // Get existing deposit or create new one
        let deposit_key = DataKey::Deposit(user.clone());
        let current_balance = Self::balance(env.clone(), user.clone());

        // Store new deposit with current timestamp
        let new_deposit = Deposit {
            amount: current_balance + amount,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().persistent().set(&deposit_key, &new_deposit);
        env.storage().persistent().extend_ttl(&deposit_key, 5184000, 5184000); // ~60 days
    }

    /// Withdraw funds including accrued interest
    pub fn withdraw(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let current_balance = Self::balance(env.clone(), user.clone());

        if amount > current_balance {
            panic!("Insufficient balance");
        }

        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let token = token::Client::new(&env, &token_address);
        let token_admin = token::StellarAssetClient::new(&env, &token_address);

        // Calculate interest earned
        let deposit_key = DataKey::Deposit(user.clone());
        let deposit = env.storage().persistent()
            .get::<DataKey, Deposit>(&deposit_key)
            .expect("No deposit found");

        let interest_earned = current_balance - deposit.amount;

        // Mint interest tokens to the pool if needed
        if interest_earned > 0 {
            token_admin.mint(&env.current_contract_address(), &interest_earned);
        }

        // Transfer tokens from contract to user
        token.transfer(&env.current_contract_address(), &user, &amount);

        let remaining_balance = current_balance - amount;

        if remaining_balance > 0 {
            // Update deposit with remaining balance
            let new_deposit = Deposit {
                amount: remaining_balance,
                timestamp: env.ledger().timestamp(),
            };
            env.storage().persistent().set(&deposit_key, &new_deposit);
            env.storage().persistent().extend_ttl(&deposit_key, 5184000, 5184000);
        } else {
            // Remove deposit if fully withdrawn
            env.storage().persistent().remove(&deposit_key);
        }
    }

    /// Get current balance including accrued interest
    pub fn balance(env: Env, user: Address) -> i128 {
        let deposit_key = DataKey::Deposit(user);

        if let Some(deposit) = env.storage().persistent().get::<DataKey, Deposit>(&deposit_key) {
            let interest_rate_bps: u32 = env.storage().instance()
                .get(&DataKey::InterestRate)
                .expect("Not initialized");

            let current_time = env.ledger().timestamp();
            let time_elapsed = current_time.saturating_sub(deposit.timestamp);

            // Calculate interest: principal * rate * time / (365 days * 10000)
            // time is in seconds, so 365 days = 31536000 seconds
            let interest = (deposit.amount as i128)
                .saturating_mul(interest_rate_bps as i128)
                .saturating_mul(time_elapsed as i128)
                / (31536000i128 * 10000i128);

            deposit.amount.saturating_add(interest)
        } else {
            0
        }
    }


    /// Get the pool's token address
    pub fn get_token(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized")
    }

    /// Get the interest rate in basis points
    pub fn get_interest_rate(env: Env) -> u32 {
        env.storage().instance()
            .get(&DataKey::InterestRate)
            .expect("Not initialized")
    }

    /// THE FOLLOWING FUNCTIONS SHOULD BE ONLY USED FOR DATA VALIDATION!
    /// Get the original deposit amount without interest
    pub fn principal(env: Env, user: Address) -> i128 {
        let deposit_key = DataKey::Deposit(user);

        if let Some(deposit) = env.storage().persistent().get::<DataKey, Deposit>(&deposit_key) {
            deposit.amount
        } else {
            0
        }
    }

    /// Get accrued interest only
    pub fn accrued_interest(env: Env, user: Address) -> i128 {
        Self::balance(env.clone(), user.clone()) - Self::principal(env, user)
    }
}
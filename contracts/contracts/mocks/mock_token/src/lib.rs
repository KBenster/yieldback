#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    Admin,
    TotalSupply,
}

#[contracttype]
#[derive(Clone)]
pub struct Balance {
    pub amount: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct Allowance {
    pub amount: i128,
    pub expiration: u32,
}

#[contracttype]
#[derive(Clone)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

/// Simplified mock token for testing and development
/// This implementation provides basic token functionality
#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    /// Initialize the token with admin, name, symbol, and decimals
    pub fn __constructor(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
    ) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);

        let info = TokenInfo {
            name,
            symbol,
            decimals,
        };
        env.storage().instance().set(&"info", &info);
    }

    /// Mint tokens to an address (no auth check for testing convenience)
    pub fn mint(env: Env, recipient: Address, amount: i128) {
        let current_balance = Self::balance(env.clone(), recipient.clone());
        env.storage()
            .persistent()
            .set(&recipient, &(current_balance + amount));

        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(supply + amount));
    }

    /// Burn tokens from an address (requires auth)
    pub fn burn(env: Env, holder: Address, amount: i128) {
        holder.require_auth();

        let current_balance = Self::balance(env.clone(), holder.clone());
        if current_balance < amount {
            panic!("insufficient balance to burn");
        }

        env.storage()
            .persistent()
            .set(&holder, &(current_balance - amount));

        let supply: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);
        env.storage()
            .instance()
            .set(&DataKey::TotalSupply, &(supply - amount));
    }

    /// Transfer tokens between addresses
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_bal = Self::balance(env.clone(), from.clone());
        let to_bal = Self::balance(env.clone(), to.clone());

        if from_bal < amount {
            panic!("insufficient balance for transfer");
        }

        env.storage().persistent().set(&from, &(from_bal - amount));
        env.storage().persistent().set(&to, &(to_bal + amount));
    }

    /// Transfer tokens from one address to another using allowance
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        // Check and update allowance
        let allowance_key = (from.clone(), spender.clone());
        let allowance: i128 = env.storage().persistent().get(&allowance_key).unwrap_or(0);

        if allowance < amount {
            panic!("insufficient allowance");
        }

        let from_bal = Self::balance(env.clone(), from.clone());
        let to_bal = Self::balance(env.clone(), to.clone());

        if from_bal < amount {
            panic!("insufficient balance");
        }

        // Update balances
        env.storage().persistent().set(&from, &(from_bal - amount));
        env.storage().persistent().set(&to, &(to_bal + amount));

        // Update allowance
        env.storage()
            .persistent()
            .set(&allowance_key, &(allowance - amount));
    }

    /// Approve a spender to transfer tokens on behalf of owner
    pub fn approve(
        env: Env,
        owner: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        owner.require_auth();

        let key = (owner, spender);
        env.storage().persistent().set(&key, &amount);

        // Store expiration (though not strictly enforced in this mock)
        let exp_key = (key.0.clone(), key.1.clone(), "exp");
        env.storage().persistent().set(&exp_key, &expiration_ledger);
    }

    /// Get balance of an address
    pub fn balance(env: Env, account: Address) -> i128 {
        env.storage().persistent().get(&account).unwrap_or(0)
    }

    /// Get allowance for a spender
    pub fn allowance(env: Env, owner: Address, spender: Address) -> i128 {
        let key = (owner, spender);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0)
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        let info: TokenInfo = env.storage().instance().get(&"info").unwrap();
        info.name
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        let info: TokenInfo = env.storage().instance().get(&"info").unwrap();
        info.symbol
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        let info: TokenInfo = env.storage().instance().get(&"info").unwrap();
        info.decimals
    }

    /// Get admin address
    pub fn admin(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    // ============ Test Helper Functions ============

    /// Set balance directly (for testing only)
    pub fn set_balance(env: Env, account: Address, amount: i128) {
        env.storage().persistent().set(&account, &amount);
    }

    /// Airdrop tokens to multiple addresses (for testing only)
    pub fn airdrop(env: Env, recipients: soroban_sdk::Vec<Address>, amount: i128) {
        for recipient in recipients.iter() {
            Self::mint(env.clone(), recipient, amount);
        }
    }
}

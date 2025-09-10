#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, contracttype,
    Address, Env, Symbol, String
};

pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const NAME: Symbol = symbol_short!("NAME");
pub const SYMBOL: Symbol = symbol_short!("SYMBOL");
pub const DECIMALS: Symbol = symbol_short!("DECIMALS");
pub const TOTAL_SUPPLY: Symbol = symbol_short!("TOT_SUPP");
pub const ESCROW_CONTRACT: Symbol = symbol_short!("ESCROW");
pub const MATURITY_DATE: Symbol = symbol_short!("MATURITY");
// New storage keys for distinguishing token types
pub const COUPON_SUPPLY: Symbol = symbol_short!("COUP_SUP");
pub const PRINCIPAL_SUPPLY: Symbol = symbol_short!("PRIN_SUP");

#[contracttype]
pub enum TokenType {
    Coupon = 0,
    Principal = 1,
}

#[contract]
pub struct ShareToken;

#[contractimpl]
impl ShareToken {
    /// Initialize the token
    pub fn __constructor(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
        decimals: u32,
        escrow_contract: Address,
        maturity_date: u64,
    ) {
        if env.storage().instance().has(&ADMIN) {
            panic!("Already initialized");
        }

        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&NAME, &name);
        env.storage().instance().set(&SYMBOL, &symbol);
        env.storage().instance().set(&DECIMALS, &decimals);
        env.storage().instance().set(&TOTAL_SUPPLY, &0i128);
        env.storage().instance().set(&ESCROW_CONTRACT, &escrow_contract);
        env.storage().instance().set(&MATURITY_DATE, &maturity_date);
        env.storage().instance().set(&COUPON_SUPPLY, &0i128);
        env.storage().instance().set(&PRINCIPAL_SUPPLY, &0i128);
    }

    /// Mint coupon tokens
    pub fn mint_coupon(env: Env, to: Address, amount: i128) {
        let escrow_contract: Address = env.storage().instance().get(&ESCROW_CONTRACT).unwrap();
        escrow_contract.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Update coupon balance
        let coupon_balance_key = (Symbol::new(&env, "coup_bal"), to.clone());
        let current_coupon_balance: i128 = env.storage()
            .persistent()
            .get(&coupon_balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&coupon_balance_key, &(current_coupon_balance + amount));

        // Update total balance
        let total_balance_key = (Symbol::new(&env, "balance"), to.clone());
        let current_total_balance: i128 = env.storage()
            .persistent()
            .get(&total_balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&total_balance_key, &(current_total_balance + amount));

        // Update supplies
        let coupon_supply: i128 = env.storage().instance().get(&COUPON_SUPPLY).unwrap();
        env.storage().instance().set(&COUPON_SUPPLY, &(coupon_supply + amount));

        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply + amount));

        env.events().publish(
            (Symbol::new(&env, "mint_coupon"), to),
            amount
        );
    }

    /// Mint principal tokens
    pub fn mint_principal(env: Env, to: Address, amount: i128) {
        let escrow_contract: Address = env.storage().instance().get(&ESCROW_CONTRACT).unwrap();
        escrow_contract.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Update principal balance
        let principal_balance_key = (Symbol::new(&env, "prin_bal"), to.clone());
        let current_principal_balance: i128 = env.storage()
            .persistent()
            .get(&principal_balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&principal_balance_key, &(current_principal_balance + amount));

        // Update total balance
        let total_balance_key = (Symbol::new(&env, "balance"), to.clone());
        let current_total_balance: i128 = env.storage()
            .persistent()
            .get(&total_balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&total_balance_key, &(current_total_balance + amount));

        // Update supplies
        let principal_supply: i128 = env.storage().instance().get(&PRINCIPAL_SUPPLY).unwrap();
        env.storage().instance().set(&PRINCIPAL_SUPPLY, &(principal_supply + amount));

        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply + amount));

        env.events().publish(
            (Symbol::new(&env, "mint_principal"), to),
            amount
        );
    }

    /// Burn coupon tokens
    pub fn burn_coupon(env: Env, from: Address, amount: i128) {
        let escrow_contract: Address = env.storage().instance().get(&ESCROW_CONTRACT).unwrap();
        escrow_contract.require_auth();
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let coupon_balance_key = (Symbol::new(&env, "coup_bal"), from.clone());
        let current_coupon_balance: i128 = env.storage()
            .persistent()
            .get(&coupon_balance_key)
            .unwrap_or(0);

        if current_coupon_balance < amount {
            panic!("Insufficient coupon balance");
        }

        // Update coupon balance
        env.storage()
            .persistent()
            .set(&coupon_balance_key, &(current_coupon_balance - amount));

        // Update total balance
        let total_balance_key = (Symbol::new(&env, "balance"), from.clone());
        let current_total_balance: i128 = env.storage()
            .persistent()
            .get(&total_balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&total_balance_key, &(current_total_balance - amount));

        // Update supplies
        let coupon_supply: i128 = env.storage().instance().get(&COUPON_SUPPLY).unwrap();
        env.storage().instance().set(&COUPON_SUPPLY, &(coupon_supply - amount));

        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply - amount));

        env.events().publish(
            (Symbol::new(&env, "burn_coupon"), from),
            amount
        );
    }

    /// Burn principal tokens
    pub fn burn_principal(env: Env, from: Address, amount: i128) {
        let escrow_contract: Address = env.storage().instance().get(&ESCROW_CONTRACT).unwrap();
        escrow_contract.require_auth();
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let principal_balance_key = (Symbol::new(&env, "prin_bal"), from.clone());
        let current_principal_balance: i128 = env.storage()
            .persistent()
            .get(&principal_balance_key)
            .unwrap_or(0);

        if current_principal_balance < amount {
            panic!("Insufficient principal balance");
        }

        // Update principal balance
        env.storage()
            .persistent()
            .set(&principal_balance_key, &(current_principal_balance - amount));

        // Update total balance
        let total_balance_key = (Symbol::new(&env, "balance"), from.clone());
        let current_total_balance: i128 = env.storage()
            .persistent()
            .get(&total_balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&total_balance_key, &(current_total_balance - amount));

        // Update supplies
        let principal_supply: i128 = env.storage().instance().get(&PRINCIPAL_SUPPLY).unwrap();
        env.storage().instance().set(&PRINCIPAL_SUPPLY, &(principal_supply - amount));

        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply - amount));

        env.events().publish(
            (Symbol::new(&env, "burn_principal"), from),
            amount
        );
    }

    /// Transfer tokens by specific type (coupon or principal)
    pub fn transfer_by_type(env: Env, from: Address, to: Address, amount: i128, token_type: TokenType) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        match token_type {
            TokenType::Coupon => {
                let from_coupon_key = (Symbol::new(&env, "coup_bal"), from.clone());
                let to_coupon_key = (Symbol::new(&env, "coup_bal"), to.clone());

                let from_coupon_balance: i128 = env.storage()
                    .persistent()
                    .get(&from_coupon_key)
                    .unwrap_or(0);

                if from_coupon_balance < amount {
                    panic!("Insufficient coupon balance");
                }

                let to_coupon_balance: i128 = env.storage()
                    .persistent()
                    .get(&to_coupon_key)
                    .unwrap_or(0);

                // Update coupon balances
                env.storage()
                    .persistent()
                    .set(&from_coupon_key, &(from_coupon_balance - amount));
                env.storage()
                    .persistent()
                    .set(&to_coupon_key, &(to_coupon_balance + amount));

                env.events().publish(
                    (Symbol::new(&env, "transfer_coupon"), from.clone(), to.clone()),
                    amount
                );
            }
            TokenType::Principal => {
                let from_principal_key = (Symbol::new(&env, "prin_bal"), from.clone());
                let to_principal_key = (Symbol::new(&env, "prin_bal"), to.clone());

                let from_principal_balance: i128 = env.storage()
                    .persistent()
                    .get(&from_principal_key)
                    .unwrap_or(0);

                if from_principal_balance < amount {
                    panic!("Insufficient principal balance");
                }

                let to_principal_balance: i128 = env.storage()
                    .persistent()
                    .get(&to_principal_key)
                    .unwrap_or(0);

                // Update principal balances
                env.storage()
                    .persistent()
                    .set(&from_principal_key, &(from_principal_balance - amount));
                env.storage()
                    .persistent()
                    .set(&to_principal_key, &(to_principal_balance + amount));

                env.events().publish(
                    (Symbol::new(&env, "transfer_principal"), from.clone(), to.clone()),
                    amount
                );
            }
        }

        // Update total balances for both addresses
        let from_total_key = (Symbol::new(&env, "balance"), from.clone());
        let to_total_key = (Symbol::new(&env, "balance"), to.clone());

        let from_total_balance: i128 = env.storage()
            .persistent()
            .get(&from_total_key)
            .unwrap_or(0);
        let to_total_balance: i128 = env.storage()
            .persistent()
            .get(&to_total_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&from_total_key, &(from_total_balance - amount));
        env.storage()
            .persistent()
            .set(&to_total_key, &(to_total_balance + amount));

        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            amount
        );
    }

    /// Get total balance
    pub fn balance(env: Env, id: Address) -> i128 {
        let balance_key = Symbol::new(&env, "balance");
        env.storage()
            .persistent()
            .get(&(balance_key, id))
            .unwrap_or(0)
    }

    /// Get coupon token balance
    pub fn coupon_balance(env: Env, id: Address) -> i128 {
        let coupon_balance_key = Symbol::new(&env, "coup_bal");
        env.storage()
            .persistent()
            .get(&(coupon_balance_key, id))
            .unwrap_or(0)
    }

    /// Get principal token balance
    pub fn principal_balance(env: Env, id: Address) -> i128 {
        let principal_balance_key = Symbol::new(&env, "prin_bal");
        env.storage()
            .persistent()
            .get(&(principal_balance_key, id))
            .unwrap_or(0)
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0)
    }

    /// Get coupon token supply
    pub fn coupon_supply(env: Env) -> i128 {
        env.storage().instance().get(&COUPON_SUPPLY).unwrap_or(0)
    }

    /// Get principal token supply
    pub fn principal_supply(env: Env) -> i128 {
        env.storage().instance().get(&PRINCIPAL_SUPPLY).unwrap_or(0)
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&NAME).unwrap()
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&SYMBOL).unwrap()
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DECIMALS).unwrap()
    }

    /// Get escrow contract address
    pub fn get_escrow_contract(env: Env) -> Address {
        env.storage().instance().get(&ESCROW_CONTRACT).unwrap()
    }

    /// Get maturity date
    pub fn get_maturity_date(env: Env) -> u64 {
        env.storage().instance().get(&MATURITY_DATE).unwrap()
    }

    /// Approve spending (for compatibility)
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        let allowance_key = Symbol::new(&env, "allowance");
        let key_tuple = (allowance_key, from, spender);

        env.storage()
            .temporary()
            .set(&key_tuple, &amount);

        env.storage()
            .temporary()
            .extend_ttl(&key_tuple, expiration_ledger, expiration_ledger);
    }

    /// Get allowance
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance_key = Symbol::new(&env, "allowance");
        env.storage()
            .temporary()
            .get(&(allowance_key, from, spender))
            .unwrap_or(0)
    }
}
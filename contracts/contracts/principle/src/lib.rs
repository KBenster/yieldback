#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Symbol, String
};

// Storage keys
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const NAME: Symbol = symbol_short!("NAME");
pub const SYMBOL: Symbol = symbol_short!("SYMBOL");
pub const DECIMALS: Symbol = symbol_short!("DECIMALS");
pub const TOTAL_SUPPLY: Symbol = symbol_short!("TOT_SUPP");
pub const ESCROW_CONTRACT: Symbol = symbol_short!("ESCROW");
pub const MATURITY_DATE: Symbol = symbol_short!("MATURITY");

#[contract]
pub struct PrincipalToken;

#[contractimpl]
impl PrincipalToken {
    /// Initialize the principal token
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
    }

    /// Mint principal tokens (only callable by escrow contract)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let escrow_contract: Address = env.storage().instance().get(&ESCROW_CONTRACT).unwrap();
        escrow_contract.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        // Update balance
        let balance_key = (Symbol::new(&env, "balance"), to.clone());
        let current_balance: i128 = env.storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&balance_key, &(current_balance + amount));

        // Update total supply
        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply + amount));

        env.events().publish(
            (Symbol::new(&env, "mint"), to),
            amount
        );
    }

    /// Get admin address
    pub fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN).unwrap()
    }

    /// Get escrow contract address
    pub fn get_escrow_contract(env: Env) -> Address {
        env.storage().instance().get(&ESCROW_CONTRACT).unwrap()
    }

    /// Get maturity date
    pub fn get_maturity_date(env: Env) -> u64 {
        env.storage().instance().get(&MATURITY_DATE).unwrap()
    }

    /// Get total supply
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0)
    }
}

// Standard token interface implementation
#[contractimpl]
impl PrincipalToken {
    /// Returns the allowance for `spender` to transfer from `from`.
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let allowance_key = (Symbol::new(&env, "allowance"), from, spender);
        env.storage()
            .temporary()
            .get(&allowance_key)
            .unwrap_or(0)
    }

    /// Set the allowance by `amount` for `spender` to transfer/burn from `from`.
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        let allowance_key = (Symbol::new(&env, "allowance"), from.clone(), spender.clone());

        env.storage()
            .temporary()
            .set(&allowance_key, &amount);

        env.storage()
            .temporary()
            .extend_ttl(&allowance_key, expiration_ledger, expiration_ledger);

        env.events().publish(
            (Symbol::new(&env, "approve"), from, spender),
            (amount, expiration_ledger)
        );
    }

    /// Returns the balance of `id`.
    pub fn balance(env: Env, id: Address) -> i128 {
        let balance_key = (Symbol::new(&env, "balance"), id);
        env.storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0)
    }

    /// Transfer `amount` from `from` to `to`.
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        if amount == 0 {
            return;
        }

        let from_balance_key = (Symbol::new(&env, "balance"), from.clone());
        let to_balance_key = (Symbol::new(&env, "balance"), to.clone());

        let from_balance: i128 = env.storage()
            .persistent()
            .get(&from_balance_key)
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env.storage()
            .persistent()
            .get(&to_balance_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&from_balance_key, &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&to_balance_key, &(to_balance + amount));

        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            amount
        );
    }

    /// Transfer `amount` from `from` to `to`, consuming the allowance of `spender`.
    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        if amount == 0 {
            return;
        }

        let allowance_key = (Symbol::new(&env, "allowance"), from.clone(), spender.clone());
        let current_allowance: i128 = env.storage()
            .temporary()
            .get(&allowance_key)
            .unwrap_or(0);

        if current_allowance < amount {
            panic!("Insufficient allowance");
        }

        let from_balance_key = (Symbol::new(&env, "balance"), from.clone());
        let to_balance_key = (Symbol::new(&env, "balance"), to.clone());

        let from_balance: i128 = env.storage()
            .persistent()
            .get(&from_balance_key)
            .unwrap_or(0);

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance: i128 = env.storage()
            .persistent()
            .get(&to_balance_key)
            .unwrap_or(0);

        // Update balances
        env.storage()
            .persistent()
            .set(&from_balance_key, &(from_balance - amount));
        env.storage()
            .persistent()
            .set(&to_balance_key, &(to_balance + amount));

        // Update allowance
        env.storage()
            .temporary()
            .set(&allowance_key, &(current_allowance - amount));

        env.events().publish(
            (Symbol::new(&env, "transfer"), from, to),
            amount
        );
    }

    /// Burn `amount` from `from`.
    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let balance_key = (Symbol::new(&env, "balance"), from.clone());
        let current_balance: i128 = env.storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);

        if current_balance < amount {
            panic!("Insufficient balance");
        }

        // Update balance
        env.storage()
            .persistent()
            .set(&balance_key, &(current_balance - amount));

        // Update total supply
        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply - amount));

        env.events().publish(
            (Symbol::new(&env, "burn"), from),
            amount
        );
    }

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let allowance_key = (Symbol::new(&env, "allowance"), from.clone(), spender.clone());
        let current_allowance: i128 = env.storage()
            .temporary()
            .get(&allowance_key)
            .unwrap_or(0);

        if current_allowance < amount {
            panic!("Insufficient allowance");
        }

        let balance_key = (Symbol::new(&env, "balance"), from.clone());
        let current_balance: i128 = env.storage()
            .persistent()
            .get(&balance_key)
            .unwrap_or(0);

        if current_balance < amount {
            panic!("Insufficient balance");
        }

        // Update balance
        env.storage()
            .persistent()
            .set(&balance_key, &(current_balance - amount));

        // Update allowance
        env.storage()
            .temporary()
            .set(&allowance_key, &(current_allowance - amount));

        // Update total supply
        let total_supply: i128 = env.storage().instance().get(&TOTAL_SUPPLY).unwrap();
        env.storage().instance().set(&TOTAL_SUPPLY, &(total_supply - amount));

        env.events().publish(
            (Symbol::new(&env, "burn"), from),
            amount
        );
    }

    /// Returns the number of decimals used to represent amounts of this token.
    pub fn decimals(env: Env) -> u32 {
        env.storage().instance().get(&DECIMALS).unwrap()
    }

    /// Returns the name for this token.
    pub fn name(env: Env) -> String {
        env.storage().instance().get(&NAME).unwrap()
    }

    /// Returns the symbol for this token.
    pub fn symbol(env: Env) -> String {
        env.storage().instance().get(&SYMBOL).unwrap()
    }
}
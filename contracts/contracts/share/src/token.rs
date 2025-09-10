#![no_std]
use soroban_sdk::{
    contracttype, symbol_short,
    Address, Env, Symbol, String
};

// Stellar Token Interface Types
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
    Admin,
}

// Common storage keys
pub const NAME: Symbol = symbol_short!("NAME");
pub const SYMBOL: Symbol = symbol_short!("SYMBOL");
pub const DECIMALS: Symbol = symbol_short!("DECIMALS");
pub const TOTAL_SUPPLY: Symbol = symbol_short!("TOT_SUPP");
pub const ESCROW_CONTRACT: Symbol = symbol_short!("ESCROW");
pub const MATURITY_DATE: Symbol = symbol_short!("MATURITY");

pub fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount);
    }
}

pub fn receive_balance(env: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    if let Some(balance) = env.storage().temporary().get::<DataKey, i128>(&key) {
        balance
    } else {
        0
    }
}

pub fn write_balance(env: &Env, addr: Address, amount: i128) {
    let key = DataKey::Balance(addr);
    env.storage().temporary().set(&key, &amount);
}

pub fn spend_balance(env: &Env, addr: Address, amount: i128) {
    let balance = receive_balance(env, addr.clone());
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(env, addr, balance - amount);
}

pub fn receive_allowance(env: &Env, from: Address, spender: Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    if let Some(allowance) = env.storage().temporary().get::<DataKey, AllowanceValue>(&key) {
        if allowance.expiration_ledger < env.ledger().sequence() {
            AllowanceValue {
                amount: 0,
                expiration_ledger: allowance.expiration_ledger,
            }
        } else {
            allowance
        }
    } else {
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

pub fn write_allowance(
    env: &Env,
    from: Address,
    spender: Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    env.storage().temporary().set(&key, &allowance);

    if amount > 0 && expiration_ledger < env.ledger().sequence() {
        panic!("expiration_ledger is less than ledger seq when amount > 0")
    }

    if expiration_ledger != 0 {
        env.storage()
            .temporary()
            .extend_ttl(&key, expiration_ledger, expiration_ledger);
    }
}

pub fn spend_allowance(env: &Env, from: Address, spender: Address, amount: i128) {
    let allowance = receive_allowance(env, from.clone(), spender.clone());
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }
    write_allowance(
        env,
        from,
        spender,
        allowance.amount - amount,
        allowance.expiration_ledger,
    );
}

pub fn check_authorized(env: &Env, auth_id: Address) {
    let key = DataKey::State(auth_id);
    let authorized: bool = env.storage().temporary().get(&key).unwrap_or(false);
    if !authorized {
        panic!("not authorized");
    }
}

pub fn write_authorization(env: &Env, id: Address, authorize: bool) {
    let key = DataKey::State(id);
    env.storage().temporary().set(&key, &authorize);
}

pub fn write_admin(env: &Env, id: Address) {
    env.storage().instance().set(&DataKey::Admin, &id);
}

pub fn read_admin(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Admin).unwrap()
}

// Shared token implementation functions
pub fn initialize(
    env: &Env,
    admin: Address,
    decimal: u32,
    name: String,
    symbol: String,
    escrow_contract: Address,
    maturity_date: u64,
) {
    if env.storage().instance().has(&DataKey::Admin) {
        panic!("already initialized");
    }

    env.storage().instance().set(&DataKey::Admin, &admin);
    env.storage().instance().set(&NAME, &name);
    env.storage().instance().set(&SYMBOL, &symbol);
    env.storage().instance().set(&DECIMALS, &decimal);
    env.storage().instance().set(&TOTAL_SUPPLY, &0i128);
    env.storage().instance().set(&ESCROW_CONTRACT, &escrow_contract);
    env.storage().instance().set(&MATURITY_DATE, &maturity_date);
}

pub fn allowance(env: &Env, from: Address, spender: Address) -> i128 {
    receive_allowance(env, from, spender).amount
}

pub fn approve(env: &Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
    from.require_auth();
    check_nonnegative_amount(amount);
    write_allowance(env, from, spender, amount, expiration_ledger);
}

pub fn balance(env: &Env, id: Address) -> i128 {
    receive_balance(env, id)
}

pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
    from.require_auth();
    check_nonnegative_amount(amount);
    spend_balance(env, from.clone(), amount);
    write_balance(env, to.clone(), receive_balance(env, to) + amount);
}

pub fn transfer_from(env: &Env, spender: Address, from: Address, to: Address, amount: i128) {
    spender.require_auth();
    check_nonnegative_amount(amount);
    spend_allowance(env, from.clone(), spender, amount);
    spend_balance(env, from, amount);
    write_balance(env, to.clone(), receive_balance(env, to) + amount);
}

pub fn burn(env: &Env, from: Address, amount: i128) {
    from.require_auth();
    check_nonnegative_amount(amount);
    spend_balance(env, from, amount);
    let total = total_supply(env);
    env.storage().instance().set(&TOTAL_SUPPLY, &(total - amount));
}

pub fn burn_from(env: &Env, spender: Address, from: Address, amount: i128) {
    spender.require_auth();
    check_nonnegative_amount(amount);
    spend_allowance(env, from.clone(), spender, amount);
    spend_balance(env, from, amount);
    let total = total_supply(env);
    env.storage().instance().set(&TOTAL_SUPPLY, &(total - amount));
}

pub fn decimals(env: &Env) -> u32 {
    env.storage().instance().get(&DECIMALS).unwrap()
}

pub fn name(env: &Env) -> String {
    env.storage().instance().get(&NAME).unwrap()
}

pub fn symbol(env: &Env) -> String {
    env.storage().instance().get(&SYMBOL).unwrap()
}

pub fn total_supply(env: &Env) -> i128 {
    env.storage().instance().get(&TOTAL_SUPPLY).unwrap_or(0)
}

pub fn mint(env: &Env, to: Address, amount: i128) {
    let escrow_contract: Address = env.storage().instance().get(&ESCROW_CONTRACT).unwrap();
    escrow_contract.require_auth();
    check_nonnegative_amount(amount);
    write_balance(env, to.clone(), receive_balance(env, to) + amount);
    let total = total_supply(env);
    env.storage().instance().set(&TOTAL_SUPPLY, &(total + amount));
}

pub fn set_admin(env: &Env, new_admin: Address) {
    let admin = read_admin(env);
    admin.require_auth();
    write_admin(env, new_admin);
}

pub fn admin(env: &Env) -> Address {
    read_admin(env)
}

pub fn authorized(env: &Env, id: Address) -> bool {
    let key = DataKey::State(id);
    env.storage().temporary().get(&key).unwrap_or(false)
}

pub fn set_authorized(env: &Env, id: Address, authorize: bool) {
    let admin = read_admin(env);
    admin.require_auth();
    write_authorization(env, id, authorize);
}

// Custom getters for bond-specific data
pub fn get_escrow_contract(env: &Env) -> Address {
    env.storage().instance().get(&ESCROW_CONTRACT).unwrap()
}

pub fn get_maturity_date(env: &Env) -> u64 {
    env.storage().instance().get(&MATURITY_DATE).unwrap()
}
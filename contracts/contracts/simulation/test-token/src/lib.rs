#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Balance(Address),
    Allowance(AllowanceDataKey),
    Decimals,
    Name,
    Symbol,
}

#[contract]
pub struct TestToken;

#[contractimpl]
impl TestToken {
    pub fn __constructor(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }

        if decimal > 18 {
            panic!("Decimal must not be greater than 18");
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Decimals, &decimal);
        env.storage().instance().set(&DataKey::Name, &name);
        env.storage().instance().set(&DataKey::Symbol, &symbol);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Not initialized");

        admin.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let balance_key = DataKey::Balance(to.clone());
        let balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);
        env.storage().persistent().set(&balance_key, &(balance + amount));
        env.storage().persistent().extend_ttl(&balance_key, 5184000, 5184000);
    }

    pub fn set_admin(env: Env, new_admin: Address) {
        let admin: Address = env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Not initialized");

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &new_admin);
    }

    pub fn admin(env: Env) -> Address {
        env.storage().instance()
            .get(&DataKey::Admin)
            .expect("Not initialized")
    }

    // SEP-41 Token Interface

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        if let Some(allowance) = env.storage().persistent().get::<DataKey, AllowanceValue>(&key) {
            if allowance.expiration_ledger < env.ledger().sequence() {
                0
            } else {
                allowance.amount
            }
        } else {
            0
        }
    }

    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let key = DataKey::Allowance(AllowanceDataKey { from, spender });
        let allowance = AllowanceValue { amount, expiration_ledger };
        env.storage().persistent().set(&key, &allowance);
        env.storage().persistent().extend_ttl(&key, 5184000, 5184000);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        let key = DataKey::Balance(id);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let from_key = DataKey::Balance(from.clone());
        let to_key = DataKey::Balance(to.clone());

        let from_balance: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&from_key, &(from_balance - amount));
        env.storage().persistent().extend_ttl(&from_key, 5184000, 5184000);

        let to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
        env.storage().persistent().set(&to_key, &(to_balance + amount));
        env.storage().persistent().extend_ttl(&to_key, 5184000, 5184000);
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let allowance_key = DataKey::Allowance(AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone()
        });

        let allowance = env.storage().persistent()
            .get::<DataKey, AllowanceValue>(&allowance_key)
            .expect("No allowance");

        if allowance.expiration_ledger < env.ledger().sequence() {
            panic!("Allowance expired");
        }

        if allowance.amount < amount {
            panic!("Insufficient allowance");
        }

        env.storage().persistent().set(&allowance_key, &AllowanceValue {
            amount: allowance.amount - amount,
            expiration_ledger: allowance.expiration_ledger,
        });
        env.storage().persistent().extend_ttl(&allowance_key, 5184000, 5184000);

        let from_key = DataKey::Balance(from.clone());
        let to_key = DataKey::Balance(to.clone());

        let from_balance: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&from_key, &(from_balance - amount));
        env.storage().persistent().extend_ttl(&from_key, 5184000, 5184000);

        let to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
        env.storage().persistent().set(&to_key, &(to_balance + amount));
        env.storage().persistent().extend_ttl(&to_key, 5184000, 5184000);
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let key = DataKey::Balance(from);
        let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&key, &(balance - amount));
        env.storage().persistent().extend_ttl(&key, 5184000, 5184000);
    }

    pub fn burn_from(env: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        if amount < 0 {
            panic!("Amount must be non-negative");
        }

        let allowance_key = DataKey::Allowance(AllowanceDataKey {
            from: from.clone(),
            spender: spender.clone()
        });

        let allowance = env.storage().persistent()
            .get::<DataKey, AllowanceValue>(&allowance_key)
            .expect("No allowance");

        if allowance.expiration_ledger < env.ledger().sequence() {
            panic!("Allowance expired");
        }

        if allowance.amount < amount {
            panic!("Insufficient allowance");
        }

        env.storage().persistent().set(&allowance_key, &AllowanceValue {
            amount: allowance.amount - amount,
            expiration_ledger: allowance.expiration_ledger,
        });
        env.storage().persistent().extend_ttl(&allowance_key, 5184000, 5184000);

        let balance_key = DataKey::Balance(from);
        let balance: i128 = env.storage().persistent().get(&balance_key).unwrap_or(0);

        if balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&balance_key, &(balance - amount));
        env.storage().persistent().extend_ttl(&balance_key, 5184000, 5184000);
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage().instance()
            .get(&DataKey::Decimals)
            .expect("Not initialized")
    }

    pub fn name(env: Env) -> String {
        env.storage().instance()
            .get(&DataKey::Name)
            .expect("Not initialized")
    }

    pub fn symbol(env: Env) -> String {
        env.storage().instance()
            .get(&DataKey::Symbol)
            .expect("Not initialized")
    }
}
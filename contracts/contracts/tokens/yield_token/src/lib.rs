#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
}

#[contract]
pub struct YieldToken;

#[contractimpl]
impl YieldToken {
    pub fn __constructor(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
    ) {
        admin.require_auth();

        let metadata = TokenMetadata {
            name,
            symbol,
        };

        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"metadata", &metadata);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&"admin").unwrap();
        admin.require_auth();

        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&to, &(balance + amount));

        let total_supply: i128 = env.storage().instance().get(&"total_supply").unwrap_or(0);
        env.storage().instance().set(&"total_supply", &(total_supply + amount));
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage().persistent().set(&from, &(from_balance - amount));
        env.storage().persistent().set(&to, &(to_balance + amount));
    }

    pub fn balance(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&address).unwrap_or(0)
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&"total_supply").unwrap_or(0)
    }

    pub fn name(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.name
    }

    pub fn symbol(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.symbol
    }
}

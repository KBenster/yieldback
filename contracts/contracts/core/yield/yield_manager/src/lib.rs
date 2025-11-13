#![no_std]
use soroban_sdk::{Address, Env};

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

pub trait YieldManagerTrait {
    fn __constructor(env: Env, admin: Address, cache_same_block: bool);
    fn exchange_rate_current(env: Env) -> i128;
    fn update_exchange_rate(env: Env, new_rate: i128);
    fn exchange_rate_stored(env: Env) -> i128;
    fn last_updated_ledger(env: Env) -> u32;
}

#[cfg(feature = "contract")]
#[contract]
pub struct YieldManager;

#[cfg(feature = "contract")]
#[contractimpl]
impl YieldManagerTrait for YieldManager {
    fn __constructor(env: Env, admin: Address, cache_same_block: bool) {
        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"cache_same_block", &cache_same_block);

        // Initialize exchange rate to 1.0 (represented as 10^7 for 7 decimals precision)
        // 1.0 = 10000000 with 7 decimals
        let initial_rate: i128 = 10_000_000;
        env.storage().instance().set(&"exchange_rate", &initial_rate);

        // Initialize last updated ledger to current ledger
        let current_ledger = env.ledger().sequence();
        env.storage().instance().set(&"last_updated_ledger", &current_ledger);
    }

    fn exchange_rate_current(env: Env) -> i128 {
        let cache_enabled: bool = env.storage()
            .instance()
            .get(&"cache_same_block")
            .unwrap_or(false);

        let current_ledger = env.ledger().sequence();
        let last_updated: u32 = env.storage()
            .instance()
            .get(&"last_updated_ledger")
            .unwrap_or(0);

        // If caching is enabled and we're in the same ledger, return cached value
        if cache_enabled && current_ledger == last_updated {
            return env.storage()
                .instance()
                .get(&"exchange_rate")
                .unwrap_or(10_000_000);
        }

        // Otherwise, fetch fresh rate (in real implementation, this would call external contract)
        // For now, just return the stored rate
        let stored_rate: i128 = env.storage()
            .instance()
            .get(&"exchange_rate")
            .unwrap_or(10_000_000);

        // Update the last updated ledger
        env.storage().instance().set(&"last_updated_ledger", &current_ledger);

        stored_rate
    }

    fn update_exchange_rate(env: Env, new_rate: i128) {
        let admin: Address = env.storage().instance().get(&"admin").unwrap();
        admin.require_auth();

        if new_rate <= 0 {
            panic!("Exchange rate must be positive");
        }

        let current_ledger = env.ledger().sequence();

        env.storage().instance().set(&"exchange_rate", &new_rate);
        env.storage().instance().set(&"last_updated_ledger", &current_ledger);
    }

    fn exchange_rate_stored(env: Env) -> i128 {
        // View function - returns cached value without updating
        env.storage()
            .instance()
            .get(&"exchange_rate")
            .unwrap_or(10_000_000)
    }

    fn last_updated_ledger(env: Env) -> u32 {
        // View function - returns the ledger sequence when rate was last updated
        env.storage()
            .instance()
            .get(&"last_updated_ledger")
            .unwrap_or(0)
    }
}

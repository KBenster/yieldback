#![no_std]
use soroban_sdk::{contracttype, Address, Env, String};

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

#[cfg(feature = "contract")]
mod escrow {
    soroban_sdk::contractimport!(
        file = "../../../wasms/escrow.wasm"
    );
}

#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct UserInterest {
    pub index: u128,      // User's last stored PY index
    pub accrued: i128,    // Accrued interest in SY tokens
}

pub trait YieldTokenTrait {
    fn __constructor(env: Env, admin: Address, name: String, symbol: String);
    fn mint(env: Env, to: Address, amount: i128);
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn balance(env: Env, address: Address) -> i128;
    fn total_supply(env: Env) -> i128;
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
}

#[cfg(feature = "contract")]
#[contract]
pub struct YieldToken;

#[cfg(feature = "contract")]
#[contractimpl]
impl YieldTokenTrait for YieldToken {
     fn __constructor(
        env: Env,
        admin: Address,
        name: String,
        symbol: String,
    ) {
        let metadata = TokenMetadata {
            name,
            symbol,
        };

        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"metadata", &metadata);
    }

     fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&"admin").unwrap();
        admin.require_auth();

        // Update interest accruals before balance change
        Self::distribute_interest(env.clone(), to.clone());

        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&to, &(balance + amount));

        let total_supply: i128 = env.storage().instance().get(&"total_supply").unwrap_or(0);
        env.storage().instance().set(&"total_supply", &(total_supply + amount));
    }

     fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        // Update interest accruals for both users before balance changes
        Self::distribute_interest_for_two(env.clone(), from.clone(), to.clone());

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage().persistent().set(&from, &(from_balance - amount));
        env.storage().persistent().set(&to, &(to_balance + amount));
    }

     fn balance(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&address).unwrap_or(0)
    }

     fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&"total_supply").unwrap_or(0)
    }

     fn name(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.name
    }

     fn symbol(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.symbol
    }
}

// Public methods for interest tracking
#[cfg(feature = "contract")]
#[contractimpl]
impl YieldToken {
    pub fn get_user_interest(env: Env, user: Address) -> UserInterest {
        Self::get_user_interest_internal(env, user)
    }

    pub fn redeem_interest(env: Env, user: Address) -> i128 {
        user.require_auth();

        // Distribute interest before redeeming to ensure accrued is up to date
        Self::distribute_interest(env.clone(), user.clone());

        let mut user_interest = Self::get_user_interest_internal(env.clone(), user.clone());
        let accrued_amount = user_interest.accrued;

        if accrued_amount <= 0 {
            return 0;
        }

        // Clear the accrued interest
        user_interest.accrued = 0;
        Self::set_user_interest(env, user, user_interest);

        accrued_amount
    }
}

// Internal helper functions
#[cfg(feature = "contract")]
#[contractimpl]
impl YieldToken {
    /// Get the current PY index from the escrow contract
    fn get_current_py_index(env: Env) -> u128 {
        let escrow_address: Address = env.storage().instance().get(&"escrow").unwrap();

        let escrow_client = escrow::Client::new(&env, &escrow_address);
        escrow_client.get_current_exchange_index() as u128
    }

    /// Calculate interest accrued based on index change
    fn calculate_interest(balance: i128, prev_index: u128, current_index: u128) -> i128 {
        if balance == 0 || prev_index == 0 || current_index <= prev_index {
            return 0;
        }

        // Formula from Pendle: interest = principal * (current_index - prev_index) / (prev_index * current_index)
        // Simplified: interest = balance * (current_index - prev_index) / prev_index / current_index * prev_index
        // Which is: interest = balance * (current_index - prev_index) / current_index

        let index_diff = (current_index - prev_index) as i128;
        let numerator = balance * index_diff;
        let denominator = current_index as i128;

        numerator / denominator
    }

    /// Distribute interest to a single user
    fn distribute_interest(env: Env, user: Address) {
        if user == Address::from_string(&String::from_str(&env, "")) { // Check for zero address
            return;
        }

        let current_index = Self::get_current_py_index(env.clone());
        let mut user_interest = Self::get_user_interest_internal(env.clone(), user.clone());

        // Initialize if first time
        if user_interest.index == 0 {
            user_interest.index = current_index;
            Self::set_user_interest(env.clone(), user.clone(), user_interest);
            return;
        }

        // If index hasn't changed, nothing to do
        if user_interest.index == current_index {
            return;
        }

        // Calculate new interest
        let balance = Self::balance(env.clone(), user.clone());
        let additional_interest = Self::calculate_interest(
            balance,
            user_interest.index,
            current_index,
        );

        // Update user interest
        user_interest.accrued += additional_interest;
        user_interest.index = current_index;

        Self::set_user_interest(env, user, user_interest);
    }

    /// Distribute interest to two users (used in transfers)
    fn distribute_interest_for_two(env: Env, user1: Address, user2: Address) {
        Self::distribute_interest(env.clone(), user1);
        Self::distribute_interest(env, user2);
    }

    /// Get user interest from storage
    fn get_user_interest_internal(env: Env, user: Address) -> UserInterest {
        env.storage()
            .persistent()
            .get(&("interest", user.clone()))
            .unwrap_or(UserInterest {
                index: 0,
                accrued: 0,
            })
    }

    /// Set user interest in storage
    fn set_user_interest(env: Env, user: Address, interest: UserInterest) {
        env.storage()
            .persistent()
            .set(&("interest", user), &interest);
    }
}
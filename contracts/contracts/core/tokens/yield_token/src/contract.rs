use soroban_sdk::{contract, contractimpl, Address, Env, String, IntoVal, Symbol};
use yield_manager_interface::YieldManagerClient;
use crate::storage;

pub trait YieldTokenTrait {
    fn __constructor(env: Env, admin: Address, vault: Address, name: String, symbol: String);
    fn mint(env: Env, to: Address, amount: i128);
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn balance(env: Env, address: Address) -> i128;
    fn user_index(env: Env, address: Address) -> i128;
    fn accrued_yield(env: Env, address: Address) -> i128;
    fn total_supply(env: Env) -> i128;
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn claim_yield(env: Env, user: Address) -> i128;
}

#[contract]
pub struct YieldToken;

impl YieldToken {
    fn accrue_yield(env: &Env, user: &Address) -> i128 {
        let balance = storage::get_balance(env, user);

        // Early return if no balance
        if balance == 0 {
            return 0;
        }

        let old_index = storage::get_user_index(env, user);
        let yield_manager = storage::get_admin(env);
        let yield_manager_client = YieldManagerClient::new(env, &yield_manager);
        let current_rate = yield_manager_client.get_exchange_rate();

        // Initialize index for new users
        if old_index == 0 {
            storage::set_user_index(env, user, current_rate);
            return current_rate;
        }

        // Accrue if rate increased
        if current_rate > old_index {
            let pending_yield = (balance * (current_rate - old_index)) / old_index;
            let current_accrued = storage::get_accrued_yield(env, user);
            storage::set_accrued_yield(env, user, current_accrued + pending_yield);
            storage::set_user_index(env, user, current_rate);
        }

        current_rate
    }
}

#[contractimpl]
impl YieldTokenTrait for YieldToken {
    fn __constructor(
        env: Env,
        admin: Address,
        vault: Address,
        name: String,
        symbol: String,
    ) {
        storage::set_admin(&env, &admin);
        storage::set_vault(&env, &vault);
        storage::set_metadata(&env, name, symbol);
    }

    fn mint(env: Env, to: Address, amount: i128) {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        Self::accrue_yield(&env, &to);

        let old_balance = storage::get_balance(&env, &to);
        let new_balance = old_balance + amount;
        storage::set_balance(&env, &to, new_balance);

        // Initialize index for new users only (preserve high water mark for existing users)
        let old_index = storage::get_user_index(&env, &to);
        if old_index == 0 {
            let yield_manager = storage::get_admin(&env);
            let yield_manager_client = YieldManagerClient::new(&env, &yield_manager);
            storage::set_user_index(&env, &to, yield_manager_client.get_exchange_rate());
        }

        let total_supply = storage::get_total_supply(&env);
        storage::set_total_supply(&env, total_supply + amount);
    }

    fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = storage::get_balance(&env, &from);
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        Self::accrue_yield(&env, &from);
        Self::accrue_yield(&env, &to);

        let to_balance = storage::get_balance(&env, &to);

        storage::set_balance(&env, &from, from_balance - amount);
        storage::set_balance(&env, &to, to_balance + amount);

        // Initialize index for new recipients only (preserve high water mark for returning users)
        let to_index = storage::get_user_index(&env, &to);
        if to_index == 0 {
            let yield_manager = storage::get_admin(&env);
            let yield_manager_client = YieldManagerClient::new(&env, &yield_manager);
            storage::set_user_index(&env, &to, yield_manager_client.get_exchange_rate());
        }
    }

    fn burn(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let balance = storage::get_balance(&env, &from);
        if balance < amount {
            panic!("Insufficient balance");
        }

        Self::accrue_yield(&env, &from);

        storage::set_balance(&env, &from, balance - amount);

        let total_supply = storage::get_total_supply(&env);
        storage::set_total_supply(&env, total_supply - amount);
    }

    fn balance(env: Env, address: Address) -> i128 {
        storage::get_balance(&env, &address)
    }

    fn user_index(env: Env, address: Address) -> i128 {
        storage::get_user_index(&env, &address)
    }

    fn accrued_yield(env: Env, address: Address) -> i128 {
        storage::get_accrued_yield(&env, &address)
    }

    fn total_supply(env: Env) -> i128 {
        storage::get_total_supply(&env)
    }

    fn name(env: Env) -> String {
        storage::get_metadata(&env).name
    }

    fn symbol(env: Env) -> String {
        storage::get_metadata(&env).symbol
    }

    fn claim_yield(env: Env, user: Address) -> i128 {
        user.require_auth();

        Self::accrue_yield(&env, &user);

        let claimable = storage::get_accrued_yield(&env, &user);
        if claimable == 0 {
            return 0;
        }

        storage::set_accrued_yield(&env, &user, 0);

        // Call yield manager (admin) to distribute vault shares
        let yield_manager = storage::get_admin(&env);
        env.invoke_contract::<()>(
            &yield_manager,
            &Symbol::new(&env, "distribute_yield"),
            (user.clone(), claimable).into_val(&env),
        );

        claimable
    }
}

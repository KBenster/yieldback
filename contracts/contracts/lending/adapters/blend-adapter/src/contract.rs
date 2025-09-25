use soroban_sdk::{
    contract, contractimpl, token, panic_with_error,
    Address, Env, Vec, Symbol, vec, IntoVal,
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}
};
use blend_contract_sdk::pool;
use lending_core::{LendingError, Position};
use crate::constants::*;

#[contract]
pub struct BlendAdapterContract;

#[contractimpl]
impl BlendAdapterContract {
    /// Initialize the adapter with Blend pool address
    pub fn __constructor(env: Env, blend_pool_address: Address) {
        // Ensure contract is not already initialized
        if env.storage().instance().has(&BLEND_POOL) {
            panic_with_error!(&env, &LendingError::AlreadyInitialized);
        }

        // Store the Blend pool address
        env.storage().instance().set(&BLEND_POOL, &blend_pool_address);

        // Emit initialization event
        env.events().publish(
            (Symbol::new(&env, "blend_adapter_initialized"), blend_pool_address.clone()),
            0i128
        );
    }

    /// Lend tokens to Blend protocol
    pub fn lend(env: Env, token: Address, amount: i128) -> i128 {
        if amount <= 0 {
            panic_with_error!(&env, &LendingError::AmountMustBePositive);
        }

        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();
        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        // Get current contract balance to confirm we have the tokens
        let token_client = token::Client::new(&env, &token);
        let contract_balance = token_client.balance(&env.current_contract_address());

        if contract_balance < amount {
            panic_with_error!(&env, &LendingError::InsufficientFunds);
        }

        // Authorize token transfer to Blend pool
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: token.clone(),
                    fn_name: Symbol::new(&env, "transfer"),
                    args: (
                        env.current_contract_address(),
                        blend_pool_address.clone(),
                        amount,
                    ).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Create supply request for Blend
        let supply_request = pool::Request {
            request_type: BLEND_SUPPLY_REQUEST,
            address: token.clone(),
            amount: amount,
        };

        let requests = Vec::from_array(&env, [supply_request]);

        // Submit to Blend pool
        blend_pool.submit(
            &env.current_contract_address(), // from (this adapter)
            &env.current_contract_address(), // spender (this adapter)
            &env.current_contract_address(), // to (bTokens recipient - this adapter)
            &requests
        );

        // Emit lending event
        env.events().publish(
            (Symbol::new(&env, "lent_to_blend"), token),
            amount
        );

        amount
    }

    /// Withdraw a specific amount from Blend protocol
    pub fn withdraw(env: Env, token: Address, amount: i128) -> i128 {
        if amount <= 0 {
            panic_with_error!(&env, &LendingError::AmountMustBePositive);
        }

        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();
        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        // Get current positions to check available balance
        let positions = blend_pool.get_positions(&env.current_contract_address());
        let total_supply = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

        if total_supply <= 0 {
            panic_with_error!(&env, &LendingError::NoPositionInProtocol);
        }

        if amount > total_supply {
            panic_with_error!(&env, &LendingError::InsufficientFunds);
        }

        // Create withdrawal request
        let withdraw_request = pool::Request {
            request_type: BLEND_WITHDRAW_REQUEST,
            address: token.clone(),
            amount: amount,
        };

        let requests = Vec::from_array(&env, [withdraw_request]);

        // Submit withdrawal request
        blend_pool.submit(
            &env.current_contract_address(), // from (this adapter)
            &env.current_contract_address(), // spender (this adapter)
            &env.current_contract_address(), // to (withdrawal recipient - this adapter)
            &requests
        );

        // Emit withdrawal event
        env.events().publish(
            (Symbol::new(&env, "withdrawn_from_blend"), token),
            amount
        );

        amount
    }

    /// Withdraw all funds from Blend protocol
    pub fn withdraw_all(env: Env, token: Address) -> i128 {
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();
        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        // Get current positions to withdraw entire balance
        let positions = blend_pool.get_positions(&env.current_contract_address());
        let total_supply = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

        if total_supply <= 0 {
            panic_with_error!(&env, &LendingError::NoPositionInProtocol);
        }

        // Create withdrawal request for entire position
        let withdraw_request = pool::Request {
            request_type: BLEND_WITHDRAW_REQUEST,
            address: token.clone(),
            amount: total_supply,
        };

        let requests = Vec::from_array(&env, [withdraw_request]);

        // Submit withdrawal request
        blend_pool.submit(
            &env.current_contract_address(), // from (this adapter)
            &env.current_contract_address(), // spender (this adapter)
            &env.current_contract_address(), // to (withdrawal recipient - this adapter)
            &requests
        );

        // Emit withdrawal event
        env.events().publish(
            (Symbol::new(&env, "withdrawn_all_from_blend"), token),
            total_supply
        );

        total_supply
    }

    /// Get current position in Blend protocol
    pub fn get_position(env: Env, account: Address, token: Address) -> Position {
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();
        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        let positions = blend_pool.get_positions(&account);
        let supplied_amount = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

        // For Blend, the earned interest would be the difference between
        // current supply position and originally supplied amount
        // This is a simplified calculation - in practice you'd need to track
        // the original supply amount separately
        Position {
            supplied_amount,
            earned_interest: 0, // Simplified for this example
            total_value: supplied_amount,
        }
    }

    /// Check if Blend protocol is healthy/available
    pub fn is_protocol_healthy(env: Env) -> bool {
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

        // Simple health check - try to get pool info
        // In a real implementation, you'd check pool status, liquidity, etc.
        match env.storage().instance().get::<Symbol, Address>(&BLEND_POOL) {
            Some(_) => true,
            None => false,
        }
    }

    /// Get the protocol name
    pub fn get_protocol_name(env: Env) -> Symbol {
        Symbol::new(&env, PROTOCOL_NAME)
    }
}
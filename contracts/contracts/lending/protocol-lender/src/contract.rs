use soroban_sdk::{
    contract, contractclient, contractimpl, contracttype, token, panic_with_error,
    Address, Env, Symbol, BytesN
};
use lending_core::{LendingError, Position};
use crate::constants::*;

#[contract]
pub struct ProtocolLenderContract;

// Storage for protocol adapter address
#[contracttype]
#[derive(Clone)]
pub struct ProtocolAdapter {
    pub adapter_address: Address,
}

// Client interface for protocol adapters
#[contractclient(name = "ProtocolAdapterClient")]
pub trait ProtocolAdapterInterface {
    fn lend(env: Env, token: Address, amount: i128) -> i128;
    fn withdraw(env: Env, token: Address, amount: i128) -> i128;
    fn withdraw_all(env: Env, token: Address) -> i128;
    fn get_position(env: Env, account: Address, token: Address) -> Position;
    fn is_protocol_healthy(env: Env) -> bool;
    fn get_protocol_name(env: Env) -> Symbol;
}

#[contractclient(name = "ProtocolLenderClient")]
pub trait ProtocolLenderInterface {
    fn initialize(
        env: Env,
        admin: Address,
        token_address: Address,
        protocol_address: Address,
        adapter_wasm_hash: BytesN<32>,
        expected_amount: i128,
    );

    fn receive_asset(env: Env, from: Address, amount: i128);
    fn lend_to_protocol(env: Env) -> i128;
    fn withdraw_from_protocol(env: Env, amount: i128) -> i128;
    fn withdraw_all_from_protocol(env: Env) -> i128;

    fn get_admin(env: Env) -> Address;
    fn get_token(env: Env) -> Address;
    fn get_protocol_address(env: Env) -> Address;
    fn get_expected_amount(env: Env) -> i128;
    fn get_received_amount(env: Env) -> i128;
    fn is_amount_received(env: Env) -> bool;
    fn is_lent_to_protocol(env: Env) -> bool;
    fn get_contract_balance(env: Env) -> i128;
    fn get_protocol_position(env: Env) -> Position;
}


#[contractimpl]
impl ProtocolLenderInterface for ProtocolLenderContract {
    fn initialize(
        env: Env,
        admin: Address,
        token_address: Address,
        protocol_address: Address,
        adapter_wasm_hash: BytesN<32>,
        expected_amount: i128,
    ) {
        // Ensure contract is not already initialized
        if env.storage().instance().has(&ADMIN) {
            panic_with_error!(&env, &LendingError::AlreadyInitialized);
        }

        // Validate parameters
        if expected_amount <= 0 {
            panic_with_error!(&env, &LendingError::InvalidAmount);
        }

        // Deploy the protocol adapter
        let salt = BytesN::from_array(&env, &[1u8; 32]);
        let adapter_address = env.deployer().with_current_contract(salt)
            .deploy_v2(
                adapter_wasm_hash,
                (protocol_address.clone(),)
            );

        // Set storage values
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token_address);
        env.storage().instance().set(&PROTOCOL_ADDR, &protocol_address);
        env.storage().instance().set(&PROTOCOL_ADAPTER, &adapter_address);
        env.storage().instance().set(&EXPECTED_AMOUNT, &expected_amount);
        env.storage().instance().set(&RECEIVED_AMOUNT, &0i128);
        env.storage().instance().set(&IS_LENT, &false);

        // Emit initialization event
        env.events().publish(
            (Symbol::new(&env, "initialized"), admin),
            expected_amount
        );
    }

    fn receive_asset(env: Env, from: Address, amount: i128) {
        // Authenticate the sender
        from.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, &LendingError::AmountMustBePositive);
        }

        // Get expected amount and current received amount
        let expected_amount: i128 = env.storage().instance().get(&EXPECTED_AMOUNT).unwrap();
        let current_received: i128 = env.storage().instance().get(&RECEIVED_AMOUNT).unwrap_or(0);

        // Check if we would exceed expected amount
        if current_received + amount > expected_amount {
            panic_with_error!(&env, &LendingError::ExpectedAmountNotMet);
        }

        // Get token contract
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from sender to this contract
        token.transfer(&from, &env.current_contract_address(), &amount);

        // Update received amount
        let new_received = current_received + amount;
        env.storage().instance().set(&RECEIVED_AMOUNT, &new_received);

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "asset_received"), from),
            amount
        );
        env.events().publish(
            (Symbol::new(&env, "total_received"), env.current_contract_address()),
            new_received
        );
    }

    fn lend_to_protocol(env: Env) -> i128 {
        // Get received amount
        let received_amount: i128 = env.storage().instance().get(&RECEIVED_AMOUNT).unwrap_or(0);

        if received_amount <= 0 {
            panic_with_error!(&env, &LendingError::NoTokensToLend);
        }

        // Get contract addresses
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let adapter_address: Address = env.storage().instance().get(&PROTOCOL_ADAPTER).unwrap();

        let token = token::Client::new(&env, &token_address);

        // Get current contract balance to confirm we have the tokens
        let contract_balance = token.balance(&env.current_contract_address());

        if contract_balance < received_amount {
            panic_with_error!(&env, &LendingError::InsufficientFunds);
        }

        // Transfer tokens to the adapter for lending
        token.transfer(&env.current_contract_address(), &adapter_address, &received_amount);

        // Call the adapter to lend to the protocol
        let adapter = ProtocolAdapterClient::new(&env, &adapter_address);
        let lent_amount = adapter.lend(&token_address, &received_amount);

        // Mark as lent
        env.storage().instance().set(&IS_LENT, &true);

        // Emit lending event
        env.events().publish(
            (Symbol::new(&env, "lent_to_protocol"), env.current_contract_address()),
            lent_amount
        );

        lent_amount
    }

    fn withdraw_from_protocol(env: Env, amount: i128) -> i128 {
        // Only admin can withdraw
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        if amount <= 0 {
            panic_with_error!(&env, &LendingError::AmountMustBePositive);
        }

        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let adapter_address: Address = env.storage().instance().get(&PROTOCOL_ADAPTER).unwrap();

        // Call the adapter to withdraw from the protocol
        let adapter = ProtocolAdapterClient::new(&env, &adapter_address);
        let withdrawn_amount = adapter.withdraw(&token_address, &amount);

        // Transfer withdrawn tokens from adapter back to this contract
        let token = token::Client::new(&env, &token_address);
        token.transfer(&adapter_address, &env.current_contract_address(), &withdrawn_amount);

        // Emit withdrawal event
        env.events().publish(
            (Symbol::new(&env, "withdrawn_from_protocol"), env.current_contract_address()),
            withdrawn_amount
        );

        withdrawn_amount
    }

    fn withdraw_all_from_protocol(env: Env) -> i128 {
        // Only admin can withdraw
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let adapter_address: Address = env.storage().instance().get(&PROTOCOL_ADAPTER).unwrap();

        // Call the adapter to withdraw all from the protocol
        let adapter = ProtocolAdapterClient::new(&env, &adapter_address);
        let withdrawn_amount = adapter.withdraw_all(&token_address);

        // Transfer withdrawn tokens from adapter back to this contract
        let token = token::Client::new(&env, &token_address);
        token.transfer(&adapter_address, &env.current_contract_address(), &withdrawn_amount);

        // Update lent status if fully withdrawn
        if withdrawn_amount > 0 {
            env.storage().instance().set(&IS_LENT, &false);
        }

        // Emit withdrawal event
        env.events().publish(
            (Symbol::new(&env, "withdrawn_all_from_protocol"), env.current_contract_address()),
            withdrawn_amount
        );

        withdrawn_amount
    }

    fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN).unwrap()
    }

    fn get_token(env: Env) -> Address {
        env.storage().instance().get(&TOKEN).unwrap()
    }

    fn get_protocol_address(env: Env) -> Address {
        env.storage().instance().get(&PROTOCOL_ADDR).unwrap()
    }

    fn get_expected_amount(env: Env) -> i128 {
        env.storage().instance().get(&EXPECTED_AMOUNT).unwrap()
    }

    fn get_received_amount(env: Env) -> i128 {
        env.storage().instance().get(&RECEIVED_AMOUNT).unwrap_or(0)
    }

    fn is_amount_received(env: Env) -> bool {
        let expected: i128 = env.storage().instance().get(&EXPECTED_AMOUNT).unwrap();
        let received: i128 = env.storage().instance().get(&RECEIVED_AMOUNT).unwrap_or(0);
        received >= expected
    }

    fn is_lent_to_protocol(env: Env) -> bool {
        env.storage().instance().get(&IS_LENT).unwrap_or(false)
    }

    fn get_contract_balance(env: Env) -> i128 {
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);
        token.balance(&env.current_contract_address())
    }

    fn get_protocol_position(env: Env) -> Position {
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let adapter_address: Address = env.storage().instance().get(&PROTOCOL_ADAPTER).unwrap();

        let adapter = ProtocolAdapterClient::new(&env, &adapter_address);
        adapter.get_position(&env.current_contract_address(), &token_address)
    }
}
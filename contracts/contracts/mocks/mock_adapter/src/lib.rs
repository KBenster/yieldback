#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, IntoVal, Symbol, Val, Vec as SorobanVec};
use adapter_trait::YieldAdapter;

#[contracttype]
#[derive(Clone, Copy)]
pub enum DataKey {
    Escrow,
    YieldProtocol,
    Token,
}

/// Mock adapter contract for testing
/// Implements the YieldAdapter trait and interacts with any yield protocol
/// Uses generic contract calls to work with the mock yield protocol
#[contract]
pub struct MockAdapter;

#[contractimpl]
impl YieldAdapter for MockAdapter {
    /// Initialize the adapter with the escrow contract, yield protocol address and token address
    fn __constructor(env: Env, escrow: Address, yield_protocol: Address, token: Address) {
        env.storage().instance().set(&DataKey::Escrow, &escrow);
        env.storage().instance().set(&DataKey::YieldProtocol, &yield_protocol);
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// Receive funds from escrow and deposit them into the yield protocol
    fn deposit(env: Env, depositor: Address, amount: i128) {
        depositor.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let yield_protocol_address: Address = env.storage().instance().get(&DataKey::YieldProtocol).unwrap();

        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from depositor to adapter
        token.transfer(&depositor, &env.current_contract_address(), &amount);

        // Deposit tokens into the yield protocol using invoke_contract
        let mut args = SorobanVec::new(&env);
        args.push_back(env.current_contract_address().into_val(&env));
        args.push_back(amount.into_val(&env));

        env.invoke_contract::<Val>(
            &yield_protocol_address,
            &Symbol::new(&env, "deposit"),
            args,
        );
    }

    /// Withdraw funds from the yield protocol and send to escrow
    fn withdraw(env: Env, amount: i128) {
        let escrow: Address = env.storage().instance().get(&DataKey::Escrow).unwrap();
        escrow.require_auth();

        let token_address: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let yield_protocol_address: Address = env.storage().instance().get(&DataKey::YieldProtocol).unwrap();

        let token = token::Client::new(&env, &token_address);

        // Withdraw from yield protocol (this includes accrued yield!) using invoke_contract
        let mut args = SorobanVec::new(&env);
        args.push_back(env.current_contract_address().into_val(&env));
        args.push_back(amount.into_val(&env));

        env.invoke_contract::<Val>(
            &yield_protocol_address,
            &Symbol::new(&env, "withdraw"),
            args,
        );

        // Transfer tokens from adapter to escrow
        token.transfer(&env.current_contract_address(), &escrow, &amount);
    }

    /// Get the configured yield protocol address
    fn get_yield_protocol(env: Env) -> Address {
        env.storage().instance().get(&DataKey::YieldProtocol).unwrap()
    }

    /// Get the configured token address
    fn get_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Token).unwrap()
    }

    /// Get the total assets managed by this adapter (queries the yield protocol)
    fn get_assets(env: Env) -> i128 {
        let yield_protocol_address: Address = env.storage().instance().get(&DataKey::YieldProtocol).unwrap();

        // Query the adapter's balance in the yield protocol using invoke_contract
        let mut args = SorobanVec::new(&env);
        args.push_back(env.current_contract_address().into_val(&env));

        env.invoke_contract::<i128>(
            &yield_protocol_address,
            &Symbol::new(&env, "balance"),
            args,
        )
    }
}
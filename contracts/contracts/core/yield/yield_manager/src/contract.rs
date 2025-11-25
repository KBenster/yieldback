use soroban_sdk::{token, Address, Env};
use crate::storage;
use vault_interface::VaultContractClient;
use yield_manager_interface::YieldManagerTrait;
use principal_token::PrincipalTokenClient;
use yield_token::YieldTokenClient;

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

#[cfg(feature = "contract")]
#[contract]
pub struct YieldManager;

#[cfg(feature = "contract")]
#[contractimpl]
impl YieldManagerTrait for YieldManager {
    fn __constructor(
        env: Env,
        admin: Address,
        vault: Address,
        maturity: u64,
    ) {
        storage::set_admin(&env, &admin);
        storage::set_vault(&env, &vault);
        storage::set_maturity(&env, maturity);
    }

    fn set_token_contracts(env: Env, pt_addr: Address, yt_addr: Address) {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        // Ensure this can only be called once
        if storage::is_initialized(&env) {
            panic!("Token contracts already initialized");
        }

        storage::set_principal_token(&env, &pt_addr);
        storage::set_yield_token(&env, &yt_addr);
        storage::set_initialized(&env);
    }

    fn get_vault(env: Env) -> Address {
        storage::get_vault(&env)
    }

    fn get_principal_token(env: Env) -> Address {
        storage::get_principal_token(&env)
    }

    fn get_yield_token(env: Env) -> Address {
        storage::get_yield_token(&env)
    }

    fn get_maturity(env: Env) -> u64 {
        storage::get_maturity(&env)
    }

    fn get_exchange_rate(env: Env) -> i128 {
        // If exchange rate at expiry is already set, return it
        if let Some(rate_at_expiry) = storage::get_exchange_rate_at_expiry(&env) {
            return rate_at_expiry;
        }

        // Get current vault exchange rate
        let vault_addr = storage::get_vault(&env);
        let vault_client = VaultContractClient::new(&env, &vault_addr);
        let current_rate = vault_client.exchange_rate();

        // If maturity has passed, automatically set and lock the rate at expiry
        let maturity = storage::get_maturity(&env);
        let current_time = env.ledger().timestamp();
        if current_time >= maturity {
            storage::set_exchange_rate_at_expiry(&env, current_rate);
        }

        current_rate
    }

    fn get_exchange_rate_at_expiry(env: Env) -> Option<i128> {
        storage::get_exchange_rate_at_expiry(&env)
    }

    fn set_exchange_rate_at_expiry(env: Env) {
        // Check that maturity has been reached
        let maturity = storage::get_maturity(&env);
        let current_time = env.ledger().timestamp();
        if current_time < maturity {
            panic!("Maturity not reached yet");
        }

        // Check if exchange rate at expiry is already set
        if storage::get_exchange_rate_at_expiry(&env).is_some() {
            panic!("Exchange rate at expiry already set");
        }

        // Get and store the current exchange rate
        let vault_addr = storage::get_vault(&env);
        let vault_client = VaultContractClient::new(&env, &vault_addr);
        let exchange_rate = vault_client.exchange_rate();

        storage::set_exchange_rate_at_expiry(&env, exchange_rate);
    }

    fn deposit(env: Env, from: Address, shares_amount: i128) {
        from.require_auth();

        if shares_amount <= 0 {
            panic!("Amount must be positive");
        }

        let vault_addr = storage::get_vault(&env);
        let pt_addr = storage::get_principal_token(&env);
        let yt_addr = storage::get_yield_token(&env);

        // Get the current exchange rate from the vault
        let vault_client = VaultContractClient::new(&env, &vault_addr);
        let exchange_rate = vault_client.exchange_rate();

        // Calculate the amount of tokens to mint based on shares and exchange rate
        let mint_amount = shares_amount * exchange_rate;

        // Transfer vault shares from user to yield manager
        let vault_token_client = token::Client::new(&env, &vault_addr);
        vault_token_client.transfer(&from, &env.current_contract_address(), &shares_amount);

        // Mint PT tokens to user (shares * exchange_rate) using type-safe client
        let pt_client = PrincipalTokenClient::new(&env, &pt_addr);
        pt_client.mint(&from, &mint_amount);

        // Mint YT tokens to user (shares * exchange_rate) using type-safe client
        let yt_client = YieldTokenClient::new(&env, &yt_addr);
        yt_client.mint(&from, &mint_amount, &exchange_rate);
    }

    fn distribute_yield(env: Env, to: Address, shares_amount: i128) {
        // Only the YT contract can call this
        let yt_addr = storage::get_yield_token(&env);
        yt_addr.require_auth();

        if shares_amount <= 0 {
            return;
        }

        // Transfer vault shares from yield manager to user
        let vault_addr = storage::get_vault(&env);
        let vault_token_client = token::Client::new(&env, &vault_addr);
        vault_token_client.transfer(
            &env.current_contract_address(),
            &to,
            &shares_amount,
        );
    }

    fn redeem_principal(env: Env, from: Address, pt_amount: i128) {
        from.require_auth();

        if pt_amount <= 0 {
            panic!("Amount must be positive");
        }

        // Check maturity has passed
        let maturity = storage::get_maturity(&env);
        let current_time = env.ledger().timestamp();
        if current_time < maturity {
            panic!("Maturity not reached");
        }

        let vault_addr = storage::get_vault(&env);
        let pt_addr = storage::get_principal_token(&env);

        // Get exchange rate (will be capped at expiry if set)
        let exchange_rate = Self::get_exchange_rate(env.clone());
        let shares_to_return = pt_amount / exchange_rate;

        // Burn PT tokens from user using type-safe client
        let pt_client = PrincipalTokenClient::new(&env, &pt_addr);
        pt_client.burn(&from, &pt_amount);

        // Transfer vault shares back to user
        let vault_token_client = token::Client::new(&env, &vault_addr);
        vault_token_client.transfer(
            &env.current_contract_address(),
            &from,
            &shares_to_return,
        );
    }
}
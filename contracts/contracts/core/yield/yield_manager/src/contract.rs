use soroban_sdk::{token, Address, BytesN, Env, String};
use crate::storage;
use vault_core::VaultContractClient;
use principal_token::PrincipalTokenClient;
use yield_token::YieldTokenClient;

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

pub trait YieldManagerTrait {
    fn __constructor(
        env: Env,
        admin: Address,
        vault: Address,
        maturity: u64,
        pt_wasm_hash: BytesN<32>,
        yt_wasm_hash: BytesN<32>,
    );

    fn get_vault(env: Env) -> Address;
    fn get_principal_token(env: Env) -> Address;
    fn get_yield_token(env: Env) -> Address;
    fn get_maturity(env: Env) -> u64;
    fn get_exchange_rate(env: Env) -> i128;
    fn deposit(env: Env, from: Address, shares_amount: i128);
}

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
        pt_wasm_hash: BytesN<32>, // THESE NEED TO BE PART OF THE HIGHER DEPLOYER CONTRACT
        yt_wasm_hash: BytesN<32>,
    ) {
        storage::set_admin(&env, &admin);
        storage::set_vault(&env, &vault);
        storage::set_maturity(&env, maturity);

        // Deploy Principal Token with hardcoded metadata
        let pt_salt = BytesN::from_array(&env, &[0u8; 32]);
        let pt_addr = env
            .deployer()
            .with_current_contract(pt_salt)
            .deploy_v2(
                pt_wasm_hash,
                (
                    env.current_contract_address(),
                    String::from_str(&env, "Principal Token"),
                    String::from_str(&env, "PT"),
                ),
            );

        storage::set_principal_token(&env, &pt_addr);

        // Deploy Yield Token with hardcoded metadata
        let yt_salt = BytesN::from_array(&env, &[1u8; 32]);
        let yt_addr = env
            .deployer()
            .with_current_contract(yt_salt)
            .deploy_v2(
                yt_wasm_hash,
                (
                    env.current_contract_address(),
                    String::from_str(&env, "Yield Token"),
                    String::from_str(&env, "YT"),
                ),
            );

        storage::set_yield_token(&env, &yt_addr);
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
        let vault_addr = storage::get_vault(&env);
        let vault_client = VaultContractClient::new(&env, &vault_addr);
        vault_client.exchange_rate()
    }

    fn deposit(env: Env, from: Address, shares_amount: i128) {
        from.require_auth();

        if shares_amount <= 0 {
            panic!("Amount must be positive");
        }

        let vault_addr = storage::get_vault(&env);
        let pt_addr = storage::get_principal_token(&env);
        let yt_addr = storage::get_yield_token(&env);

        // Transfer vault shares from user to yield manager
        let vault_token_client = token::Client::new(&env, &vault_addr);
        vault_token_client.transfer(&from, &env.current_contract_address(), &shares_amount);

        // Mint PT tokens to user (1:1 with shares)
        let pt_client = PrincipalTokenClient::new(&env, &pt_addr);
        pt_client.mint(&from, &shares_amount);

        // Mint YT tokens to user (1:1 with shares)
        let yt_client = YieldTokenClient::new(&env, &yt_addr);
        yt_client.mint(&from, &shares_amount);
    }
}
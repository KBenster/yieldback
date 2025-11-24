use soroban_sdk::{Address, BytesN, Env, String};
use crate::storage;
use yield_manager_interface::YieldManagerClient;

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

const PT_WASM_HASH: [u8; 32] = [0u8; 32];
const YT_WASM_HASH: [u8; 32] = [0u8; 32];
const YM_WASM_HASH: [u8; 32] = [0u8; 32];
const AMM_WASM_HASH: [u8; 32] = [0u8; 32];

pub trait FactoryTrait {
    fn __constructor(env: Env, admin: Address);

    fn deploy_yield_manager(
        env: Env,
        vault: Address,
        maturity: u64,
    ) -> Address;

    fn deploy_liquidity_pools(env: Env, yield_manager: Address, vault: Address) -> (Address, Address);
}

#[cfg(feature = "contract")]
#[contract]
pub struct Factory;

#[cfg(feature = "contract")]
#[contractimpl]
impl FactoryTrait for Factory {
    fn __constructor(env: Env, admin: Address) {
        storage::set_admin(&env, &admin);
    }

    fn deploy_yield_manager(
        env: Env,
        vault: Address,
        maturity: u64,
    ) -> Address {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        // Create WASM hash BytesN from constants
        let pt_wasm_hash = BytesN::from_array(&env, &PT_WASM_HASH);
        let yt_wasm_hash = BytesN::from_array(&env, &YT_WASM_HASH);
        let ym_wasm_hash = BytesN::from_array(&env, &YM_WASM_HASH);

        // Deploy yield manager first
        // Use a unique salt based on vault address and maturity
        let mut ym_salt_data = [0u8; 32];
        // Simple salt derivation - could be made more sophisticated
        let ym_salt = BytesN::from_array(&env, &ym_salt_data);

        let ym_addr = env
            .deployer()
            .with_current_contract(ym_salt.clone())
            .deploy_v2(
                ym_wasm_hash,
                (
                    env.current_contract_address(),
                    vault,
                    maturity,
                ),
            );

        // Deploy Principal Token with yield manager as admin
        let pt_salt = BytesN::from_array(&env, &[0u8; 32]);
        let pt_addr = env
            .deployer()
            .with_current_contract(pt_salt)
            .deploy_v2(
                pt_wasm_hash,
                (
                    ym_addr.clone(),
                    String::from_str(&env, "Principal Token"),
                    String::from_str(&env, "PT"),
                ),
            );

        // Deploy Yield Token with yield manager as admin
        let yt_salt = BytesN::from_array(&env, &[1u8; 32]);
        let yt_addr = env
            .deployer()
            .with_current_contract(yt_salt)
            .deploy_v2(
                yt_wasm_hash,
                (
                    ym_addr.clone(),
                    String::from_str(&env, "Yield Token"),
                    String::from_str(&env, "YT"),
                ),
            );

        // Set token contracts in yield manager
        let ym_client = YieldManagerClient::new(&env, &ym_addr);
        ym_client.set_token_contracts(&pt_addr, &yt_addr);

        ym_addr
    }

    fn deploy_liquidity_pools(env: Env, yield_manager: Address, vault: Address) -> (Address, Address) {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        // Get PT and YT addresses from yield manager
        let ym_client = YieldManagerClient::new(&env, &yield_manager);
        let pt_addr = ym_client.get_principal_token();
        let yt_addr = ym_client.get_yield_token();

        let amm_wasm_hash = BytesN::from_array(&env, &AMM_WASM_HASH);

        // Deploy PT/Vault AMM pool
        // Ensure proper ordering: token_a < token_b
        let (pt_pool_token_a, pt_pool_token_b) = if pt_addr < vault {
            (pt_addr.clone(), vault.clone())
        } else {
            (vault.clone(), pt_addr.clone())
        };

        let pt_pool_salt = BytesN::from_array(&env, &[2u8; 32]);
        let pt_pool_addr = env
            .deployer()
            .with_current_contract(pt_pool_salt)
            .deploy_v2(
                amm_wasm_hash.clone(),
                (pt_pool_token_a, pt_pool_token_b),
            );

        // Deploy YT/Vault AMM pool
        // Ensure proper ordering: token_a < token_b
        let (yt_pool_token_a, yt_pool_token_b) = if yt_addr < vault {
            (yt_addr.clone(), vault.clone())
        } else {
            (vault.clone(), yt_addr.clone())
        };

        let yt_pool_salt = BytesN::from_array(&env, &[3u8; 32]);
        let yt_pool_addr = env
            .deployer()
            .with_current_contract(yt_pool_salt)
            .deploy_v2(
                amm_wasm_hash,
                (yt_pool_token_a, yt_pool_token_b),
            );

        (pt_pool_addr, yt_pool_addr)
    }
}

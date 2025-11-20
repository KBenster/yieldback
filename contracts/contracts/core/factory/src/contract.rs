use soroban_sdk::{Address, BytesN, Env, String};
use crate::storage;
use yield_manager_interface::YieldManagerClient;

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

const PT_WASM_HASH: [u8; 32] = [0u8; 32];
const YT_WASM_HASH: [u8; 32] = [0u8; 32];
const YM_WASM_HASH: [u8; 32] = [0u8; 32];

pub trait FactoryTrait {
    fn __constructor(env: Env, admin: Address);

    fn deploy_yield_manager(
        env: Env,
        vault: Address,
        maturity: u64,
    ) -> Address;

    fn deploy_liquidity_pools(env: Env, asset1: Address, asset2: Address) -> (Address, Address);
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
                    admin.clone(),
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

    fn deploy_liquidity_pools(env: Env, asset1: Address, asset2: Address) -> (Address, Address) {
        // TODO: Replace the placeholder
        (asset1.clone(), asset2.clone())
    }
}

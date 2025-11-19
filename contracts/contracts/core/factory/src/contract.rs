use soroban_sdk::{Address, BytesN, Env};
use crate::storage;
use yield_manager::YieldManagerClient;

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl};

// TODO: Replace these with actual WASM hashes after deployment
const PT_WASM_HASH: [u8; 32] = [0u8; 32]; // PLACEHOLDER - Replace with actual PT WASM hash
const YT_WASM_HASH: [u8; 32] = [0u8; 32]; // PLACEHOLDER - Replace with actual YT WASM hash
const YM_WASM_HASH: [u8; 32] = [0u8; 32]; // PLACEHOLDER - Replace with actual YM WASM hash

pub trait FactoryTrait {
    fn __constructor(env: Env, admin: Address);

    fn deploy_yield_manager(
        env: Env,
        vault: Address,
        maturity: u64,
    ) -> Address;
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

        // Deploy yield manager
        // Use a unique salt based on vault address and maturity
        let mut salt_data = [0u8; 32];
        // Simple salt derivation - could be made more sophisticated
        let salt = BytesN::from_array(&env, &salt_data);

        let ym_addr = env
            .deployer()
            .with_current_contract(salt)
            .deploy_v2(
                ym_wasm_hash,
                (
                    admin.clone(),
                    vault,
                    maturity,
                    pt_wasm_hash,
                    yt_wasm_hash,
                ),
            );

        ym_addr
    }
}

use soroban_sdk::{Address, Bytes, BytesN, Env, String};
use crate::escrow::DataKey;

pub fn deploy_sy_token(env: &Env, admin: Address, name: String, symbol: String) -> Address {
    admin.require_auth();

    let sy_wasm_hash: BytesN<32> = env.storage().instance()
        .get(&DataKey::SYWasmHash)
        .expect("Not initialized");

    let maturity_date: u64 = env.storage().instance()
        .get(&DataKey::MaturityDate)
        .expect("Not initialized");

    // Create salt from maturity date
    let mut salt_bytes = Bytes::new(&env);
    salt_bytes.extend_from_array(&maturity_date.to_be_bytes());
    let salt = env.crypto().keccak256(&salt_bytes);

    // Deploy StandardizedYield token contract
    let sy_token_address = env.deployer().with_current_contract(salt)
        .deploy_v2(sy_wasm_hash, (admin, name, symbol));

    env.storage().instance().set(&DataKey::SYToken, &sy_token_address);

    sy_token_address
}

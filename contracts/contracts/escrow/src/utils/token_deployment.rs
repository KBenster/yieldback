use soroban_sdk::{Address, Bytes, BytesN, Env, String};

pub fn deploy_sy_token(env: &Env, admin: Address, wasm_hash: BytesN<32>, name: String, symbol: String, maturity_date: u64) -> Address {
    // Create salt from maturity date
    let mut salt_bytes = Bytes::new(&env);
    salt_bytes.extend_from_array(&maturity_date.to_be_bytes());
    let salt = env.crypto().keccak256(&salt_bytes);

    // Deploy StandardizedYield token contract
    env.deployer().with_current_contract(salt)
        .deploy_v2(wasm_hash, (admin, name, symbol))
}

pub fn deploy_pt_token(env: &Env, admin: Address, wasm_hash: BytesN<32>, name: String, symbol: String, maturity_date: u64) -> Address {
    // Create salt from maturity date with a different prefix to avoid collision
    let mut salt_bytes = Bytes::new(&env);
    salt_bytes.extend_from_array(&[0xFF]); // Prefix to differentiate from SY token
    salt_bytes.extend_from_array(&maturity_date.to_be_bytes());
    let salt = env.crypto().keccak256(&salt_bytes);

    // Deploy PrincipalToken contract
    env.deployer().with_current_contract(salt)
        .deploy_v2(wasm_hash, (admin, name, symbol))
}

pub fn deploy_yt_token(env: &Env, admin: Address, wasm_hash: BytesN<32>, name: String, symbol: String, maturity_date: u64) -> Address {
    // Create salt from maturity date with a different prefix to avoid collision
    let mut salt_bytes = Bytes::new(&env);
    salt_bytes.extend_from_array(&[0xFE]); // Prefix to differentiate from SY and PT tokens
    salt_bytes.extend_from_array(&maturity_date.to_be_bytes());
    let salt = env.crypto().keccak256(&salt_bytes);

    // Deploy YieldToken contract
    env.deployer().with_current_contract(salt)
        .deploy_v2(wasm_hash, (admin, name, symbol))
}

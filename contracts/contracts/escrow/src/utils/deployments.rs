use soroban_sdk::{Address, Bytes, BytesN, Env, String};

pub struct DeploymentAddresses {
    pub adapter: Address,
    pub sy_token: Address,
    pub pt_token: Address,
    pub yt_token: Address,
}

pub fn deploy_all(
    env: &Env,
    escrow: Address,
    yield_source: Address,
    token: Address,
    adapter_wasm_hash: BytesN<32>,
    sy_wasm_hash: BytesN<32>,
    pt_wasm_hash: BytesN<32>,
    yt_wasm_hash: BytesN<32>,
    maturity_date: u64,
    sy_name: String,
    sy_symbol: String,
    pt_name: String,
    pt_symbol: String,
    yt_name: String,
    yt_symbol: String,
) -> DeploymentAddresses {
    // Deploy adapter contract
    let adapter = deploy_adapter(env, adapter_wasm_hash, escrow.clone(), yield_source, token);

    // Deploy SY token
    let sy_token = deploy_sy_token(env, escrow.clone(), sy_wasm_hash, sy_name, sy_symbol, maturity_date);

    // Deploy PT token
    let pt_token = deploy_pt_token(env, escrow.clone(), pt_wasm_hash, pt_name, pt_symbol, maturity_date);

    // Deploy YT token
    let yt_token = deploy_yt_token(env, escrow, yt_wasm_hash, yt_name, yt_symbol, maturity_date);

    DeploymentAddresses {
        adapter,
        sy_token,
        pt_token,
        yt_token,
    }
}

pub fn deploy_adapter(env: &Env, wasm_hash: BytesN<32>, escrow: Address, yield_protocol: Address, token: Address) -> Address {
    // Create salt for adapter deployment
    let mut salt_bytes = Bytes::new(&env);
    salt_bytes.extend_from_array(&[0xAD]); // Prefix for adapter
    let salt = env.crypto().keccak256(&salt_bytes);

    // Deploy adapter contract - __constructor is automatically called with the init args
    env.deployer().with_current_contract(salt)
        .deploy_v2(wasm_hash, (escrow, yield_protocol, token))
}

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

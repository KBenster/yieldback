use soroban_sdk::{contract, contractimpl, contracttype, token, Address, BytesN, Env, IntoVal};
use crate::utils::deployments;
use adapter_trait::YieldAdapterClient;

#[contracttype]
pub enum DataKey { // TODO: Is storing WASMs like this good practice? We'll find out eventually I guess
    BlendPool,
    Token,
    SYToken,
    SYWasmHash,
    PTToken,
    PTWasmHash,
    YTToken,
    YTWasmHash,
    MaturityDate,
    Adapter,
    AdapterWasmHash,
}

pub trait Escrow {
    fn deposit(env: Env, user: Address, amount: i128);
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    pub fn get_sy_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::SYToken).expect("SY token not deployed")
    }

    pub fn get_pt_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::PTToken).expect("PT token not deployed")
    }

    pub fn get_yt_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::YTToken).expect("YT token not deployed")
    }

    pub fn __constructor(
        env: Env,
        admin: Address,
        blend_pool: Address,
        token: Address,
        sy_wasm_hash: BytesN<32>,
        pt_wasm_hash: BytesN<32>,
        yt_wasm_hash: BytesN<32>,
        adapter_wasm_hash: BytesN<32>,
        maturity_date: u64,
    ) {
        admin.require_auth();

        env.storage().instance().set(&DataKey::BlendPool, &blend_pool);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::MaturityDate, &maturity_date);

        // Deploy adapter contract
        let adapter_address = deployments::deploy_adapter(&env, adapter_wasm_hash, blend_pool.clone(), token.clone());
        env.storage().instance().set(&DataKey::Adapter, &adapter_address);

        // Deploy SY token
        let sy_name = soroban_sdk::String::from_str(&env, "TODO");
        let sy_symbol = soroban_sdk::String::from_str(&env, "TODO");
        let sy_token_address = deployments::deploy_sy_token(&env, env.current_contract_address(), sy_wasm_hash, sy_name, sy_symbol, maturity_date);
        env.storage().instance().set(&DataKey::SYToken, &sy_token_address);

        // Deploy PT token
        let pt_name = soroban_sdk::String::from_str(&env, "TODO");
        let pt_symbol = soroban_sdk::String::from_str(&env, "TODO");
        let pt_token_address = deployments::deploy_pt_token(&env, env.current_contract_address(), pt_wasm_hash, pt_name, pt_symbol, maturity_date);
        env.storage().instance().set(&DataKey::PTToken, &pt_token_address);

        // Deploy YT token
        let yt_name = soroban_sdk::String::from_str(&env, "TODO");
        let yt_symbol = soroban_sdk::String::from_str(&env, "TODO");
        let yt_token_address = deployments::deploy_yt_token(&env, env.current_contract_address(), yt_wasm_hash, yt_name, yt_symbol, maturity_date);
        env.storage().instance().set(&DataKey::YTToken, &yt_token_address);
    }
}

#[contractimpl]
impl Escrow for EscrowContract {
    fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("Amount must be positive");
        }

        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let adapter_address: Address = env.storage().instance()
            .get(&DataKey::Adapter)
            .expect("Not initialized");

        let sy_token_address: Address = env.storage().instance()
            .get(&DataKey::SYToken)
            .expect("Not initialized");

        let pt_token_address: Address = env.storage().instance()
            .get(&DataKey::PTToken)
            .expect("Not initialized");

        let yt_token_address: Address = env.storage().instance()
            .get(&DataKey::YTToken)
            .expect("Not initialized");

        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from user to escrow contract
        token.transfer(&user, &env.current_contract_address(), &amount);

        // Deposit into yield protocol via adapter (this will transfer from escrow to adapter to pool)
        let adapter_client = YieldAdapterClient::new(&env, &adapter_address);
        adapter_client.deposit(&env.current_contract_address(), &amount);

        // syAmount = assetAmount / exchangeRate
        let exchange_rate: i128 = 1; // At the start, 1 SY = 1 asset. As yield accrues, the exchange rate increases proportionally.
        let sy_amount = amount / exchange_rate;

        // Mint SY tokens to the escrow contract
        env.invoke_contract::<()>(
            &sy_token_address,
            &soroban_sdk::symbol_short!("mint"),
            (env.current_contract_address(), sy_amount).into_val(&env)
        );

        // Mint PT tokens based on SY quantity * index
        let index: i128 = 1; // TODO: Replace with actual index calculation
        let pt_amount = sy_amount * index;
        env.invoke_contract::<()>(
            &pt_token_address,
            &soroban_sdk::symbol_short!("mint"),
            (user.clone(), pt_amount).into_val(&env)
        );

        // Mint YT tokens in the same way and quantity as PT tokens
        env.invoke_contract::<()>(
            &yt_token_address,
            &soroban_sdk::symbol_short!("mint"),
            (user.clone(), pt_amount).into_val(&env)
        );
    }
}

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, BytesN, Env};
use crate::utils::deployments;
use adapter_trait::YieldAdapterClient;

mod standardized_yield {
    soroban_sdk::contractimport!(
        file = "../../wasms/standardized_yield.wasm"
    );
}

mod principal_token {
    soroban_sdk::contractimport!(
        file = "../../wasms/principal_token.wasm"
    );
}

mod yield_token {
    soroban_sdk::contractimport!(
        file = "../../wasms/yield_token.wasm"
    );
}

use standardized_yield::Client as StandardizedYieldClient;
use principal_token::Client as PrincipalTokenClient;
use yield_token::Client as YieldTokenClient;

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

    pub fn get_adapter(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Adapter).expect("Adapter not deployed")
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

        // Deploy all contracts
        let addresses = deployments::deploy_all(
            &env,
            env.current_contract_address(),
            blend_pool,
            token,
            adapter_wasm_hash,
            sy_wasm_hash,
            pt_wasm_hash,
            yt_wasm_hash,
            maturity_date,
        );

        env.storage().instance().set(&DataKey::Adapter, &addresses.adapter);
        env.storage().instance().set(&DataKey::SYToken, &addresses.sy_token);
        env.storage().instance().set(&DataKey::PTToken, &addresses.pt_token);
        env.storage().instance().set(&DataKey::YTToken, &addresses.yt_token);
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

        let token = token::Client::new(&env, &token_address);

        // Transfer tokens from user to escrow contract
        token.transfer(&user, &env.current_contract_address(), &amount);

        // Deposit into yield protocol via adapter (this will transfer from escrow to adapter to pool)
        let adapter_client = YieldAdapterClient::new(&env, &adapter_address);
        adapter_client.deposit(&env.current_contract_address(), &amount);

        // syAmount = assetAmount / exchangeRate
        let sy_amount = amount / Self::get_current_exchange_index(env.clone());

        // Mint SY tokens to the escrow contract
        Self::mint_sy(env.clone(), sy_amount);

        // Mint PT and YT tokens to the user
        Self::mint_pt_and_yt(env, user, sy_amount);
    }
}

#[contractimpl]
impl EscrowContract {
    /// Get the current exchange index
    /// Formula: PY Index = total_assets / total_shares
    /// Initially returns 1 when no shares exist
    pub fn get_current_exchange_index(env: Env) -> i128 {
        let adapter_address: Address = env.storage().instance()
            .get(&DataKey::Adapter)
            .expect("Not initialized");

        let sy_token_address: Address = env.storage().instance()
            .get(&DataKey::SYToken)
            .expect("Not initialized");

        // Get total assets from adapter
        let adapter_client = YieldAdapterClient::new(&env, &adapter_address);
        let total_assets = adapter_client.get_assets();

        // Get total shares (SY token supply)
        let sy_client = StandardizedYieldClient::new(&env, &sy_token_address);
        let total_shares = sy_client.total_supply();

        // If no shares exist yet, return initial index of 1
        if total_shares == 0 {
            return 1;
        }

        // Calculate exchange index: total_assets / total_shares
        total_assets / total_shares
    }

    /// Mints SY tokens to the escrow contract
    fn mint_sy(env: Env, sy_amount: i128) {
        let sy_token_address: Address = env.storage().instance()
            .get(&DataKey::SYToken)
            .expect("Not initialized");

        let sy_client = StandardizedYieldClient::new(&env, &sy_token_address);
        sy_client.mint(&env.current_contract_address(), &sy_amount);
    }

    /// Mints PT and YT tokens to the user based on the SY amount
    fn mint_pt_and_yt(env: Env, user: Address, sy_amount: i128) {
        let pt_token_address: Address = env.storage().instance()
            .get(&DataKey::PTToken)
            .expect("Not initialized");

        let yt_token_address: Address = env.storage().instance()
            .get(&DataKey::YTToken)
            .expect("Not initialized");

        // Mint PT tokens based on SY quantity * index
        let pt_amount = sy_amount * Self::get_current_exchange_index(env.clone()); // Interchangeable for PT and YT quantities

        let pt_client = PrincipalTokenClient::new(&env, &pt_token_address);
        pt_client.mint(&user, &pt_amount);

        let yt_client = YieldTokenClient::new(&env, &yt_token_address);
        yt_client.mint(&user, &pt_amount); // These should be interchangeable
    }

    /// PT Redemption
    pub fn redeem_principal(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let maturity_date: u64 = env.storage().instance()
            .get(&DataKey::MaturityDate)
            .expect("Not initialized");

        if env.ledger().timestamp() < maturity_date {
            panic!("Cannot redeem before maturity");
        }

        let pt_token_address: Address = env.storage().instance()
            .get(&DataKey::PTToken)
            .expect("Not initialized");

        let pt_client = PrincipalTokenClient::new(&env, &pt_token_address);
        pt_client.burn(&user, &amount);

        // Calculate SY amount to redeem: PT amount / yield index
        let sy_amount = amount / Self::get_current_exchange_index(env.clone());

        // Calculate underlying asset amount to withdraw: SY amount * exchange rate
        let withdraw_amount = sy_amount * Self::get_current_exchange_index(env.clone());

        // Withdraw from the adapter
        let adapter_address: Address = env.storage().instance()
            .get(&DataKey::Adapter)
            .expect("Not initialized");

        let adapter_client = YieldAdapterClient::new(&env, &adapter_address);
        adapter_client.withdraw(&withdraw_amount);
    }
}

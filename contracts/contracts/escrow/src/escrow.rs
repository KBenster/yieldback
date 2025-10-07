use soroban_sdk::{contract, contractclient, contractimpl, contracttype, token, Address, BytesN, Env, String};
use crate::utils::deployments;
use adapter_trait::YieldAdapterClient;

// Scaling factor for exchange index calculations (matches token decimals)
const INDEX_SCALE: i128 = 10_000_000; // 1e7

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
pub struct MarketInitMeta {
    pub yield_source: Address,
    pub token: Address,
    pub sy_wasm_hash: BytesN<32>,
    pub pt_wasm_hash: BytesN<32>,
    pub yt_wasm_hash: BytesN<32>,
    pub adapter_wasm_hash: BytesN<32>,
}

#[contracttype]
pub enum DataKey {
    MarketMeta,
    MaturityDate,
    Adapter,
    SYToken,
    PTToken,
    YTToken,
    YieldSource,
    Token,
    IsDeployed,
}

/// Core escrow interface for Pendle-style yield tokenization
/// This trait defines the public API that other contracts can use to interact with the escrow
#[contractclient(name = "EscrowClient")]
pub trait Escrow {
    /// Initialize the escrow contract with admin and market metadata
    /// Only stores WASM hashes - deployment happens separately
    fn __constructor(env: Env, admin: Address, market_meta: MarketInitMeta);

    /// Deploy the market with the specified maturity date and token metadata
    /// Can only be called once per escrow instance
    fn deploy_market(
        env: Env,
        admin: Address,
        maturity_date: u64,
        sy_name: String,
        sy_symbol: String,
        pt_name: String,
        pt_symbol: String,
        yt_name: String,
        yt_symbol: String,
    );

    /// Deposit tokens and receive PT + YT tokens
    /// User receives PT and YT tokens representing principal and yield
    fn deposit(env: Env, user: Address, amount: i128);

    /// Redeem YT token interest for a user
    /// Returns the amount of interest redeemed in SY tokens
    fn redeem_yt_interest(env: Env, user: Address) -> i128;

    /// Redeem PT tokens for underlying assets after maturity
    /// Can only be called after the maturity date
    fn redeem_principal(env: Env, user: Address, amount: i128);

    /// Get the current exchange index (scaled by INDEX_SCALE)
    /// Formula: PY Index = (total_assets * INDEX_SCALE) / total_shares
    fn get_current_exchange_index(env: Env) -> i128;

    /// Get accrued interest for a user (view function)
    fn get_user_accrued_interest(env: Env, user: Address) -> i128;

    /// Get the Standardized Yield token address
    fn get_sy_token(env: Env) -> Address;

    /// Get the Principal Token address
    fn get_pt_token(env: Env) -> Address;

    /// Get the Yield Token address
    fn get_yt_token(env: Env) -> Address;

    /// Get the adapter contract address
    fn get_adapter(env: Env) -> Address;

    /// Check if the market has been deployed
    fn is_deployed(env: Env) -> bool;
    #[doc = " Mints PT and YT tokens to the user based on the SY amount"]

    fn mint_pt_and_yt(env: Env, user: Address, sy_amount: i128);
    #[doc = " Mints SY tokens to the escrow contract"]

    fn mint_sy(env: Env, sy_amount: i128);
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl Escrow for EscrowContract {
    fn __constructor(env: Env, admin: Address, market_meta: MarketInitMeta) {
        admin.require_auth();
        env.storage().instance().set(&DataKey::MarketMeta, &market_meta);
        env.storage().instance().set(&DataKey::IsDeployed, &false);
    }

    fn deploy_market(
        env: Env,
        admin: Address,
        maturity_date: u64,
        sy_name: String,
        sy_symbol: String,
        pt_name: String,
        pt_symbol: String,
        yt_name: String,
        yt_symbol: String,
    ) {
        admin.require_auth();

        // Check if already deployed
        let is_deployed: bool = env.storage().instance()
            .get(&DataKey::IsDeployed)
            .unwrap_or(false);

        if is_deployed {
            panic!("Market already deployed");
        }

        // Get stored metadata
        let market_meta: MarketInitMeta = env.storage().instance()
            .get(&DataKey::MarketMeta)
            .expect("Not initialized");

        // Store market configuration
        env.storage().instance().set(&DataKey::MaturityDate, &maturity_date);
        env.storage().instance().set(&DataKey::YieldSource, &market_meta.yield_source);
        env.storage().instance().set(&DataKey::Token, &market_meta.token);

        // Deploy all contracts with provided names
        let addresses = deployments::deploy_all(
            &env,
            env.current_contract_address(),
            market_meta.yield_source,
            market_meta.token,
            market_meta.adapter_wasm_hash,
            market_meta.sy_wasm_hash,
            market_meta.pt_wasm_hash,
            market_meta.yt_wasm_hash,
            maturity_date,
            sy_name,
            sy_symbol,
            pt_name,
            pt_symbol,
            yt_name,
            yt_symbol,
        );

        // Store deployed addresses
        env.storage().instance().set(&DataKey::Adapter, &addresses.adapter);
        env.storage().instance().set(&DataKey::SYToken, &addresses.sy_token);
        env.storage().instance().set(&DataKey::PTToken, &addresses.pt_token);
        env.storage().instance().set(&DataKey::YTToken, &addresses.yt_token);
        env.storage().instance().set(&DataKey::IsDeployed, &true);
    }

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

        // syAmount = (assetAmount * INDEX_SCALE) / exchangeRate
        // Since exchangeRate is already scaled, we get the correct result
        let exchange_index = Self::get_current_exchange_index(env.clone());
        let sy_amount = (amount * INDEX_SCALE) / exchange_index;

        // Mint SY tokens to the escrow contract
        Self::mint_sy(env.clone(), sy_amount);

        // Mint PT and YT tokens to the user
        Self::mint_pt_and_yt(env, user, sy_amount);
    }

    fn redeem_yt_interest(env: Env, user: Address) -> i128 {
        user.require_auth();

        let yt_token_address: Address = env.storage().instance()
            .get(&DataKey::YTToken)
            .expect("YT token not deployed");

        // Get and clear the accrued interest from YT contract
        let yt_client = YieldTokenClient::new(&env, &yt_token_address);
        let interest_amount = yt_client.redeem_interest(&user);

        if interest_amount <= 0 {
            return 0;
        }

        // Withdraw from adapter to cover the interest
        let adapter_address: Address = env.storage().instance()
            .get(&DataKey::Adapter)
            .expect("Not initialized");

        let adapter_client = YieldAdapterClient::new(&env, &adapter_address);
        adapter_client.withdraw(&interest_amount);

        // Transfer SY tokens to user
        let sy_token_address: Address = env.storage().instance()
            .get(&DataKey::SYToken)
            .expect("Not initialized");

        let sy_client = StandardizedYieldClient::new(&env, &sy_token_address);
        sy_client.transfer(&env.current_contract_address(), &user, &interest_amount);

        interest_amount
    }

    fn get_user_accrued_interest(env: Env, user: Address) -> i128 {
        let yt_token_address: Address = env.storage().instance()
            .get(&DataKey::YTToken)
            .expect("YT token not deployed");

        let yt_client = YieldTokenClient::new(&env, &yt_token_address);
        yt_client.get_user_interest(&user).accrued
    }

    fn get_current_exchange_index(env: Env) -> i128 {
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

        // If no shares exist yet, return initial index of INDEX_SCALE (representing 1.0)
        if total_shares == 0 {
            return INDEX_SCALE;
        }

        // Calculate exchange index: (total_assets * INDEX_SCALE) / total_shares
        // This preserves precision by scaling the result
        (total_assets * INDEX_SCALE) / total_shares
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

        // Mint PT tokens based on SY quantity * index / INDEX_SCALE
        // Since exchange index is scaled, we need to divide by INDEX_SCALE to get actual amount
        let exchange_index = Self::get_current_exchange_index(env.clone());
        let pt_amount = (sy_amount * exchange_index) / INDEX_SCALE;

        let pt_client = PrincipalTokenClient::new(&env, &pt_token_address);
        pt_client.mint(&user, &pt_amount);

        let yt_client = YieldTokenClient::new(&env, &yt_token_address);
        yt_client.mint(&user, &pt_amount); // These should be interchangeable
    }

    fn redeem_principal(env: Env, user: Address, amount: i128) {
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

        let exchange_index = Self::get_current_exchange_index(env.clone());

        // Calculate SY amount to redeem: (PT amount * INDEX_SCALE) / yield index
        let sy_amount = (amount * INDEX_SCALE) / exchange_index;

        // Calculate underlying asset amount to withdraw: (SY amount * exchange rate) / INDEX_SCALE
        let withdraw_amount = (sy_amount * exchange_index) / INDEX_SCALE;

        // Withdraw from the adapter (transfers tokens to escrow contract)
        let adapter_address: Address = env.storage().instance()
            .get(&DataKey::Adapter)
            .expect("Not initialized");

        let adapter_client = YieldAdapterClient::new(&env, &adapter_address);
        adapter_client.withdraw(&withdraw_amount);

        // Transfer the withdrawn tokens from escrow to user
        let token_address: Address = env.storage().instance()
            .get(&DataKey::Token)
            .expect("Not initialized");

        let token = token::Client::new(&env, &token_address);
        token.transfer(&env.current_contract_address(), &user, &withdraw_amount);
    }

    fn get_sy_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::SYToken).expect("SY token not deployed")
    }

    fn get_pt_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::PTToken).expect("PT token not deployed")
    }

    fn get_yt_token(env: Env) -> Address {
        env.storage().instance().get(&DataKey::YTToken).expect("YT token not deployed")
    }

    fn get_adapter(env: Env) -> Address {
        env.storage().instance().get(&DataKey::Adapter).expect("Adapter not deployed")
    }

    fn is_deployed(env: Env) -> bool {
        env.storage().instance()
            .get(&DataKey::IsDeployed)
            .unwrap_or(false)
    }
}

// Internal helper functions
#[contractimpl]
impl EscrowContract {
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

        // Mint PT tokens based on SY quantity * index / INDEX_SCALE
        // Since exchange index is scaled, we need to divide by INDEX_SCALE to get actual amount
        let exchange_index = EscrowContract::get_current_exchange_index(env.clone());
        let pt_amount = (sy_amount * exchange_index) / INDEX_SCALE;

        let pt_client = PrincipalTokenClient::new(&env, &pt_token_address);
        pt_client.mint(&user, &pt_amount);

        let yt_client = YieldTokenClient::new(&env, &yt_token_address);
        yt_client.mint(&user, &pt_amount); // These should be interchangeable
    }
}

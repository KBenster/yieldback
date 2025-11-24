#![no_std]

use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, token, vec, Address, Env, IntoVal, Symbol, Vec,
};
use vault_strategy_traits::IStrategy;


// Import Blend pool contract
soroban_sdk::contractimport!(file = "../../../../wasms/pool_v2.0.0.wasm");
pub type BlendPoolClient<'a> = Client<'a>;

/// Blend Lending Strategy
///
/// This strategy deposits assets into Blend protocol to earn lending yield.
/// It handles deposits, withdrawals, and tracks total assets deployed in Blend.
#[contract]
pub struct BlendLendingStrategy;

/// Request types for Blend pool operations
#[repr(u32)]
pub enum RequestType {
    Supply = 0,
    Withdraw = 1,
}

impl RequestType {
    fn to_u32(self) -> u32 {
        self as u32
    }
}

#[contractimpl]
impl BlendLendingStrategy {
    /// Initialize the Blend lending strategy
    ///
    /// # Arguments
    /// * `vault` - The vault address that owns this strategy
    /// * `asset` - The underlying asset to lend
    /// * `blend_pool` - The Blend pool contract address
    pub fn __constructor(e: &Env, vault: Address, asset: Address, blend_pool: Address) {
        // Store the vault address that owns this strategy
        e.storage().instance().set(&"vault", &vault);

        // Store the asset address
        e.storage().instance().set(&"asset", &asset);

        // Store the Blend pool address
        e.storage().instance().set(&"blend_pool", &blend_pool);

        // Get and store the reserve ID from Blend pool
        let pool_client = BlendPoolClient::new(e, &blend_pool);
        let reserve = pool_client.get_reserve(&asset);
        let reserve_id = reserve.config.index;
        e.storage().instance().set(&"reserve_id", &reserve_id);
    }

    /// Get the Blend pool address
    fn get_blend_pool(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&"blend_pool")
            .expect("Blend pool not set")
    }

    /// Get the reserve ID for this asset in Blend
    fn get_reserve_id(e: &Env) -> u32 {
        e.storage()
            .instance()
            .get(&"reserve_id")
            .expect("Reserve ID not set")
    }

    /// Get the asset address
    fn get_asset(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&"asset")
            .expect("Asset not set")
    }

    /// Get the vault address
    fn get_vault(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&"vault")
            .expect("Vault not set")
    }

    /// Supply assets to Blend pool
    fn supply_to_blend(e: &Env, amount: i128) {
        let pool_address = Self::get_blend_pool(e);
        let asset_address = Self::get_asset(e);
        let pool_client = BlendPoolClient::new(e, &pool_address);

        // Create supply request
        let requests: Vec<Request> = vec![
            e,
            Request {
                address: asset_address.clone(),
                amount,
                request_type: RequestType::Supply.to_u32(),
            },
        ];

        // Authorize the asset transfer from strategy to pool
        e.authorize_as_current_contract(vec![
            e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: asset_address,
                    fn_name: Symbol::new(e, "transfer"),
                    args: (
                        e.current_contract_address(),
                        pool_address.clone(),
                        amount,
                    )
                        .into_val(e),
                },
                sub_invocations: vec![e],
            }),
        ]);

        // Submit the supply request to Blend
        pool_client.submit(
            &e.current_contract_address(),
            &e.current_contract_address(),
            &e.current_contract_address(),
            &requests,
        );
    }

    /// Withdraw assets from Blend pool
    fn withdraw_from_blend(e: &Env, amount: i128, recipient: &Address) {
        let pool_address = Self::get_blend_pool(e);
        let asset_address = Self::get_asset(e);
        let pool_client = BlendPoolClient::new(e, &pool_address);

        // Create withdrawal request
        let requests: Vec<Request> = vec![
            e,
            Request {
                address: asset_address,
                amount,
                request_type: RequestType::Withdraw.to_u32(),
            },
        ];

        // Submit the withdrawal request to Blend
        // The assets will be transferred to the recipient
        pool_client.submit(
            &e.current_contract_address(),
            &e.current_contract_address(),
            recipient,
            &requests,
        );
    }

    /// Get the current supply position in Blend
    fn get_blend_position(e: &Env) -> i128 {
        let pool_address = Self::get_blend_pool(e);
        let reserve_id = Self::get_reserve_id(e);
        let pool_client = BlendPoolClient::new(e, &pool_address);

        // Get positions from Blend pool
        let positions = pool_client.get_positions(&e.current_contract_address());

        // Get supply amount for this reserve
        positions
            .supply
            .get(reserve_id)
            .unwrap_or(0)
    }

    /// Convert bTokens to underlying assets using Blend's exchange rate
    fn b_tokens_to_assets(e: &Env, b_tokens: i128) -> i128 {
        let pool_address = Self::get_blend_pool(e);
        let asset_address = Self::get_asset(e);
        let pool_client = BlendPoolClient::new(e, &pool_address);

        // Get the reserve data to access b_rate
        let reserve = pool_client.get_reserve(&asset_address);
        let b_rate = reserve.data.b_rate;

        // Convert bTokens to assets: assets = bTokens * b_rate / SCALAR_12
        // SCALAR_12 = 1_000_000_000_000 (12 decimals)
        const SCALAR_12: i128 = 1_000_000_000_000;

        b_tokens
            .checked_mul(b_rate)
            .and_then(|v| v.checked_div(SCALAR_12))
            .unwrap_or(0)
    }
}

#[contractimpl]
impl IStrategy for BlendLendingStrategy {
    /// Deploy assets into Blend lending pool
    ///
    /// # Arguments
    /// * `assets` - Amount of underlying assets to deploy
    ///
    /// # Returns
    /// Amount of assets actually deployed
    fn deposit(e: Env, assets: i128) -> i128 {
        if assets <= 0 {
            return 0;
        }

        // Supply the assets to Blend
        // Note: The vault has already transferred assets to this contract
        BlendLendingStrategy::supply_to_blend(&e, assets);

        assets
    }

    /// Withdraw assets from Blend lending pool
    ///
    /// # Arguments
    /// * `assets` - Amount of underlying assets to withdraw
    ///
    /// # Returns
    /// Amount of assets actually withdrawn
    fn withdraw(e: Env, assets: i128) -> i128 {
        // Only the vault can withdraw
        let vault_addr = BlendLendingStrategy::get_vault(&e);
        vault_addr.require_auth();

        if assets <= 0 {
            return 0;
        }

        // Withdraw from Blend and send directly to vault
        BlendLendingStrategy::withdraw_from_blend(&e, assets, &vault_addr);

        assets
    }

    /// Get total assets managed by this strategy
    ///
    /// This includes both the Blend position (converted to underlying assets)
    /// and any idle assets held by the strategy contract.
    ///
    /// # Returns
    /// Total asset value in the strategy
    fn total_assets(e: Env) -> i128 {
        let asset_addr = BlendLendingStrategy::get_asset(&e);
        let asset_client = token::Client::new(&e, &asset_addr);

        // Get idle assets held by strategy
        let idle_assets = asset_client.balance(&e.current_contract_address());

        // Get Blend position in bTokens
        let b_tokens = BlendLendingStrategy::get_blend_position(&e);

        // Convert bTokens to underlying assets
        let blend_assets = BlendLendingStrategy::b_tokens_to_assets(&e, b_tokens);

        // Total = idle + deployed in Blend
        idle_assets
            .checked_add(blend_assets)
            .unwrap_or(idle_assets)
    }

    /// Get the vault address that owns this strategy
    ///
    /// # Returns
    /// Address of the vault that controls this strategy
    fn vault(e: Env) -> Address {
        BlendLendingStrategy::get_vault(&e)
    }
}
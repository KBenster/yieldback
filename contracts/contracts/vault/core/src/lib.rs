#![no_std]

#[cfg(feature = "contract")]
use soroban_sdk::{contract, contractimpl, token, Address, Env, String};
#[cfg(feature = "contract")]
use stellar_macros::default_impl;
#[cfg(feature = "contract")]
use stellar_tokens::{
    fungible::{Base, FungibleToken},
    vault::{FungibleVault, Vault},
};
#[cfg(feature = "contract")]
use vault_strategy_traits::StrategyClient;

pub use vault_interface::VaultTrait;

#[cfg(feature = "contract")]
#[contract]
pub struct VaultContract;

// Implement the interface trait for client generation
#[cfg(feature = "contract")]
#[contractimpl]
impl VaultTrait for VaultContract {
    fn __constructor(e: Env, asset: Address, decimals_offset: u32, strategy: Address) {
        Self::__constructor_impl(&e, asset, decimals_offset, strategy);
    }

    fn exchange_rate(e: Env) -> i128 {
        Self::exchange_rate_impl(&e)
    }
}

#[cfg(feature = "contract")]
#[contractimpl]
impl VaultContract {
    fn __constructor_impl(e: &Env, asset: Address, decimals_offset: u32, strategy: Address) {
        // Set the underlying asset address (immutable after initialization)
        Vault::set_asset(e, asset.clone());

        // Set the decimals offset for precision (immutable after initialization)
        Vault::set_decimals_offset(e, decimals_offset);

        // Set the strategy address (immutable after initialization)
        e.storage().instance().set(&"strategy", &strategy);

        // Get the underlying asset's symbol for vault naming
        let asset_client = token::Client::new(e, &asset);
        let asset_symbol = asset_client.symbol();

        // Create vault token name and symbol using the asset symbol
        // We'll use a simplified approach since Soroban strings don't support push_str
        // For now, use the asset symbol directly with a prefix in the name
        let vault_name = String::from_str(e, "Vault Share Token");

        // For symbol, we'll just use the asset symbol as-is for simplicity
        // TODO: Implement proper string concatenation when needed
        let vault_symbol = asset_symbol;

        // Initialize token metadata
        // Note: Vault overrides the decimals function, so set offset first
        Base::set_metadata(
            e,
            Vault::decimals(e),
            vault_name,
            vault_symbol,
        );
    }

    /// Get the strategy address
    fn get_strategy(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&"strategy")
            .expect("Strategy not set")
    }

    /// Get the exchange rate (assets per share)
    fn exchange_rate_impl(e: &Env) -> i128 {
        Vault::convert_to_assets(e, 1i128)
    }
}
//TODO: this could be better probably i think, figure it out, maybe not
#[cfg(feature = "contract")]
#[default_impl]
#[cfg(feature = "contract")]
#[contractimpl]
impl FungibleToken for VaultContract {
    type ContractType = Vault;

    fn total_supply (e : & Env )-> i128 {Self :: ContractType :: total_supply (e )}
}

#[cfg(feature = "contract")]
#[contractimpl]
impl FungibleVault for VaultContract {
    fn query_asset(e: &Env) -> Address {
        Vault::query_asset(e)
    }

    fn total_assets(e: &Env) -> i128 {
        // Query the strategy for total assets (assets are held in the strategy)
        let strategy_addr = VaultContract::get_strategy(e);
        let strategy_client = StrategyClient::new(e, &strategy_addr);
        strategy_client.total_assets()
    }

    fn convert_to_shares(e: &Env, assets: i128) -> i128 {
        Vault::convert_to_shares(e, assets)
    }

    fn convert_to_assets(e: &Env, shares: i128) -> i128 {
        Vault::convert_to_assets(e, shares)
    }

    fn max_deposit(e: &Env, receiver: Address) -> i128 {
        Vault::max_deposit(e, receiver)
    }

    fn preview_deposit(e: &Env, assets: i128) -> i128 {
        Vault::preview_deposit(e, assets)
    }

    fn deposit(
        e: &Env,
        assets: i128,
        receiver: Address,
        from: Address,
        operator: Address,
    ) -> i128 {
        operator.require_auth();

        // Execute the vault deposit (transfers assets from user to vault, mints shares)
        let shares = Vault::deposit(e, assets, receiver, from, operator);

        // Immediately transfer the deposited assets to the strategy
        let asset_addr = Vault::query_asset(e);
        let strategy_addr = VaultContract::get_strategy(e);
        let asset_client = token::Client::new(e, &asset_addr);

        // Transfer from vault to strategy
        asset_client.transfer(&e.current_contract_address(), &strategy_addr, &assets);

        // Notify the strategy about the deposit
        let strategy_client = StrategyClient::new(e, &strategy_addr);
        strategy_client.deposit(&assets);

        shares
    }

    fn max_mint(e: &Env, receiver: Address) -> i128 {
        Vault::max_mint(e, receiver)
    }

    fn preview_mint(e: &Env, shares: i128) -> i128 {
        Vault::preview_mint(e, shares)
    }

    fn mint(
        e: &Env,
        shares: i128,
        receiver: Address,
        from: Address,
        operator: Address,
    ) -> i128 {
        operator.require_auth();

        // Execute the vault mint (transfers assets from user to vault, mints shares)
        let assets = Vault::mint(e, shares, receiver, from, operator);

        // Immediately transfer the deposited assets to the strategy
        let asset_addr = Vault::query_asset(e);
        let strategy_addr = VaultContract::get_strategy(e);
        let asset_client = token::Client::new(e, &asset_addr);

        // Transfer from vault to strategy
        asset_client.transfer(&e.current_contract_address(), &strategy_addr, &assets);

        // Notify the strategy about the deposit
        let strategy_client = StrategyClient::new(e, &strategy_addr);
        strategy_client.deposit(&assets);

        assets
    }

    fn max_withdraw(e: &Env, owner: Address) -> i128 {
        Vault::max_withdraw(e, owner)
    }

    fn preview_withdraw(e: &Env, assets: i128) -> i128 {
        Vault::preview_withdraw(e, assets)
    }

    fn withdraw(
        e: &Env,
        assets: i128,
        receiver: Address,
        owner: Address,
        operator: Address,
    ) -> i128 {
        operator.require_auth();

        // First, withdraw assets from the strategy to the vault
        let strategy_addr = VaultContract::get_strategy(e);
        let strategy_client = StrategyClient::new(e, &strategy_addr);
        strategy_client.withdraw(&assets);

        // Strategy should have transferred assets back to vault
        // Now execute the vault withdraw (burns shares, transfers assets to receiver)
        Vault::withdraw(e, assets, receiver, owner, operator)
    }

    fn max_redeem(e: &Env, owner: Address) -> i128 {
        Vault::max_redeem(e, owner)
    }

    fn preview_redeem(e: &Env, shares: i128) -> i128 {
        Vault::preview_redeem(e, shares)
    }

    fn redeem(
        e: &Env,
        shares: i128,
        receiver: Address,
        owner: Address,
        operator: Address,
    ) -> i128 {
        operator.require_auth();

        // Calculate how many assets these shares are worth
        let assets = Vault::convert_to_assets(e, shares);

        // First, withdraw assets from the strategy to the vault
        let strategy_addr = VaultContract::get_strategy(e);
        let strategy_client = StrategyClient::new(e, &strategy_addr);
        strategy_client.withdraw(&assets);

        // Strategy should have transferred assets back to vault
        // Now execute the vault redeem (burns shares, transfers assets to receiver)
        Vault::redeem(e, shares, receiver, owner, operator)
    }
}
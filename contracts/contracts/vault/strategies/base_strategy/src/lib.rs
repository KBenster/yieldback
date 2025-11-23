#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, Env};
use vault_strategy_traits::IStrategy;

/// Basic vault strategy
///
/// This is a base implementation that can be extended for specific yield strategies.
/// Implements the IStrategy trait to ensure compatibility with the vault.
#[contract]
pub struct BaseStrategy;

#[contractimpl]
impl BaseStrategy {
    /// Initialize the strategy
    pub fn __constructor(e: &Env, vault: Address, asset: Address) {
        // Store the vault address that owns this strategy
        e.storage().instance().set(&"vault", &vault);
        // Store the asset address
        e.storage().instance().set(&"asset", &asset);
    }

    /// Get the asset address
    fn get_asset(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&"asset")
            .expect("Asset not set")
    }
}

#[contractimpl]
impl IStrategy for BaseStrategy {
    /// Deploy assets into the strategy
    /// Called by the vault to invest assets
    /// The vault has already transferred the assets to this contract
    fn deposit(e: Env, assets: i128) -> i128 {
        // Base implementation: just hold the assets (vault already transferred them)
        // Override this in specific strategies to deploy assets for yield generation
        // Example: stake assets, provide liquidity, lend, etc.
        assets
    }

    /// Withdraw assets from the strategy
    /// Called by the vault to retrieve assets
    fn withdraw(e: Env, assets: i128) -> i128 {
        // Get the vault address (only vault can withdraw)
        let vault_addr: Address = e.storage().instance().get(&"vault").expect("Vault not set");
        vault_addr.require_auth();

        // Transfer assets back to vault
        let asset_addr = BaseStrategy::get_asset(&e);
        let asset_client = token::Client::new(&e, &asset_addr);
        asset_client.transfer(&e.current_contract_address(), &vault_addr, &assets);

        assets
    }

    /// Get total assets managed by this strategy
    fn total_assets(e: Env) -> i128 {
        // Return the balance of assets held by this strategy contract
        let asset_addr = BaseStrategy::get_asset(&e);
        let asset_client = token::Client::new(&e, &asset_addr);
        asset_client.balance(&e.current_contract_address())
    }

    /// Get the vault address that owns this strategy
    fn vault(e: Env) -> Address {
        e.storage()
            .instance()
            .get(&"vault")
            .unwrap()
    }
}

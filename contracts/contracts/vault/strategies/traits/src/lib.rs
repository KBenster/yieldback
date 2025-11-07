#![no_std]
use soroban_sdk::{contractclient, Address, Env};

/// Standard interface for vault strategies
///
/// All strategies must implement this trait to be compatible with the vault.
/// The vault interacts with strategies through this interface, allowing
/// plug-and-play strategy integration without vault modifications.
#[contractclient(name = "StrategyClient")]
pub trait IStrategy {
    /// Deploy assets into the strategy to generate yield
    ///
    /// # Arguments
    /// * `assets` - Amount of underlying assets to deploy
    ///
    /// # Returns
    /// Amount of assets actually deployed
    fn deposit(e: Env, assets: i128) -> i128;

    /// Withdraw assets from the strategy
    ///
    /// # Arguments
    /// * `assets` - Amount of underlying assets to withdraw
    ///
    /// # Returns
    /// Amount of assets actually withdrawn
    fn withdraw(e: Env, assets: i128) -> i128;

    /// Get total assets managed by this strategy (including accrued yield)
    ///
    /// # Returns
    /// Total asset value in the strategy
    fn total_assets(e: Env) -> i128;

    /// Get the vault address that owns this strategy
    ///
    /// # Returns
    /// Address of the vault that controls this strategy
    fn vault(e: Env) -> Address;
}

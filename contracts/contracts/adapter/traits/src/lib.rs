#![no_std]

use soroban_sdk::{contractclient, Address, Env};

/// Common interface that all adapter contracts must implement
/// Adapters bridge between the escrow contract and various yield-generating protocols
#[contractclient(name = "YieldAdapterClient")]
pub trait YieldAdapter {
    /// Initialize the adapter with the yield protocol address and token address
    fn __constructor(env: Env, yield_protocol: Address, token: Address);

    /// Receive funds from escrow and forward them to the underlying yield protocol
    ///
    /// # Arguments
    /// * `depositor` - The address depositing funds (typically the escrow contract)
    /// * `amount` - The amount to deposit
    fn deposit(env: Env, depositor: Address, amount: i128);

    /// Get the configured yield protocol address
    fn get_yield_protocol(env: Env) -> Address;

    /// Get the configured token address
    fn get_token(env: Env) -> Address;
}

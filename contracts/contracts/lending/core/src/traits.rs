use soroban_sdk::{Address, Env};
use crate::types::*;
use crate::errors::LendingError;

/// Core trait that all lending protocol adapters must implement
pub trait LendingProtocol {
    /// Lend tokens to the protocol
    fn lend(&self, env: &Env, token: &Address, amount: i128) -> Result<LendingResponse, LendingError>;

    /// Withdraw a specific amount from the protocol
    fn withdraw(&self, env: &Env, token: &Address, amount: i128) -> Result<WithdrawResponse, LendingError>;

    /// Withdraw all funds from the protocol
    fn withdraw_all(&self, env: &Env, token: &Address) -> Result<WithdrawResponse, LendingError>;

    /// Get current position/balance in the protocol
    fn get_position(&self, env: &Env, account: &Address, token: &Address) -> Result<Position, LendingError>;

    /// Check if the protocol is healthy/available
    fn is_protocol_healthy(&self, env: &Env) -> bool;

    /// Get the protocol identifier/name
    fn get_protocol_name(&self) -> &'static str;
}

/// Interface for the generic protocol lender contract
pub trait ProtocolLender {
    /// Initialize the lender with configuration
    fn initialize(env: Env, config: LenderConfig);

    /// Receive assets from external sources
    fn receive_asset(env: Env, from: Address, amount: i128);

    /// Lend all received assets to the configured protocol
    fn lend_to_protocol(env: Env) -> i128;

    /// Withdraw specific amount from protocol (admin only)
    fn withdraw_from_protocol(env: Env, amount: i128) -> i128;

    /// Withdraw all from protocol (admin only)
    fn withdraw_all_from_protocol(env: Env) -> i128;

    // Getters
    fn get_admin(env: Env) -> Address;
    fn get_token(env: Env) -> Address;
    fn get_protocol_address(env: Env) -> Address;
    fn get_expected_amount(env: Env) -> i128;
    fn get_received_amount(env: Env) -> i128;
    fn is_amount_received(env: Env) -> bool;
    fn is_lent_to_protocol(env: Env) -> bool;
    fn get_contract_balance(env: Env) -> i128;
    fn get_protocol_position(env: Env) -> Position;
}
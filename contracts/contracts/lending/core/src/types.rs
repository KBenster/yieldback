use soroban_sdk::{contracttype, Address};

/// Configuration for initializing a protocol lender
#[contracttype]
#[derive(Clone)]
pub struct LenderConfig {
    pub admin: Address,
    pub token_address: Address,
    pub protocol_address: Address,
    pub expected_amount: i128,
}

/// Response from lending operations
#[contracttype]
#[derive(Clone)]
pub struct LendingResponse {
    pub amount: i128,
    pub success: bool,
}

/// Response from withdrawal operations
#[contracttype]
#[derive(Clone)]
pub struct WithdrawResponse {
    pub amount: i128,
    pub success: bool,
}

/// Current position information in lending protocol
#[contracttype]
#[derive(Clone)]
pub struct Position {
    pub supplied_amount: i128,
    pub earned_interest: i128,
    pub total_value: i128,
}
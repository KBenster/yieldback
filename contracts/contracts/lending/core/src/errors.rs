use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LendingError {
    // Core contract errors
    AlreadyInitialized = 1,
    InvalidAmount = 2,
    InsufficientFunds = 3,
    InvalidAsset = 4,
    AmountMustBePositive = 5,
    ExpectedAmountNotMet = 6,
    Unauthorized = 7,

    // Lending protocol errors
    NoTokensToLend = 10,
    NoPositionInProtocol = 11,
    ProtocolError = 12,
    InvalidProtocolAddress = 13,
    LendingFailed = 14,
    WithdrawFailed = 15,

    // State errors
    NotInitialized = 20,
    InvalidState = 21,
}
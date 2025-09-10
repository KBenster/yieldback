use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum EscrowError {
    // Initialization errors
    AlreadyInitialized = 101,

    // Parameter validation errors
    InvalidCouponAmount = 102,
    InvalidPrincipalAmount = 103,
    MaturityInPast = 104,
    InvalidAmount = 105,

    // Deposit state errors
    CouponAlreadyDeposited = 106,
    PrincipalAlreadyDeposited = 107,

    // Balance and liquidity errors
    NoTokensToLend = 108,
    NoPositionInBlend = 109,
    InsufficientFundsInBlend = 110,
    InsufficientBalance = 111,
    InsufficientCouponBalance = 112,
    InsufficientPrincipalBalance = 113,
    InsufficientFundsAfterWithdrawal = 114,

    // Token supply errors
    NoPrincipalTokensInCirculation = 115,
    NoCouponTokensInCirculation = 116,

    // Yield and redemption errors
    NoExcessYieldAvailable = 117,
    NoYieldAvailableForRedemption = 118,

    // General errors
    AmountMustBePositive = 119,
}
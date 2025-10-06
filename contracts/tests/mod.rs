// Centralized WASM imports for all tests
// Following Blend Protocol's pattern of importing WASMs once per module

// Standardized Yield token
pub mod standardized_yield {
    soroban_sdk::contractimport!(
        file = "../wasms/standardized_yield.wasm"
    );
}

// Principal Token
pub mod principal_token {
    soroban_sdk::contractimport!(
        file = "../wasms/principal_token.wasm"
    );
}

// Yield Token
pub mod yield_token {
    soroban_sdk::contractimport!(
        file = "../wasms/yield_token.wasm"
    );
}

// Re-export WASMs for use in test fixtures
pub use standardized_yield::WASM as SY_WASM;
pub use principal_token::WASM as PT_WASM;
pub use yield_token::WASM as YT_WASM;

// Re-export clients
pub use standardized_yield::Client as StandardizedYieldClient;
pub use principal_token::Client as PrincipalTokenClient;
pub use yield_token::Client as YieldTokenClient;

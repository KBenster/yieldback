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

// Mock Token
pub mod mock_token {
    soroban_sdk::contractimport!(
        file = "../wasms/mock_token.wasm"
    );
}

// Mock Adapter
pub mod mock_adapter {
    soroban_sdk::contractimport!(
        file = "../wasms/mock_adapter.wasm"
    );
}

// Mock Yield Protocol
pub mod mock_yield_protocol {
    soroban_sdk::contractimport!(
        file = "../wasms/mock_yield_protocol.wasm"
    );
}

// Re-export WASMs for use in test fixtures
pub use standardized_yield::WASM as SY_WASM;
pub use principal_token::WASM as PT_WASM;
pub use yield_token::WASM as YT_WASM;
pub use mock_token::WASM as MOCK_TOKEN_WASM;
pub use mock_adapter::WASM as MOCK_ADAPTER_WASM;
pub use mock_yield_protocol::WASM as MOCK_YIELD_PROTOCOL_WASM;

// Re-export clients
pub use standardized_yield::Client as StandardizedYieldClient;
pub use principal_token::Client as PrincipalTokenClient;
pub use yield_token::Client as YieldTokenClient;
pub use mock_token::Client as MockTokenClient;
pub use mock_adapter::Client as MockAdapterClient;
pub use mock_yield_protocol::Client as MockYieldProtocolClient;

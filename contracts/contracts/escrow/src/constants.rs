use soroban_sdk::{symbol_short, Symbol};

// Storage key constants
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const TOKEN: Symbol = symbol_short!("TOKEN");
pub const BLEND_POOL: Symbol = symbol_short!("BLND_PL");
pub const SHARE_TOKEN: Symbol = symbol_short!("SHARE_TK");
pub const MATURITY: Symbol = symbol_short!("MATURITY");
pub const COUPON_AMOUNT: Symbol = symbol_short!("COUP_AMT");
pub const PRINCIPAL_AMOUNT: Symbol = symbol_short!("PRIN_AMT");
pub const COUPON_DEPOSITED: Symbol = symbol_short!("COUP_DEP"); // boolean flag
pub const PRINCIPAL_DEPOSITED: Symbol = symbol_short!("PRIN_DEP"); // boolean flag

// Hardcoded share token WASM hash
pub const SHARE_TOKEN_WASM_HASH: [u8; 32] = [
    0x59, 0x02, 0x2e, 0x87, 0x32, 0xdd, 0x42, 0xf0,
    0x00, 0x91, 0xb1, 0xf9, 0xe6, 0xe9, 0xc3, 0x6c,
    0xf1, 0xa3, 0x6f, 0xc1, 0xcf, 0xb9, 0xfd, 0x1e,
    0xb7, 0x9b, 0xc9, 0x45, 0xa1, 0xa3, 0xd7, 0x05
];

// Default salt for contract deployment
pub const DEFAULT_SALT: [u8; 32] = [0; 32];

// Request types for Blend protocol
pub const BLEND_SUPPLY_REQUEST: u32 = 0;
pub const BLEND_WITHDRAW_REQUEST: u32 = 1;

// Default reserve ID for Blend protocol
pub const DEFAULT_RESERVE_ID: u32 = 0;
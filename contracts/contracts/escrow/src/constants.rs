use soroban_sdk::{symbol_short, Symbol};

// Storage key constants
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const TOKEN: Symbol = symbol_short!("TOKEN");
pub const BLEND_POOL: Symbol = symbol_short!("BLND_PL");
pub const COUPON_TOKEN: Symbol = symbol_short!("COUP_TK");
pub const PRINCIPAL_TOKEN: Symbol = symbol_short!("PRIN_TK");
pub const MATURITY: Symbol = symbol_short!("MATURITY");
pub const COUPON_AMOUNT: Symbol = symbol_short!("COUP_AMT");
pub const PRINCIPAL_AMOUNT: Symbol = symbol_short!("PRIN_AMT");
pub const COUPON_DEPOSITED: Symbol = symbol_short!("COUP_DEP"); // boolean flag
pub const PRINCIPAL_DEPOSITED: Symbol = symbol_short!("PRIN_DEP"); // boolean flag

// Hardcoded coupon token WASM hash
pub const COUPON_TOKEN_WASM_HASH: [u8; 32] = [
    0xed, 0xc1, 0x51, 0xb5, 0x67, 0x3b, 0x12, 0xc3,
    0xa5, 0xd0, 0x65, 0x3b, 0xd8, 0xf7, 0x28, 0x04,
    0x73, 0x57, 0xeb, 0xcc, 0xd7, 0x97, 0x1d, 0x8a,
    0x63, 0x6d, 0xf6, 0x06, 0x71, 0x3d, 0x84, 0x7f
];

// Hardcoded principal token WASM hash
pub const PRINCIPAL_TOKEN_WASM_HASH: [u8; 32] = [
    0xb9, 0x6c, 0x22, 0xf7, 0x25, 0xae, 0x61, 0x7e,
    0x5d, 0x09, 0x56, 0xf3, 0x25, 0xbe, 0xe3, 0xcc,
    0x3d, 0xfd, 0xf6, 0xd2, 0x60, 0x20, 0x44, 0x8e,
    0x55, 0x49, 0x17, 0xd0, 0x1a, 0x50, 0x66, 0xac
];

// Default salts for contract deployment
pub const COUPON_TOKEN_SALT: [u8; 32] = [1; 32];
pub const PRINCIPAL_TOKEN_SALT: [u8; 32] = [2; 32];

// Request types for Blend protocol
pub const BLEND_SUPPLY_REQUEST: u32 = 0;
pub const BLEND_WITHDRAW_REQUEST: u32 = 1;

// Default reserve ID for Blend protocol
pub const DEFAULT_RESERVE_ID: u32 = 0;
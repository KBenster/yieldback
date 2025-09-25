use soroban_sdk::{symbol_short, Symbol};

// Storage key constants
pub const BLEND_POOL: Symbol = symbol_short!("BLND_PL");

// Request types for Blend protocol
pub const BLEND_SUPPLY_REQUEST: u32 = 0;
pub const BLEND_WITHDRAW_REQUEST: u32 = 1;

// Default reserve ID for Blend protocol
pub const DEFAULT_RESERVE_ID: u32 = 0;

// Protocol name
pub const PROTOCOL_NAME: &str = "Blend";
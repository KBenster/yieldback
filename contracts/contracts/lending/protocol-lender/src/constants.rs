use soroban_sdk::{symbol_short, Symbol};

// Storage key constants
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const TOKEN: Symbol = symbol_short!("TOKEN");
pub const PROTOCOL_ADDR: Symbol = symbol_short!("PROT_ADR");
pub const PROTOCOL_ADAPTER: Symbol = symbol_short!("PROT_ADP");
pub const EXPECTED_AMOUNT: Symbol = symbol_short!("EXP_AMT");
pub const RECEIVED_AMOUNT: Symbol = symbol_short!("REC_AMT");
pub const IS_LENT: Symbol = symbol_short!("IS_LENT");
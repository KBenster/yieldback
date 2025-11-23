#![no_std]

mod contract;
mod storage;

pub use contract::LiquidityPool;

use soroban_sdk::contractmeta;

// Metadata that is added on to the WASM custom section
contractmeta!(
    key = "Description",
    val = "Constant product AMM with a .3% swap fee"
);

#![no_std]

#[cfg(feature = "contract")]
mod contract;
#[cfg(feature = "contract")]
mod storage;

#[cfg(feature = "contract")]
pub use contract::{YieldToken, YieldTokenTrait};

#![no_std]
mod storage;

#[cfg(test)]
mod test;

use soroban_sdk::{contract, contractimpl, token, Address, Env, String};
use storage::{VaultError, VaultStorage};

#[contract]
pub struct Vault;


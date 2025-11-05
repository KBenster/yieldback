#![no_std]

use soroban_sdk::{contracttype, panic_with_error, token, Address, Env};

/// Storage keys for the data associated with the vault extension
#[contracttype]
pub enum VaultStorageKey {
    /// Stores the address of the vault's underlying asset
    AssetAddress,
    /// Stores the virtual decimals offset of the vault (for inflation attack mitigation)
    VirtualDecimalsOffset,
}

/// Scaling factor for calculations (7 decimals like in your escrow contract)
pub const INDEX_SCALE: i128 = 10_000_000; // 1e7

/// Maximum decimals offset for security
pub const MAX_DECIMALS_OFFSET: u32 = 10;

/// Vault errors following OpenZeppelin conventions
#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum VaultError {
    /// Asset address not initialized
    AssetAddressNotSet = 400,
    /// Asset address already set
    AssetAddressAlreadySet = 401,
    /// Virtual decimals offset already set
    VirtualDecimalsOffsetAlreadySet = 402,
    /// Invalid assets amount
    InvalidAssetsAmount = 403,
    /// Invalid shares amount
    InvalidSharesAmount = 404,
    /// Exceeded max deposit
    ExceededMaxDeposit = 405,
    /// Exceeded max mint
    ExceededMaxMint = 406,
    /// Exceeded max withdraw
    ExceededMaxWithdraw = 407,
    /// Exceeded max redeem
    ExceededMaxRedeem = 408,
    /// Max decimals offset exceeded
    MaxDecimalsOffsetExceeded = 409,
    /// Math overflow
    MathOverflow = 410,
}

/// Storage and core logic for the vault
pub struct VaultStorage;

impl VaultStorage {
    /// Set the underlying asset address (one-time only)
    pub fn set_asset(e: &Env, asset: Address) {
        if e.storage().instance().has(&VaultStorageKey::AssetAddress) {
            panic_with_error!(e, VaultError::AssetAddressAlreadySet);
        }
        e.storage().instance().set(&VaultStorageKey::AssetAddress, &asset);
    }

    /// Get the underlying asset address
    pub fn query_asset(e: &Env) -> Address {
        e.storage()
            .instance()
            .get(&VaultStorageKey::AssetAddress)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::AssetAddressNotSet))
    }

    /// Set the virtual decimals offset (one-time only)
    pub fn set_decimals_offset(e: &Env, offset: u32) {
        if offset > MAX_DECIMALS_OFFSET {
            panic_with_error!(e, VaultError::MaxDecimalsOffsetExceeded);
        }
        if e.storage().instance().has(&VaultStorageKey::VirtualDecimalsOffset) {
            panic_with_error!(e, VaultError::VirtualDecimalsOffsetAlreadySet);
        }
        e.storage().instance().set(&VaultStorageKey::VirtualDecimalsOffset, &offset);
    }

    /// Get the virtual decimals offset (defaults to 0)
    pub fn get_decimals_offset(e: &Env) -> u32 {
        e.storage()
            .instance()
            .get(&VaultStorageKey::VirtualDecimalsOffset)
            .unwrap_or(0)
    }

    /// Get the underlying asset decimals
    pub fn get_underlying_asset_decimals(e: &Env) -> u32 {
        let token_client = token::Client::new(e, &Self::query_asset(e));
        token_client.decimals()
    }

    /// Get vault decimals (asset decimals + offset)
    pub fn decimals(e: &Env) -> u32 {
        Self::get_underlying_asset_decimals(e)
            .checked_add(Self::get_decimals_offset(e))
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow))
    }

    /// Get total assets held by the vault
    pub fn total_assets(e: &Env) -> i128 {
        let token_client = token::Client::new(e, &Self::query_asset(e));
        token_client.balance(&e.current_contract_address())
    }

    /// Get total supply of shares from storage
    pub fn total_supply(e: &Env) -> i128 {
        e.storage()
            .instance()
            .get(&"total_supply")
            .unwrap_or(0)
    }

    /// Set total supply of shares
    pub fn set_total_supply(e: &Env, amount: i128) {
        e.storage().instance().set(&"total_supply", &amount);
    }

    /// Get balance of an account
    pub fn balance(e: &Env, account: &Address) -> i128 {
        e.storage()
            .persistent()
            .get(account)
            .unwrap_or(0)
    }

    /// Set balance of an account
    pub fn set_balance(e: &Env, account: &Address, amount: i128) {
        e.storage().persistent().set(account, &amount);
    }

    /// Convert assets to shares with rounding
    pub fn convert_to_shares(e: &Env, assets: i128) -> i128 {
        if assets < 0 {
            panic_with_error!(e, VaultError::InvalidAssetsAmount);
        }
        if assets == 0 {
            return 0;
        }

        let pow = 10_i128
            .checked_pow(Self::get_decimals_offset(e))
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        let total_supply = Self::total_supply(e);
        let total_assets = Self::total_assets(e);

        // Virtual supply = totalSupply + 10^offset
        let virtual_supply = total_supply
            .checked_add(pow)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        // Virtual assets = totalAssets + 1
        let virtual_assets = total_assets
            .checked_add(1)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        // shares = (assets * virtual_supply) / virtual_assets
        let numerator = assets
            .checked_mul(virtual_supply)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        numerator / virtual_assets
    }

    /// Convert shares to assets with rounding
    pub fn convert_to_assets(e: &Env, shares: i128) -> i128 {
        if shares < 0 {
            panic_with_error!(e, VaultError::InvalidSharesAmount);
        }
        if shares == 0 {
            return 0;
        }

        let pow = 10_i128
            .checked_pow(Self::get_decimals_offset(e))
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        let total_supply = Self::total_supply(e);
        let total_assets = Self::total_assets(e);

        // Virtual supply = totalSupply + 10^offset
        let virtual_supply = total_supply
            .checked_add(pow)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        // Virtual assets = totalAssets + 1
        let virtual_assets = total_assets
            .checked_add(1)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        // assets = (shares * virtual_assets) / virtual_supply
        let numerator = shares
            .checked_mul(virtual_assets)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));

        numerator / virtual_supply
    }

    /// Mint shares to an account
    pub fn mint_shares(e: &Env, to: &Address, amount: i128) {
        let balance = Self::balance(e, to);
        let new_balance = balance
            .checked_add(amount)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));
        Self::set_balance(e, to, new_balance);

        let total_supply = Self::total_supply(e);
        let new_supply = total_supply
            .checked_add(amount)
            .unwrap_or_else(|| panic_with_error!(e, VaultError::MathOverflow));
        Self::set_total_supply(e, new_supply);
    }

    /// Burn shares from an account
    pub fn burn_shares(e: &Env, from: &Address, amount: i128) {
        let balance = Self::balance(e, from);
        if balance < amount {
            panic_with_error!(e, VaultError::InvalidSharesAmount);
        }
        Self::set_balance(e, from, balance - amount);

        let total_supply = Self::total_supply(e);
        Self::set_total_supply(e, total_supply - amount);
    }
}
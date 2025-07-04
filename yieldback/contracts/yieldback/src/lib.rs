// lib.rs
#![no_std]

mod test;

use soroban_sdk::{contract, contractimpl, contracttype, contracterror, token, Address, Env, log, panic_with_error, vec, Vec, contractclient};

use token::{StellarAssetClient, TokenClient};

// Storage keys
#[contracttype]
pub enum DataKey {
    // Bond configuration
    BondToken,         // Address of the bond token
    BaseAsset,         // Address of the base asset (USDC)
    BlendPool,         // Address of the Blend pool
    BlendToken,        // Address of BLND token

    // Bond parameters (set by sponsor)
    MaturityTimestamp, // Bond maturity timestamp
    BondDuration,      // Duration in seconds
    DepositAmount,     // Exact deposit amount required from user
    CouponAmount,      // Exact coupon amount guaranteed by sponsor

    // State tracking
    TotalBondsIssued,  // Total bond tokens issued
    UserDeposited,     // User's deposit amount
    CouponFundsDeposited, // Sponsor's coupon guarantee funds
    LastHarvestTime,   // Last harvest timestamp
    IsMatured,         // Whether bond has matured
    IsActive,          // Whether position is open for deposit
    IsTaken,           // Whether position has been taken by a user

    // Addresses
    Sponsor,           // Sponsor who creates and funds position
    BondHolder,        // The single user who took this position
    Treasury,          // Treasury address for fees (optional)

    // User data (simplified since only one user)
    UserBonds,         // User's bond token balance
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    BondMatured = 5,
    BondNotMatured = 6,
    InsufficientBalance = 7,
    HarvestTooEarly = 8,
    PositionNotActive = 9,
    InsufficientCouponFunds = 10,
    PositionAlreadyTaken = 11,
    NotBondHolder = 12,
}

#[contracttype]
pub struct SponsorBondConfig {
    pub sponsor: Address,           // Sponsor creating the position
    pub treasury: Address,          // Optional treasury for fees
    pub bond_token: Address,        // Bond token to mint
    pub base_asset: Address,        // Base asset (USDC)
    pub blend_pool: Address,        // Blend pool address
    pub blend_token: Address,       // BLND token address
    pub bond_duration: u64,         // Duration in seconds
    pub deposit_amount: i128,       // Exact amount this position needs from user
    pub coupon_amount: i128,        // Exact coupon amount sponsor guarantees
}

#[contractclient(name = "BlendPoolClient")]
pub trait BlendPoolInterface {
    /// Submit requests to the pool (withdraw, supply, borrow, etc)
    /// Based on Blend protocol: submit(from, spender, to, requests)
    fn submit(
        env: Env,
        from: Address,
        spender: Address,
        to: Address,
        requests: Vec<(Address, i128, u32)>  // (asset_address, amount, request_type)
    );

    /// Get user's balance/position in the pool for a specific asset
    fn get_user_balance(env: Env, user: Address, asset: Address) -> i128;

    /// Get pool reserves for an asset
    fn get_reserve(env: Env, asset: Address) -> i128;
}
// BLEND PROTOCOL REQUEST TYPE CONSTANTS
pub const REQUEST_TYPE_SUPPLY: u32 = 0;
pub const REQUEST_TYPE_WITHDRAW: u32 = 1;
pub const REQUEST_TYPE_BORROW: u32 = 2;
pub const REQUEST_TYPE_REPAY: u32 = 3;
pub trait BondWrapperTrait {
    /// Sponsor creates and funds the bond position (immediately active)
    fn create_position(env: Env, config: SponsorBondConfig, coupon_funding: i128);

    /// User takes the entire bond position
    fn deposit(env: Env, user: Address, amount: i128);

    /// Harvest yield from Blend pool
    fn harvest(env: Env);

    /// Redeem bond tokens at maturity
    fn redeem(env: Env, user: Address) -> i128;

    /// Sponsor functions
    fn add_coupon_funding(env: Env, sponsor: Address, amount: i128);

    /// View functions
    fn get_bond_info(env: Env) -> BondInfo;
    fn get_user_position(env: Env, user: Address) -> UserPosition;
}

#[contracttype]
pub struct BondInfo {
    pub total_bonds_issued: i128,
    pub user_deposited: i128,
    pub bond_holder: Option<Address>,   // The single user who took this position
    pub coupon_amount: i128,            // Simplified - sponsor's coupon amount
    pub maturity_timestamp: u64,
    pub is_matured: bool,
    pub is_active: bool,
    pub is_taken: bool,                 // Whether position has been claimed
    pub last_harvest_time: u64,
}

#[contracttype]
pub struct UserPosition {
    pub deposit_amount: i128,
    pub bond_balance: i128,
    pub coupon_earned: i128,
}

#[contract]
pub struct BondWrapper;

#[contractimpl]
impl BondWrapperTrait for BondWrapper {
    fn create_position(env: Env, config: SponsorBondConfig, coupon_funding: i128) {
        config.sponsor.require_auth();

        if env.storage().persistent().has(&DataKey::Sponsor) {
            panic_with_error!(&env, Error::AlreadyInitialized);
        }

        if config.bond_duration == 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }

        if config.deposit_amount <= 0 || config.coupon_amount <= 0 {
            panic_with_error!(&env, Error::InvalidAmount);
        }

        let current_time = env.ledger().timestamp();
        let maturity = current_time + config.bond_duration;

        // Sponsor must provide exactly the coupon amount they're guaranteeing
        if coupon_funding != config.coupon_amount {
            panic_with_error!(&env, Error::InsufficientCouponFunds);
        }

        // Transfer coupon funding from sponsor
        let base_asset_client = TokenClient::new(&env, &config.base_asset);
        base_asset_client.transfer(&config.sponsor, &env.current_contract_address(), &coupon_funding);

        // Store configuration
        env.storage().persistent().set(&DataKey::Sponsor, &config.sponsor);
        env.storage().persistent().set(&DataKey::Treasury, &config.treasury);
        env.storage().persistent().set(&DataKey::BondToken, &config.bond_token);
        env.storage().persistent().set(&DataKey::BaseAsset, &config.base_asset);
        env.storage().persistent().set(&DataKey::BlendPool, &config.blend_pool);
        env.storage().persistent().set(&DataKey::BlendToken, &config.blend_token);

        // Store bond parameters
        env.storage().persistent().set(&DataKey::MaturityTimestamp, &maturity);
        env.storage().persistent().set(&DataKey::BondDuration, &config.bond_duration);
        env.storage().persistent().set(&DataKey::DepositAmount, &config.deposit_amount);
        env.storage().persistent().set(&DataKey::CouponAmount, &config.coupon_amount);

        // Initialize state - position is IMMEDIATELY ACTIVE
        env.storage().persistent().set(&DataKey::TotalBondsIssued, &0i128);
        env.storage().persistent().set(&DataKey::UserDeposited, &0i128);
        env.storage().persistent().set(&DataKey::CouponFundsDeposited, &coupon_funding);
        env.storage().persistent().set(&DataKey::LastHarvestTime, &current_time);
        env.storage().persistent().set(&DataKey::IsMatured, &false);
        env.storage().persistent().set(&DataKey::IsActive, &true); // ACTIVE immediately!
        env.storage().persistent().set(&DataKey::IsTaken, &false);
        env.storage().persistent().set(&DataKey::UserBonds, &0i128);

        log!(&env, "Bond position created and ACTIVE - amount: {}, coupon: {}, duration: {} seconds",
             config.deposit_amount, config.coupon_amount, config.bond_duration);
    }

    fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();
        Self::require_initialized(&env);

        if Self::is_matured(&env) {
            panic_with_error!(&env, Error::BondMatured);
        }

        let is_active: bool = env.storage().persistent().get(&DataKey::IsActive).unwrap_or(false);
        if !is_active {
            panic_with_error!(&env, Error::PositionNotActive);
        }

        let is_taken: bool = env.storage().persistent().get(&DataKey::IsTaken).unwrap_or(false);
        if is_taken {
            panic_with_error!(&env, Error::PositionAlreadyTaken);
        }

        let required_amount: i128 = env.storage().persistent().get(&DataKey::DepositAmount).unwrap();
        if amount != required_amount {
            panic_with_error!(&env, Error::InvalidAmount);
        }

        let base_asset: Address = env.storage().persistent().get(&DataKey::BaseAsset).unwrap();
        let bond_token: Address = env.storage().persistent().get(&DataKey::BondToken).unwrap();
        let blend_pool: Address = env.storage().persistent().get(&DataKey::BlendPool).unwrap();

        // Transfer base asset from user
        let base_client = TokenClient::new(&env, &base_asset);
        base_client.transfer(&user, &env.current_contract_address(), &amount);

        // Get sponsor's coupon funds
        let coupon_amount: i128 = env.storage().persistent().get(&DataKey::CouponAmount).unwrap();

        // Send EVERYTHING to Blend pool (user principal + sponsor coupon)
        let total_to_blend = amount + coupon_amount;
        base_client.transfer(&env.current_contract_address(), &blend_pool, &total_to_blend);

        // Mint bond tokens representing user's total claim (principal + coupon)
        let user_claim = amount + coupon_amount;
        let bond_admin_client = StellarAssetClient::new(&env, &bond_token);
        bond_admin_client.mint(&user, &user_claim);

        // Update storage - mark position as taken and close it
        env.storage().persistent().set(&DataKey::TotalBondsIssued, &user_claim);
        env.storage().persistent().set(&DataKey::UserDeposited, &amount);
        env.storage().persistent().set(&DataKey::UserBonds, &user_claim);
        env.storage().persistent().set(&DataKey::BondHolder, &user);
        env.storage().persistent().set(&DataKey::IsTaken, &true);
        env.storage().persistent().set(&DataKey::IsActive, &false); // Position now closed

        log!(&env, "Position TAKEN by user: {} principal + {} coupon = {} total to Blend",
             amount, coupon_amount, total_to_blend);
    }

    fn harvest(env: Env) {
        Self::require_initialized(&env);

        let current_time = env.ledger().timestamp();
        let last_harvest: u64 = env.storage().persistent().get(&DataKey::LastHarvestTime).unwrap();

        // Prevent too frequent harvesting (minimum 1 hour)
        if current_time - last_harvest < 3600 {
            panic_with_error!(&env, Error::HarvestTooEarly);
        }

        let base_asset: Address = env.storage().persistent().get(&DataKey::BaseAsset).unwrap();
        let blend_token: Address = env.storage().persistent().get(&DataKey::BlendToken).unwrap();
        let blend_pool: Address = env.storage().persistent().get(&DataKey::BlendPool).unwrap();
        let sponsor: Address = env.storage().persistent().get(&DataKey::Sponsor).unwrap();

        let base_client = TokenClient::new(&env, &base_asset);
        let blnd_client = TokenClient::new(&env, &blend_token);

        // Get balance before harvest
        let base_balance_before = base_client.balance(&env.current_contract_address());
        let blnd_balance_before = blnd_client.balance(&env.current_contract_address());

        // === STEP 1: Claim rewards from Blend protocol ===
        // TODO: Replace with actual Blend protocol calls
        // This might look like:
        // let blend_client = BlendClient::new(&env, &blend_pool);
        // blend_client.claim_rewards(&env.current_contract_address());
        // blend_client.withdraw_interest(&base_asset, &amount_available);

        // Get balance after harvest
        let base_balance_after = base_client.balance(&env.current_contract_address());
        let blnd_balance_after = blnd_client.balance(&env.current_contract_address());

        let base_yield = base_balance_after - base_balance_before;
        let blnd_yield = blnd_balance_after - blnd_balance_before;

        // === STEP 2: Convert BLND tokens to base asset (if any) ===
        let mut total_yield_in_base = base_yield;

        if blnd_yield > 0 {
            // TODO: Swap BLND to base asset via DEX
            // This might involve calling a DEX contract like:
            // let dex_client = DexClient::new(&env, &dex_address);
            // let swapped_amount = dex_client.swap(&blend_token, &base_asset, &blnd_yield);
            // total_yield_in_base += swapped_amount;

            // For now, we'll assume BLND has some value equivalent
            // In practice, you'd get the actual swap amount
            log!(&env, "BLND yield collected: {} (needs to be swapped)", blnd_yield);
        }

        // === STEP 3: All EXCESS yield goes to sponsor ===
        // Note: User is already guaranteed their principal + coupon
        // Any yield beyond that amount goes to sponsor
        if total_yield_in_base > 0 {
            // Transfer ALL yield to sponsor (they'll get net profit after user redemption)
            base_client.transfer(&env.current_contract_address(), &sponsor, &total_yield_in_base);
            log!(&env, "All yield sent to sponsor: {}", total_yield_in_base);
        }

        // Send any remaining BLND tokens directly to sponsor
        if blnd_yield > 0 {
            blnd_client.transfer(&env.current_contract_address(), &sponsor, &blnd_yield);
            log!(&env, "BLND emissions sent to sponsor: {}", blnd_yield);
        }

        log!(&env, "Harvest completed - Base yield: {}, BLND yield: {}",
             base_yield, blnd_yield);

        env.storage().persistent().set(&DataKey::LastHarvestTime, &current_time);
    }

    fn redeem(env: Env, user: Address) -> i128 {
        user.require_auth();
        Self::require_initialized(&env);

        if !Self::is_matured(&env) {
            panic_with_error!(&env, Error::BondNotMatured);
        }

        // Verify this user is the bond holder
        let bond_holder: Address = env.storage().persistent().get(&DataKey::BondHolder)
            .expect("No bond holder found");
        if user != bond_holder {
            panic_with_error!(&env, Error::NotBondHolder);
        }

        let bond_token: Address = env.storage().persistent().get(&DataKey::BondToken).unwrap();
        let base_asset: Address = env.storage().persistent().get(&DataKey::BaseAsset).unwrap();
        let blend_pool: Address = env.storage().persistent().get(&DataKey::BlendPool).unwrap();

        let bond_client = TokenClient::new(&env, &bond_token);
        let base_client = TokenClient::new(&env, &base_asset);

        let user_bonds: i128 = env.storage().persistent().get(&DataKey::UserBonds).unwrap_or(0);

        if user_bonds == 0 {
            panic_with_error!(&env, Error::InsufficientBalance);
        }

        // === BLEND PROTOCOL INTEGRATION ===
        // Withdraw ALL funds from Blend pool using the proper Blend protocol interface

        let contract_address = env.current_contract_address();
        let pool_balance_before = base_client.balance(&contract_address);

        // Create Blend pool client using the testnet pool address
        let pool_client = BlendPoolClient::new(&env, &blend_pool);

        // Query our current position in the Blend pool
        let our_pool_balance = pool_client.get_user_balance(&contract_address, &base_asset);

        log!(&env, "Withdrawing {} from Blend pool", our_pool_balance);

        if our_pool_balance > 0 {
            // Create withdrawal request using Blend's submit interface
            // Format: Vec<(asset_address, amount, request_type)>
            let withdrawal_requests = vec![&env, (
                base_asset.clone(),              // asset to withdraw
                our_pool_balance,               // amount to withdraw (all of it)
                REQUEST_TYPE_WITHDRAW,          // request type: withdraw = 1
            )];

            // Submit the withdrawal request to Blend pool
            pool_client.submit(
                &contract_address,  // from - our contract address
                &contract_address,  // spender - our contract has permission
                &contract_address,  // to - receive tokens at our contract
                &withdrawal_requests
            );
        }

        // Get our balance after withdrawal from Blend
        let pool_balance_after = base_client.balance(&contract_address);
        let total_available = pool_balance_after;

        // User gets their bond token amount (principal + guaranteed coupon)
        let user_redemption = user_bonds;

        // Calculate any excess yield that goes to sponsor
        let excess_yield = if total_available > user_redemption {
            total_available - user_redemption
        } else {
            0
        };

        if excess_yield > 0 {
            // Send excess yield to sponsor
            let sponsor: Address = env.storage().persistent().get(&DataKey::Sponsor).unwrap();
            base_client.transfer(&contract_address, &sponsor, &excess_yield);
            log!(&env, "Excess yield to sponsor: {}", excess_yield);
        }

        // Ensure we have enough to cover user's redemption
        let available_for_user = if total_available >= user_redemption {
            user_redemption
        } else {
            // If somehow we don't have enough from Blend, use what we have
            total_available
        };

        // Transfer user's portion (principal + coupon)
        base_client.transfer(&contract_address, &user, &available_for_user);

        // Burn bond tokens
        bond_client.burn(&user, &user_bonds);

        // Clear position
        env.storage().persistent().set(&DataKey::UserBonds, &0i128);
        env.storage().persistent().remove(&DataKey::BondHolder);

        log!(&env, "User redeemed: {} from Blend withdrawal of {}",
             available_for_user, total_available);

        available_for_user
    }

    fn add_coupon_funding(env: Env, sponsor: Address, amount: i128) {
        sponsor.require_auth();
        Self::require_sponsor(&env, &sponsor);

        let base_asset: Address = env.storage().persistent().get(&DataKey::BaseAsset).unwrap();
        let base_client = TokenClient::new(&env, &base_asset);

        base_client.transfer(&sponsor, &env.current_contract_address(), &amount);

        let mut coupon_deposited: i128 = env.storage().persistent().get(&DataKey::CouponFundsDeposited).unwrap();
        coupon_deposited += amount;
        env.storage().persistent().set(&DataKey::CouponFundsDeposited, &coupon_deposited);

        log!(&env, "Additional coupon funding: {}", amount);
    }

    fn get_bond_info(env: Env) -> BondInfo {
        Self::require_initialized(&env);

        let bond_holder: Option<Address> = env.storage().persistent().get(&DataKey::BondHolder);
        let coupon_amount: i128 = env.storage().persistent().get(&DataKey::CouponAmount).unwrap_or(0);

        BondInfo {
            total_bonds_issued: env.storage().persistent().get(&DataKey::TotalBondsIssued).unwrap_or(0),
            user_deposited: env.storage().persistent().get(&DataKey::UserDeposited).unwrap_or(0),
            bond_holder,
            coupon_amount,
            maturity_timestamp: env.storage().persistent().get(&DataKey::MaturityTimestamp).unwrap(),
            is_matured: Self::is_matured(&env),
            is_active: env.storage().persistent().get(&DataKey::IsActive).unwrap_or(false),
            is_taken: env.storage().persistent().get(&DataKey::IsTaken).unwrap_or(false),
            last_harvest_time: env.storage().persistent().get(&DataKey::LastHarvestTime).unwrap(),
        }
    }

    fn get_user_position(env: Env, user: Address) -> UserPosition {
        Self::require_initialized(&env);

        let bond_holder: Option<Address> = env.storage().persistent().get(&DataKey::BondHolder);

        // Only return position if this user is the bond holder
        if let Some(holder) = bond_holder {
            if holder == user {
                let deposit_amount = env.storage().persistent().get(&DataKey::UserDeposited).unwrap_or(0);
                let bond_balance = env.storage().persistent().get(&DataKey::UserBonds).unwrap_or(0);
                let coupon_earned = env.storage().persistent().get(&DataKey::CouponAmount).unwrap_or(0);

                return UserPosition {
                    deposit_amount,
                    bond_balance,
                    coupon_earned,
                };
            }
        }

        // Return empty position if user is not the bond holder
        UserPosition {
            deposit_amount: 0,
            bond_balance: 0,
            coupon_earned: 0,
        }
    }
}

impl BondWrapper {
    fn require_initialized(env: &Env) {
        if !env.storage().persistent().has(&DataKey::Sponsor) {
            panic_with_error!(env, Error::NotInitialized);
        }
    }

    fn require_sponsor(env: &Env, sponsor: &Address) {
        let stored_sponsor: Address = env.storage().persistent().get(&DataKey::Sponsor).unwrap();
        if *sponsor != stored_sponsor {
            panic_with_error!(env, Error::Unauthorized);
        }
    }

    fn is_matured(env: &Env) -> bool {
        let maturity: u64 = env.storage().persistent().get(&DataKey::MaturityTimestamp).unwrap();
        let current_time = env.ledger().timestamp();
        current_time >= maturity
    }
}
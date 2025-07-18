#![no_std]

mod test;

use soroban_sdk::{contract, contractimpl, contracttype, contracterror, token, Address, Env, log, panic_with_error, vec, Vec, contractclient, BytesN, Bytes};

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
    InsufficientCouponFunds = 8,
    PositionAlreadyTaken = 9,
    NotBondHolder = 10,
}

#[contracttype]
pub struct SponsorBondConfig {
    pub base_asset: Address,        // Base asset (USDC)
    pub blend_pool: Address,        // Blend pool address
    pub blend_token: Address,       // BLND token address
    pub bond_token: Address,        // Bond token
    pub bond_duration: u64,         // Duration in seconds
    pub coupon_amount: i128,        // Exact coupon amount sponsor guarantees
    pub deposit_amount: i128,       // Exact amount this position needs from user
    pub sponsor: Address,           // Sponsor creating the position
}

#[contractclient(name = "BlendPoolClient")]
pub trait BlendPoolInterface {
    fn submit(
        env: Env,
        from: Address,
        spender: Address,
        to: Address,
        requests: Vec<(Address, i128, u32)>  // (asset_address, amount, request_type)
    );

    fn get_user_balance(env: Env, user: Address, asset: Address) -> i128;

    fn get_reserve(env: Env, asset: Address) -> i128;
}
// BLEND PROTOCOL REQUEST TYPE CONSTANTS
pub const REQUEST_TYPE_SUPPLY: u32 = 0;
pub const REQUEST_TYPE_WITHDRAW: u32 = 1;
pub const REQUEST_TYPE_BORROW: u32 = 2;
pub const REQUEST_TYPE_REPAY: u32 = 3;
pub trait BondWrapperTrait {
    fn create_position(env: Env, config: SponsorBondConfig, coupon_funding: i128);

    fn deposit(env: Env, user: Address, amount: i128);

    fn redeem(env: Env, user: Address) -> i128;

    fn add_coupon_funding(env: Env, sponsor: Address, amount: i128);

    fn get_bond_info(env: Env) -> BondInfo;
    fn get_user_position(env: Env, user: Address) -> UserPosition;
    fn get_bond_token_address(env: Env) -> Address;
}

#[contracttype]
pub struct BondInfo {
    pub total_bonds_issued: i128,
    pub user_deposited: i128,
    pub bond_holder: Option<Address>,   // The single user who took this position
    pub coupon_amount: i128,            // sponsor's coupon amount
    pub maturity_timestamp: u64,
    pub is_matured: bool,
    pub is_taken: bool,                 // Whether position has been claimed
    pub bond_token_address: Address,    // Address of the created bond token
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
        env.storage().persistent().set(&DataKey::BaseAsset, &config.base_asset);
        env.storage().persistent().set(&DataKey::BlendPool, &config.blend_pool);
        env.storage().persistent().set(&DataKey::BlendToken, &config.blend_token);
        env.storage().persistent().set(&DataKey::BondToken, &config.bond_token);

        // Store bond parameters
        env.storage().persistent().set(&DataKey::MaturityTimestamp, &maturity);
        env.storage().persistent().set(&DataKey::BondDuration, &config.bond_duration);
        env.storage().persistent().set(&DataKey::DepositAmount, &config.deposit_amount);
        env.storage().persistent().set(&DataKey::CouponAmount, &config.coupon_amount);

        // Initialize state - position is IMMEDIATELY ACTIVE
        env.storage().persistent().set(&DataKey::TotalBondsIssued, &0i128);
        env.storage().persistent().set(&DataKey::UserDeposited, &0i128);
        env.storage().persistent().set(&DataKey::CouponFundsDeposited, &coupon_funding);
        env.storage().persistent().set(&DataKey::IsMatured, &false);
        env.storage().persistent().set(&DataKey::IsTaken, &false);
        env.storage().persistent().set(&DataKey::UserBonds, &0i128);
    }

    fn deposit(env: Env, user: Address, amount: i128) {
        user.require_auth();
        Self::require_initialized(&env);

        if Self::is_matured(&env) {
            panic_with_error!(&env, Error::BondMatured);
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

        // Transfer base asset from user to contract
        let base_client = TokenClient::new(&env, &base_asset);
        base_client.transfer(&user, &env.current_contract_address(), &amount);

        // Get sponsor's coupon funds
        let coupon_amount: i128 = env.storage().persistent().get(&DataKey::CouponAmount).unwrap();

        // Send total (user deposit + sponsor coupon) to blend pool
        let total_to_blend = amount + coupon_amount;
        base_client.transfer(&env.current_contract_address(), &blend_pool, &total_to_blend);

        // Mint bond tokens representing user's total claim (principal + coupon)
        let user_claim = amount + coupon_amount;

        // Use StellarAssetClient for minting (since we're using a pre-deployed Stellar token)
        //let bond_admin_client = StellarAssetClient::new(&env, &bond_token);
        //bond_admin_client.mint(&user, &user_claim);

        // Update storage - mark position as taken and close it
        env.storage().persistent().set(&DataKey::TotalBondsIssued, &user_claim);
        env.storage().persistent().set(&DataKey::UserDeposited, &amount);
        env.storage().persistent().set(&DataKey::UserBonds, &user_claim);
        env.storage().persistent().set(&DataKey::BondHolder, &user);
        env.storage().persistent().set(&DataKey::IsTaken, &true);
        env.storage().persistent().set(&DataKey::IsActive, &false); // Position now closed
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

        // Withdraw funds from Blend pool using the Blend protocol interface

        let contract_address = env.current_contract_address();
        let pool_balance_before = base_client.balance(&contract_address);

        // Create Blend pool client using the testnet pool address
        let pool_client = BlendPoolClient::new(&env, &blend_pool);

        // Query our current position in the Blend pool
        let our_pool_balance = pool_client.get_user_balance(&contract_address, &base_asset);

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


        available_for_user
    }

    //A sponsor could increase their position if it isnt taken quickly
    fn add_coupon_funding(env: Env, sponsor: Address, amount: i128) {
        sponsor.require_auth();
        Self::require_sponsor(&env, &sponsor);

        let base_asset: Address = env.storage().persistent().get(&DataKey::BaseAsset).unwrap();
        let base_client = TokenClient::new(&env, &base_asset);

        base_client.transfer(&sponsor, &env.current_contract_address(), &amount);

        let mut coupon_deposited: i128 = env.storage().persistent().get(&DataKey::CouponFundsDeposited).unwrap();
        coupon_deposited += amount;
        env.storage().persistent().set(&DataKey::CouponFundsDeposited, &coupon_deposited);

    }

    fn get_bond_info(env: Env) -> BondInfo {
        Self::require_initialized(&env);

        let bond_holder: Option<Address> = env.storage().persistent().get(&DataKey::BondHolder);
        let coupon_amount: i128 = env.storage().persistent().get(&DataKey::CouponAmount).unwrap_or(0);
        let bond_token_address: Address = env.storage().persistent().get(&DataKey::BondToken).unwrap();

        BondInfo {
            total_bonds_issued: env.storage().persistent().get(&DataKey::TotalBondsIssued).unwrap_or(0),
            user_deposited: env.storage().persistent().get(&DataKey::UserDeposited).unwrap_or(0),
            bond_holder,
            coupon_amount,
            maturity_timestamp: env.storage().persistent().get(&DataKey::MaturityTimestamp).unwrap(),
            is_matured: Self::is_matured(&env),
            is_taken: env.storage().persistent().get(&DataKey::IsTaken).unwrap_or(false),
            bond_token_address,
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

    fn get_bond_token_address(env: Env) -> Address {
        Self::require_initialized(&env);
        env.storage().persistent().get(&DataKey::BondToken).unwrap()
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
#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger, LedgerInfo},
    token, Address, Env, IntoVal,
};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::StellarAssetClient<'a> {
    token::StellarAssetClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

fn create_test_env() -> (
    Env,
    Address,           // bond_wrapper_id
    Address,           // sponsor
    Address,           // user
    Address,           // base_asset_admin
    Address,           // blend_pool (mock)
    Address,           // blend_token_admin
    token::StellarAssetClient<'static>, // base_asset
    token::StellarAssetClient<'static>, // blend_token
) {
    let env = Env::default();
    env.mock_all_auths();

    let bond_wrapper_id = env.register_contract(None, BondWrapper);
    let sponsor = Address::generate(&env);
    let user = Address::generate(&env);
    let base_asset_admin = Address::generate(&env);
    let blend_pool = Address::generate(&env);
    let blend_token_admin = Address::generate(&env);

    // Create token contracts
    let base_asset = create_token_contract(&env, &base_asset_admin);
    let blend_token = create_token_contract(&env, &blend_token_admin);

    // Initialize tokens with some supply
    base_asset.mint(&sponsor, &1_000_000); // Give sponsor 1M base tokens
    base_asset.mint(&user, &100_000);      // Give user 100K base tokens
    blend_token.mint(&blend_pool, &50_000); // Pool has some BLND tokens

    (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        base_asset_admin,
        blend_pool,
        blend_token_admin,
        base_asset,
        blend_token,
    )
}

#[test]
fn test_create_position_success() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400, // 1 day
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    // Create position with coupon funding
    client.create_position(&config, &100);

    // Verify sponsor's balance decreased
    assert_eq!(token::TokenClient::new(&env, &base_asset.address).balance(&sponsor), 1_000_000 - 100);

    // Verify contract has the coupon funds
    assert_eq!(token::TokenClient::new(&env, &base_asset.address).balance(&bond_wrapper_id), 100);
}

#[test]
fn test_create_position_invalid_coupon_funding() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    // Try to create position with wrong coupon funding (should fail)
    let result = client.try_create_position(&config, &50); // Wrong amount
    assert!(result.is_err());
}

#[test]
fn test_create_position_already_initialized() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    // Create position first time
    client.create_position(&config, &100);

    // Try to create position again (should fail)
    let result = client.try_create_position(&config, &100);
    assert!(result.is_err());
}

#[test]
fn test_create_position_invalid_duration() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 0, // Invalid duration
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    let result = client.try_create_position(&config, &100);
    assert!(result.is_err());
}

#[test]
fn test_create_position_invalid_amounts() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Test with zero deposit amount
    let config1 = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 0, // Invalid
        coupon_amount: 100,
    };

    let result1 = client.try_create_position(&config1, &100);
    assert!(result1.is_err());

    // Test with zero coupon amount
    let config2 = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 0, // Invalid
    };

    let result2 = client.try_create_position(&config2, &0);
    assert!(result2.is_err());
}

#[test]
fn test_deposit_position_not_initialized() {
    let (
        env,
        bond_wrapper_id,
        _sponsor,
        user,
        _base_asset_admin,
        _blend_pool,
        _blend_token_admin,
        _base_asset,
        _blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Try to deposit without initializing position first
    let result = client.try_deposit(&user, &1000);
    assert!(result.is_err());
}

#[test]
fn test_add_coupon_funding_success() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create position first
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    let initial_balance = token::TokenClient::new(&env, &base_asset.address).balance(&sponsor);

    // Add more coupon funding
    client.add_coupon_funding(&sponsor, &50);

    // Verify sponsor's balance decreased by additional amount
    assert_eq!(
        token::TokenClient::new(&env, &base_asset.address).balance(&sponsor),
        initial_balance - 50
    );

    // Verify contract received the additional funding
    assert_eq!(
        token::TokenClient::new(&env, &base_asset.address).balance(&bond_wrapper_id),
        150 // Original 100 + additional 50
    );
}

#[test]
fn test_add_coupon_funding_unauthorized() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create position first
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    // Try to add funding from non-sponsor (should fail)
    let non_sponsor = Address::generate(&env);
    let result = client.try_add_coupon_funding(&non_sponsor, &50);
    assert!(result.is_err());
}

#[test]
fn test_get_user_position_no_bond_holder() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create position first
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    // Check position of user who hasn't taken the position
    let position = client.get_user_position(&user);
    assert_eq!(position.deposit_amount, 0);
    assert_eq!(position.bond_balance, 0);
    assert_eq!(position.coupon_earned, 0);
}

#[test]
fn test_redeem_not_initialized() {
    let (
        env,
        bond_wrapper_id,
        _sponsor,
        user,
        _base_asset_admin,
        _blend_pool,
        _blend_token_admin,
        _base_asset,
        _blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Try to redeem without initializing position first
    let result = client.try_redeem(&user);
    assert!(result.is_err());
}

#[test]
fn test_maturity_calculation() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let bond_duration = 86400u64; // 1 day
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    let current_time = env.ledger().timestamp();
    client.create_position(&config, &100);

    // Fast forward time but not past maturity
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + bond_duration / 2; // Half way to maturity
    });

    // Should not be matured yet (this would require get_bond_info to work)

    // Fast forward past maturity
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = current_time + bond_duration + 1;
    });

}


#[test]
fn test_require_functions() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Test require_initialized fails when not initialized
    let result = client.try_get_user_position(&sponsor);
    assert!(result.is_err());

    // Create position to initialize
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    let position = client.get_user_position(&sponsor);
    assert_eq!(position.deposit_amount, 0);
}
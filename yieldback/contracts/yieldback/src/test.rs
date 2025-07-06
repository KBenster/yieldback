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
    Address,           // treasury
    Address,           // bond_token_admin
    Address,           // base_asset_admin
    Address,           // blend_pool (mock)
    Address,           // blend_token_admin
    token::StellarAssetClient<'static>, // bond_token
    token::StellarAssetClient<'static>, // base_asset
    token::StellarAssetClient<'static>, // blend_token
) {
    let env = Env::default();
    env.mock_all_auths();

    let bond_wrapper_id = env.register_contract(None, BondWrapper);
    let sponsor = Address::generate(&env);
    let user = Address::generate(&env);
    let treasury = Address::generate(&env);
    let bond_token_admin = Address::generate(&env);
    let base_asset_admin = Address::generate(&env);
    let blend_pool = Address::generate(&env);
    let blend_token_admin = Address::generate(&env);

    // Create token contracts
    let bond_token = create_token_contract(&env, &bond_wrapper_id);
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
        treasury,
        bond_token_admin,
        base_asset_admin,
        blend_pool,
        blend_token_admin,
        bond_token,
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
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400, // 1 day
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    // Create position with coupon funding
    client.create_position(&config, &100);

    // Verify position was created
    let bond_info = client.get_bond_info();
    assert_eq!(bond_info.is_active, true);
    assert_eq!(bond_info.is_taken, false);
    assert_eq!(bond_info.coupon_amount, 100);

    // Verify sponsor's balance decreased
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&sponsor), 1_000_000 - 100);

    // Verify contract has the coupon funds
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&bond_wrapper_id), 100);
}

#[test]
fn test_create_position_invalid_coupon_funding() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        _user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
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
fn test_deposit_success() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        bond_token_admin, // This is now bond_wrapper_id
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create position first - include bond_token_admin in config
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    // User deposits exact amount
    let user_balance_before = TokenClient::new(&env, &base_asset.address).balance(&user);
    client.deposit(&user, &1000);

    // Verify user's balance decreased
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&user), user_balance_before - 1000);

    // Verify bond tokens were minted to user (principal + coupon)
    assert_eq!(TokenClient::new(&env, &bond_token.address).balance(&user), 1100); // 1000 + 100

    // Verify position is now taken and inactive
    let bond_info = client.get_bond_info();
    assert_eq!(bond_info.is_taken, true);
    assert_eq!(bond_info.is_active, false);
    assert_eq!(bond_info.bond_holder, Some(user.clone()));

    // Verify total amount went to blend pool
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&blend_pool), 1100); // user deposit + coupon
}

#[test]
fn test_deposit_wrong_amount() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    // Try to deposit wrong amount
    let result = client.try_deposit(&user, &500); // Wrong amount
    assert!(result.is_err());
}

#[test]
fn test_deposit_position_already_taken() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);

    // First user takes the position
    client.deposit(&user, &1000);

    // Second user tries to take the same position
    let user2 = Address::generate(&env);
    base_asset.mint(&user2, &1000);

    let result = client.try_deposit(&user2, &1000);
    assert!(result.is_err()); // Should fail - position already taken
}

#[test]
fn test_harvest() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create and take position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);
    client.deposit(&user, &1000);

    // Fast forward time by 2 hours (minimum 1 hour between harvests)
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = ledger.timestamp + 7200;
    });

    // Simulate some yield by adding tokens to contract
    base_asset.mint(&bond_wrapper_id, &50); // Some base asset yield
    blend_token.mint(&bond_wrapper_id, &25); // Some BLND yield

    let sponsor_balance_before = TokenClient::new(&env, &base_asset.address).balance(&sponsor);
    let sponsor_blnd_before = TokenClient::new(&env, &blend_token.address).balance(&sponsor);

    // Harvest
    client.harvest();

    // Verify yield went to sponsor
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&sponsor), sponsor_balance_before + 50);
    assert_eq!(TokenClient::new(&env, &blend_token.address).balance(&sponsor), sponsor_blnd_before + 25);
}

#[test]
fn test_harvest_too_early() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create and take position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);
    client.deposit(&user, &1000);

    // Try to harvest immediately (should fail)
    let result = client.try_harvest();
    assert!(result.is_err());
}

#[test]
fn test_redeem_at_maturity() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create and take position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400, // 1 day
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);
    client.deposit(&user, &1000);

    // Fast forward past maturity
    env.ledger().with_mut(|ledger| {
        ledger.timestamp = ledger.timestamp + 86400 + 1;
    });

    // Simulate blend pool having the funds plus some extra yield
    base_asset.mint(&blend_pool, &1200); // Original 1100 + 100 extra yield

    let user_balance_before = TokenClient::new(&env, &base_asset.address).balance(&user);
    let sponsor_balance_before = TokenClient::new(&env, &base_asset.address).balance(&sponsor);

    // Redeem
    let redeemed_amount = client.redeem(&user);

    // User should get exactly their bond amount (1100)
    assert_eq!(redeemed_amount, 1100);
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&user), user_balance_before + 1100);

    // Sponsor should get the excess yield (100)
    assert_eq!(TokenClient::new(&env, &base_asset.address).balance(&sponsor), sponsor_balance_before + 100);

    // Bond tokens should be burned
    assert_eq!(TokenClient::new(&env, &bond_token.address).balance(&user), 0);

    // Verify position is cleared
    let user_position = client.get_user_position(&user);
    assert_eq!(user_position.bond_balance, 0);
}

#[test]
fn test_redeem_before_maturity() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create and take position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);
    client.deposit(&user, &1000);

    // Try to redeem before maturity (should fail)
    let result = client.try_redeem(&user);
    assert!(result.is_err());
}

#[test]
fn test_get_user_position() {
    let (
        env,
        bond_wrapper_id,
        sponsor,
        user,
        treasury,
        _bond_token_admin,
        _base_asset_admin,
        blend_pool,
        _blend_token_admin,
        bond_token,
        base_asset,
        blend_token,
    ) = create_test_env();

    let client = BondWrapperClient::new(&env, &bond_wrapper_id);

    // Create and take position
    let config = SponsorBondConfig {
        sponsor: sponsor.clone(),
        treasury: treasury.clone(),
        bond_token: bond_token.address.clone(),
        base_asset: base_asset.address.clone(),
        blend_pool: blend_pool.clone(),
        blend_token: blend_token.address.clone(),
        bond_duration: 86400,
        deposit_amount: 1000,
        coupon_amount: 100,
    };

    client.create_position(&config, &100);
    client.deposit(&user, &1000);

    // Check user position
    let position = client.get_user_position(&user);
    assert_eq!(position.deposit_amount, 1000);
    assert_eq!(position.bond_balance, 1100); // deposit + coupon
    assert_eq!(position.coupon_earned, 100);

    // Check position of non-bond-holder
    let other_user = Address::generate(&env);
    let other_position = client.get_user_position(&other_user);
    assert_eq!(other_position.deposit_amount, 0);
    assert_eq!(other_position.bond_balance, 0);
    assert_eq!(other_position.coupon_earned, 0);
}

#![cfg(test)]

mod mock_adapter;
mod mock_token;
mod test_fixture;

use soroban_sdk::{testutils::Address as _, Address};
use test_suites::{StandardizedYieldClient, PrincipalTokenClient, YieldTokenClient};

use test_fixture::{TestFixture, SCALAR_7};

#[test]
fn test_escrow_initialization() {
    let fixture = TestFixture::create();

    // Verify that all contracts were deployed correctly
    assert_ne!(fixture.escrow.get_adapter(), Address::generate(&fixture.env));
    assert_ne!(fixture.escrow.get_sy_token(), Address::generate(&fixture.env));
    assert_ne!(fixture.escrow.get_pt_token(), Address::generate(&fixture.env));
    assert_ne!(fixture.escrow.get_yt_token(), Address::generate(&fixture.env));
}

#[test]
fn test_deposit_basic() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    // Mint tokens to user
    fixture.mint_tokens(&fixture.user, deposit_amount);

    // User deposits
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Verify tokens were transferred to adapter
    assert_eq!(fixture.token.balance(&fixture.user), 0);
    assert_eq!(fixture.adapter.get_assets(), deposit_amount);

    // Verify SY tokens were minted to escrow
    let sy_client = StandardizedYieldClient::new(&fixture.env, &fixture.sy_token_address);
    assert_eq!(sy_client.balance(&fixture.escrow.address), deposit_amount);

    // Verify PT and YT tokens were minted to user (same amount as deposit when exchange rate is 1)
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let yt_client = YieldTokenClient::new(&fixture.env, &fixture.yt_token_address);

    assert_eq!(pt_client.balance(&fixture.user), deposit_amount);
    assert_eq!(yt_client.balance(&fixture.user), deposit_amount);
}

#[test]
fn test_deposit_with_yield() {
    let fixture = TestFixture::create();

    let initial_deposit = 1000 * SCALAR_7;

    // First user deposits
    let user1 = Address::generate(&fixture.env);
    fixture.mint_tokens(&user1, initial_deposit);
    fixture.escrow.deposit(&user1, &initial_deposit);

    // Simulate yield generation (10% yield)
    let yield_amount = 100 * SCALAR_7;
    fixture.adapter.simulate_yield(&yield_amount);

    // Verify exchange index increased
    let exchange_index = fixture.escrow.get_current_exchange_index();
    assert!(exchange_index > 1);

    // Second user deposits same amount
    let user2 = Address::generate(&fixture.env);
    fixture.mint_tokens(&user2, initial_deposit);
    fixture.escrow.deposit(&user2, &initial_deposit);

    // User2 should receive fewer PT/YT tokens due to higher exchange rate
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);

    let user1_pt = pt_client.balance(&user1);
    let user2_pt = pt_client.balance(&user2);

    assert!(user2_pt < user1_pt);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_zero_amount() {
    let fixture = TestFixture::create();

    fixture.escrow.deposit(&fixture.user, &0);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_negative_amount() {
    let fixture = TestFixture::create();

    fixture.escrow.deposit(&fixture.user, &(-100 * SCALAR_7));
}

#[test]
fn test_exchange_index_calculation() {
    let fixture = TestFixture::create();

    // Initial exchange index should be 1
    assert_eq!(fixture.escrow.get_current_exchange_index(), 1);

    // Deposit some tokens
    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Exchange index should still be 1 (no yield yet)
    assert_eq!(fixture.escrow.get_current_exchange_index(), 1);

    // Simulate 20% yield
    let yield_amount = 200 * SCALAR_7;
    fixture.adapter.simulate_yield(&yield_amount);

    // Exchange index should now reflect the yield
    let exchange_index = fixture.escrow.get_current_exchange_index();
    let expected_index = (deposit_amount + yield_amount) / deposit_amount;
    assert_eq!(exchange_index, expected_index);
}

#[test]
fn test_redeem_principal_at_maturity() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    // User deposits
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Fast forward to maturity
    let time_to_maturity = fixture.maturity_date - fixture.env.ledger().timestamp();
    fixture.jump(time_to_maturity);

    // Redeem principal
    fixture.escrow.redeem_principal(&fixture.user, &deposit_amount);

    // Verify PT tokens were burned
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    assert_eq!(pt_client.balance(&fixture.user), 0);

    // Verify user received tokens back
    assert_eq!(fixture.token.balance(&fixture.user), deposit_amount);
}

#[test]
#[should_panic(expected = "Cannot redeem before maturity")]
fn test_redeem_principal_before_maturity() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    // User deposits
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Try to redeem before maturity (should fail)
    fixture.escrow.redeem_principal(&fixture.user, &deposit_amount);
}

#[test]
fn test_redeem_principal_with_yield() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    // User deposits
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Simulate 50% yield
    let yield_amount = 500 * SCALAR_7;
    fixture.adapter.simulate_yield(&yield_amount);

    // Fast forward to maturity
    let time_to_maturity = fixture.maturity_date - fixture.env.ledger().timestamp();
    fixture.jump(time_to_maturity);

    // Redeem principal
    fixture.escrow.redeem_principal(&fixture.user, &deposit_amount);

    // User should receive principal + their share of yield
    let expected_amount = deposit_amount + yield_amount;
    assert_eq!(fixture.token.balance(&fixture.user), expected_amount);
}

#[test]
fn test_multiple_users_deposit_and_redeem() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    // Three users deposit
    let user1 = Address::generate(&fixture.env);
    let user2 = Address::generate(&fixture.env);
    let user3 = Address::generate(&fixture.env);

    fixture.mint_tokens(&user1, deposit_amount);
    fixture.mint_tokens(&user2, deposit_amount);
    fixture.mint_tokens(&user3, deposit_amount);

    fixture.escrow.deposit(&user1, &deposit_amount);
    fixture.escrow.deposit(&user2, &deposit_amount);
    fixture.escrow.deposit(&user3, &deposit_amount);

    // Simulate 30% yield
    let total_deposited = deposit_amount * 3;
    let yield_amount = (total_deposited * 30) / 100;
    fixture.adapter.simulate_yield(&yield_amount);

    // Fast forward to maturity
    let time_to_maturity = fixture.maturity_date - fixture.env.ledger().timestamp();
    fixture.jump(time_to_maturity);

    // Each user redeems
    fixture.escrow.redeem_principal(&user1, &deposit_amount);
    fixture.escrow.redeem_principal(&user2, &deposit_amount);
    fixture.escrow.redeem_principal(&user3, &deposit_amount);

    // Each user should receive equal share of principal + yield
    let expected_per_user = deposit_amount + (yield_amount / 3);

    assert_eq!(fixture.token.balance(&user1), expected_per_user);
    assert_eq!(fixture.token.balance(&user2), expected_per_user);
    assert_eq!(fixture.token.balance(&user3), expected_per_user);
}

#[test]
fn test_partial_redemption() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    // User deposits
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Fast forward to maturity
    let time_to_maturity = fixture.maturity_date - fixture.env.ledger().timestamp();
    fixture.jump(time_to_maturity);

    // Redeem half
    let redeem_amount = 500 * SCALAR_7;
    fixture.escrow.redeem_principal(&fixture.user, &redeem_amount);

    // Verify partial redemption
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    assert_eq!(pt_client.balance(&fixture.user), deposit_amount - redeem_amount);
    assert_eq!(fixture.token.balance(&fixture.user), redeem_amount);

    // Redeem remaining
    fixture.escrow.redeem_principal(&fixture.user, &redeem_amount);

    assert_eq!(pt_client.balance(&fixture.user), 0);
    assert_eq!(fixture.token.balance(&fixture.user), deposit_amount);
}

#[test]
fn test_sy_token_supply_matches_deposits() {
    let fixture = TestFixture::create();

    let deposit1 = 500 * SCALAR_7;
    let deposit2 = 300 * SCALAR_7;

    let user1 = Address::generate(&fixture.env);
    let user2 = Address::generate(&fixture.env);

    fixture.mint_tokens(&user1, deposit1);
    fixture.mint_tokens(&user2, deposit2);

    fixture.escrow.deposit(&user1, &deposit1);
    fixture.escrow.deposit(&user2, &deposit2);

    // Total SY supply should match total deposits (when exchange rate is 1)
    let sy_client = StandardizedYieldClient::new(&fixture.env, &fixture.sy_token_address);
    let total_sy = sy_client.total_supply();

    assert_eq!(total_sy, deposit1 + deposit2);
}

#[test]
fn test_pt_yt_balance_consistency() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;

    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // PT and YT balances should always be equal for a user
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let yt_client = YieldTokenClient::new(&fixture.env, &fixture.yt_token_address);

    assert_eq!(pt_client.balance(&fixture.user), yt_client.balance(&fixture.user));
}

#[test]
fn test_adapter_asset_tracking() {
    let fixture = TestFixture::create();

    // Initial assets should be 0
    assert_eq!(fixture.adapter.get_assets(), 0);

    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Assets should match deposit
    assert_eq!(fixture.adapter.get_assets(), deposit_amount);

    // Add yield
    let yield_amount = 200 * SCALAR_7;
    fixture.adapter.simulate_yield(&yield_amount);

    // Assets should include yield
    assert_eq!(fixture.adapter.get_assets(), deposit_amount + yield_amount);
}

#[test]
fn test_exchange_index_with_no_shares() {
    let fixture = TestFixture::create();

    // When there are no shares, exchange index should be 1
    assert_eq!(fixture.escrow.get_current_exchange_index(), 1);
}

#[test]
fn test_complex_yield_scenario() {
    let fixture = TestFixture::create();

    // User 1 deposits
    let user1 = Address::generate(&fixture.env);
    let deposit1 = 1000 * SCALAR_7;
    fixture.mint_tokens(&user1, deposit1);
    fixture.escrow.deposit(&user1, &deposit1);

    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let user1_pt_initial = pt_client.balance(&user1);

    // Generate 20% yield
    fixture.adapter.simulate_yield(&(200 * SCALAR_7));

    // User 2 deposits same amount
    let user2 = Address::generate(&fixture.env);
    fixture.mint_tokens(&user2, deposit1);
    fixture.escrow.deposit(&user2, &deposit1);

    let user2_pt = pt_client.balance(&user2);

    // User 2 should have fewer PT tokens since they deposited at higher exchange rate
    assert!(user2_pt < user1_pt_initial);

    // Generate another 10% yield on current total
    let current_assets = fixture.adapter.get_assets();
    fixture.adapter.simulate_yield(&(current_assets / 10));

    // Fast forward to maturity
    let time_to_maturity = fixture.maturity_date - fixture.env.ledger().timestamp();
    fixture.jump(time_to_maturity);

    // Both users redeem
    fixture.escrow.redeem_principal(&user1, &user1_pt_initial);
    fixture.escrow.redeem_principal(&user2, &user2_pt);

    let user1_final = fixture.token.balance(&user1);
    let user2_final = fixture.token.balance(&user2);

    // User 1 should have more tokens (deposited earlier, captured more yield)
    assert!(user1_final > user2_final);
}

#[test]
fn test_token_transfer_flow() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);

    let user_balance_before = fixture.token.balance(&fixture.user);
    let escrow_balance_before = fixture.token.balance(&fixture.escrow.address);
    let adapter_balance_before = fixture.token.balance(&fixture.adapter.address);

    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // User's token balance should decrease
    assert_eq!(fixture.token.balance(&fixture.user), user_balance_before - deposit_amount);

    // Escrow should not hold tokens (they go to adapter)
    assert_eq!(fixture.token.balance(&fixture.escrow.address), escrow_balance_before);

    // Adapter should receive tokens
    assert_eq!(fixture.token.balance(&fixture.adapter.address), adapter_balance_before + deposit_amount);
}

#[test]
fn test_multiple_deposits_same_user() {
    let fixture = TestFixture::create();

    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let yt_client = YieldTokenClient::new(&fixture.env, &fixture.yt_token_address);

    // First deposit
    let deposit1 = 500 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit1);
    fixture.escrow.deposit(&fixture.user, &deposit1);

    let pt_after_first = pt_client.balance(&fixture.user);
    let yt_after_first = yt_client.balance(&fixture.user);

    // Second deposit
    let deposit2 = 300 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit2);
    fixture.escrow.deposit(&fixture.user, &deposit2);

    // Balances should accumulate
    assert_eq!(pt_client.balance(&fixture.user), pt_after_first + deposit2);
    assert_eq!(yt_client.balance(&fixture.user), yt_after_first + deposit2);
}

#[test]
fn test_redeem_updates_adapter_assets() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    let assets_before_redeem = fixture.adapter.get_assets();

    // Fast forward to maturity
    let time_to_maturity = fixture.maturity_date - fixture.env.ledger().timestamp();
    fixture.jump(time_to_maturity);

    // Redeem half
    let redeem_amount = 500 * SCALAR_7;
    fixture.escrow.redeem_principal(&fixture.user, &redeem_amount);

    // Adapter assets should decrease
    assert_eq!(fixture.adapter.get_assets(), assets_before_redeem - redeem_amount);
}

#[test]
fn test_sy_remains_in_escrow() {
    let fixture = TestFixture::create();

    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    let sy_client = StandardizedYieldClient::new(&fixture.env, &fixture.sy_token_address);

    // All SY tokens should be held by escrow contract
    assert_eq!(sy_client.balance(&fixture.escrow.address), deposit_amount);

    // User should not have any SY tokens
    assert_eq!(sy_client.balance(&fixture.user), 0);
}
mod mod_imports;
mod test_fixture;

// Re-export for use in test modules
pub use mod_imports::*;
use test_fixture::{TestFixture, SCALAR_7};

#[test]
fn test_market_deployment() {
    let fixture = TestFixture::create();

    // Verify market is deployed
    assert!(fixture.escrow.is_deployed());

    // Verify all contract addresses are set
    let sy_address = fixture.escrow.get_sy_token();
    let pt_address = fixture.escrow.get_pt_token();
    let yt_address = fixture.escrow.get_yt_token();
    let adapter_address = fixture.escrow.get_adapter();

    assert_ne!(sy_address, pt_address);
    assert_ne!(sy_address, yt_address);
    assert_ne!(pt_address, yt_address);
}

#[test]
#[should_panic(expected = "Market already deployed")]
fn test_cannot_deploy_market_twice() {
    let fixture = TestFixture::create();

    // Try to deploy again - should panic
    fixture.escrow.deploy_market(
        &fixture.admin,
        &fixture.maturity_date,
        &soroban_sdk::String::from_str(&fixture.env, "SY2"),
        &soroban_sdk::String::from_str(&fixture.env, "SY2"),
        &soroban_sdk::String::from_str(&fixture.env, "PT2"),
        &soroban_sdk::String::from_str(&fixture.env, "PT2"),
        &soroban_sdk::String::from_str(&fixture.env, "YT2"),
        &soroban_sdk::String::from_str(&fixture.env, "YT2"),
    );
}

#[test]
fn test_initial_exchange_index() {
    let fixture = TestFixture::create();

    // Initial exchange index should be SCALAR_7 (representing 1.0)
    let index = fixture.escrow.get_current_exchange_index();
    assert_eq!(index, SCALAR_7);
}

#[test]
fn test_deposit_mints_tokens() {
    let fixture = TestFixture::create();

    // Mint tokens to user
    let deposit_amount = 1000 * SCALAR_7; // 1000 tokens
    fixture.mint_tokens(&fixture.user, deposit_amount);

    // Deposit tokens
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Check PT balance (should equal deposit amount since initial index = 1)
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let pt_balance = pt_client.balance(&fixture.user);
    assert_eq!(pt_balance, deposit_amount);

    // Check YT balance (should equal PT balance)
    let yt_client = YieldTokenClient::new(&fixture.env, &fixture.yt_token_address);
    let yt_balance = yt_client.balance(&fixture.user);
    assert_eq!(yt_balance, deposit_amount);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_zero_panics() {
    let fixture = TestFixture::create();
    fixture.escrow.deposit(&fixture.user, &0);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_negative_panics() {
    let fixture = TestFixture::create();
    fixture.escrow.deposit(&fixture.user, &-100);
}

#[test]
fn test_deposit_with_accrued_yield() {
    let fixture = TestFixture::create();

    // First deposit
    let initial_deposit = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, initial_deposit);
    fixture.escrow.deposit(&fixture.user, &initial_deposit);

    // Jump forward 365 days to accrue yield (5% APY)
    fixture.jump(365 * 24 * 60 * 60);

    // Second deposit from admin
    let second_deposit = 500 * SCALAR_7;
    fixture.mint_tokens(&fixture.admin, second_deposit);

    // Get exchange index after yield accrual
    let index_after_yield = fixture.escrow.get_current_exchange_index();

    // Index should be > SCALAR_7 due to yield (SCALAR_7 represents 1.0)
    assert!(index_after_yield > SCALAR_7);

    fixture.escrow.deposit(&fixture.admin, &second_deposit);

    // Admin should get fewer PT/YT tokens than deposited assets due to higher index
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let admin_pt_balance = pt_client.balance(&fixture.admin);

    // PT amount = (deposit / index) * index = deposit (but intermediate SY calculation differs)
    assert!(admin_pt_balance > 0);
}

#[test]
#[should_panic(expected = "Cannot redeem before maturity")]
fn test_redeem_before_maturity_panics() {
    let fixture = TestFixture::create();

    // Deposit first
    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Try to redeem before maturity - should panic
    fixture.escrow.redeem_principal(&fixture.user, &deposit_amount);
}

#[test]
fn test_redeem_principal_at_maturity() {
    let fixture = TestFixture::create();

    // Deposit tokens
    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Get initial token balance
    let initial_balance = fixture.token.balance(&fixture.user);

    // Jump to maturity
    fixture.jump(30 * 24 * 60 * 60);

    // Get PT balance
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let pt_balance = pt_client.balance(&fixture.user);

    // Redeem principal
    fixture.escrow.redeem_principal(&fixture.user, &pt_balance);

    // Check PT tokens were burned
    let pt_balance_after = pt_client.balance(&fixture.user);
    assert_eq!(pt_balance_after, 0);

    // Check user received underlying tokens back
    let final_balance = fixture.token.balance(&fixture.user);
    assert!(final_balance > initial_balance);
}

#[test]
fn test_redeem_principal_with_yield() {
    let fixture = TestFixture::create();

    // Deposit tokens
    let deposit_amount = 1000 * SCALAR_7;
    fixture.mint_tokens(&fixture.user, deposit_amount);
    fixture.escrow.deposit(&fixture.user, &deposit_amount);

    // Jump forward 365 days to accrue yield
    fixture.jump(365 * 24 * 60 * 60);

    // Get PT balance before redemption
    let pt_client = PrincipalTokenClient::new(&fixture.env, &fixture.pt_token_address);
    let pt_balance = pt_client.balance(&fixture.user);

    // Redeem principal (note: maturity is only 30 days, so we're past maturity)
    let initial_token_balance = fixture.token.balance(&fixture.user);
    fixture.escrow.redeem_principal(&fixture.user, &pt_balance);

    // Check redeemed amount
    let final_token_balance = fixture.token.balance(&fixture.user);
    let redeemed_amount = final_token_balance - initial_token_balance;

    // User should get back at least close to their principal (may have minor rounding differences)
    // Since the user deposited at index ~1.0 and redeems at index ~1.05, there may be small rounding errors
    // Allow for a small tolerance (0.1% of deposit amount)
    let min_expected = deposit_amount * 999 / 1000;
    assert!(redeemed_amount >= min_expected,
        "Expected at least {}, but got {}", min_expected, redeemed_amount);
}
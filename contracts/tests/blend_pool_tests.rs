use blend_pool_sim::{BlendPoolSimulator, BlendPoolSimulatorClient};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, Address, Env,
};

#[test]
fn test_deposit_and_withdrawal_with_interest() {
    let env = Env::default();
    env.mock_all_auths();

    // Create and register token
    let token_id = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let token_client = token::Client::new(&env, &token_id.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id.address());

    // Create pool contract with constructor
    let interest_rate_bps = 1000u32;
    let pool_contract_id = env.register(
        BlendPoolSimulator,
        (&token_id.address(), &interest_rate_bps),
    );
    let pool_client = BlendPoolSimulatorClient::new(&env, &pool_contract_id);

    // Set pool as admin to allow minting interest
    token_admin_client.set_admin(&pool_contract_id);

    // Create user and mint tokens
    let user = Address::generate(&env);
    token_admin_client.mint(&user, &1_000_000);

    // Deposit 100,000 tokens
    let deposit_amount = 100_000i128;
    pool_client.deposit(&user, &deposit_amount);

    // Verify initial balance
    let initial_balance = pool_client.balance(&user);
    assert_eq!(initial_balance, deposit_amount);

    // Fast-forward time by 1 year (31536000 seconds)
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp.saturating_add(31_536_000);
    });

    // Check balance after 1 year - should be ~110,000 (10% interest)
    let balance_after_year = pool_client.balance(&user);
    let expected_balance = 110_000i128;

    // Allow small margin for rounding
    assert!(balance_after_year >= expected_balance - 1 && balance_after_year <= expected_balance + 1,
        "Expected ~{}, got {}", expected_balance, balance_after_year);

    // Verify accrued interest
    let accrued = pool_client.accrued_interest(&user);
    let expected_interest = 10_000i128;
    assert!(accrued >= expected_interest - 1 && accrued <= expected_interest + 1,
        "Expected interest ~{}, got {}", expected_interest, accrued);

    // Withdraw everything
    pool_client.withdraw(&user, &balance_after_year);

    // Verify withdrawal
    let final_balance = pool_client.balance(&user);
    assert_eq!(final_balance, 0);

    // Verify user received tokens (original deposit + interest)
    let user_token_balance = token_client.balance(&user);
    assert_eq!(user_token_balance, 1_000_000 - deposit_amount + balance_after_year);
}

#[test]
fn test_partial_withdrawal_with_interest() {
    let env = Env::default();
    env.mock_all_auths();

    // Create and register token
    let token_id = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let token_client = token::Client::new(&env, &token_id.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id.address());

    // Create pool contract with constructor
    let interest_rate_bps = 500u32;
    let pool_contract_id = env.register(
        BlendPoolSimulator,
        (&token_id.address(), &interest_rate_bps),
    );
    let pool_client = BlendPoolSimulatorClient::new(&env, &pool_contract_id);

    // Set pool as admin
    token_admin_client.set_admin(&pool_contract_id);

    // Create user and mint tokens
    let user = Address::generate(&env);
    token_admin_client.mint(&user, &1_000_000);

    // Deposit 200,000 tokens
    let deposit_amount = 200_000i128;
    pool_client.deposit(&user, &deposit_amount);

    // Fast forward 6 months (15768000 seconds)
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp.saturating_add(15_768_000);
    });

    // Balance should be ~205,000 (2.5% for 6 months)
    let balance_after_6_months = pool_client.balance(&user);
    let expected_balance = 205_000i128;
    assert!(balance_after_6_months >= expected_balance - 1 && balance_after_6_months <= expected_balance + 1,
        "Expected ~{}, got {}", expected_balance, balance_after_6_months);

    // Withdraw half
    let withdraw_amount = 100_000i128;
    pool_client.withdraw(&user, &withdraw_amount);

    // Remaining balance should be updated
    let remaining_balance = pool_client.balance(&user);
    assert!(remaining_balance >= 105_000 - 1 && remaining_balance <= 105_000 + 1);
}

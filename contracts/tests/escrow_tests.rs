#![cfg(test)]

use soroban_sdk::{
    contract, contractimpl, testutils::{Address as _, Ledger}, Address, BytesN, Env,
};

// Import the escrow contract
mod escrow {
    soroban_sdk::contractimport!(
        file = "../wasms/escrow.wasm"
    );
}

mod standardized_yield {
    soroban_sdk::contractimport!(
        file = "../wasms/standardized_yield.wasm"
    );
}

mod principal_token {
    soroban_sdk::contractimport!(
        file = "../wasms/principal_token.wasm"
    );
}

mod yield_token {
    soroban_sdk::contractimport!(
        file = "../wasms/yield_token.wasm"
    );
}

use soroban_sdk::token::{StellarAssetClient, TokenClient};
use adapter_trait::YieldAdapter;

// Mock yield adapter for testing
#[contract]
pub struct MockYieldAdapter;

#[contractimpl]
impl YieldAdapter for MockYieldAdapter {
    fn __constructor(env: Env, escrow: Address, yield_protocol: Address, token: Address) {
        env.storage().instance().set(&soroban_sdk::symbol_short!("escrow"), &escrow);
        env.storage().instance().set(&soroban_sdk::symbol_short!("protocol"), &yield_protocol);
        env.storage().instance().set(&soroban_sdk::symbol_short!("token"), &token);
        env.storage().instance().set(&soroban_sdk::symbol_short!("assets"), &0i128);
    }

    fn deposit(env: Env, depositor: Address, amount: i128) {
        depositor.require_auth();

        // Transfer tokens from depositor to this adapter
        let token_address: Address = env.storage().instance()
            .get(&soroban_sdk::symbol_short!("token"))
            .unwrap();
        let token = TokenClient::new(&env, &token_address);
        token.transfer(&depositor, &env.current_contract_address(), &amount);

        // Track total assets
        let current_assets: i128 = env.storage().instance()
            .get(&soroban_sdk::symbol_short!("assets"))
            .unwrap_or(0);
        env.storage().instance().set(&soroban_sdk::symbol_short!("assets"), &(current_assets + amount));
    }

    fn withdraw(env: Env, amount: i128) {
        let escrow_address: Address = env.storage().instance()
            .get(&soroban_sdk::symbol_short!("escrow"))
            .unwrap();

        // Transfer tokens from adapter to escrow
        let token_address: Address = env.storage().instance()
            .get(&soroban_sdk::symbol_short!("token"))
            .unwrap();
        let token = TokenClient::new(&env, &token_address);
        token.transfer(&env.current_contract_address(), &escrow_address, &amount);

        // Update total assets
        let current_assets: i128 = env.storage().instance()
            .get(&soroban_sdk::symbol_short!("assets"))
            .unwrap_or(0);
        env.storage().instance().set(&soroban_sdk::symbol_short!("assets"), &(current_assets - amount));
    }

    fn get_yield_protocol(env: Env) -> Address {
        env.storage().instance()
            .get(&soroban_sdk::symbol_short!("protocol"))
            .unwrap()
    }

    fn get_token(env: Env) -> Address {
        env.storage().instance()
            .get(&soroban_sdk::symbol_short!("token"))
            .unwrap()
    }

    fn get_assets(env: Env) -> i128 {
        env.storage().instance()
            .get(&soroban_sdk::symbol_short!("assets"))
            .unwrap_or(0)
    }
}

fn create_token_contract<'a>(env: &Env, admin: &Address) -> (TokenClient<'a>, Address) {
    let contract_address = env.register_stellar_asset_contract_v2(admin.clone());
    (
        TokenClient::new(env, &contract_address.address()),
        contract_address.address(),
    )
}

fn setup_test_env() -> (
    Env,
    Address,
    Address,
    Address,
    escrow::Client<'static>,
    TokenClient<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let blend_pool = Address::generate(&env);

    // Create test token
    let (token_client, token_address) = create_token_contract(&env, &admin);

    // Mint tokens to admin
    let stellar_admin = StellarAssetClient::new(&env, &token_address);
    stellar_admin.mint(&admin, &1_000_000_000);

    let maturity_date = env.ledger().timestamp() + 86400 * 30; // 30 days from now

    // Deploy escrow contract (without constructor - we'll set up state manually)
    let escrow_address = env.register(escrow::WASM, ());

    // Deploy mock adapter
    let adapter_address = env.register(MockYieldAdapter, ());

    // Deploy SY, PT, YT tokens with escrow as admin
    let sy_address = env.deployer().deploy_v2(
        &env.deployer().upload_contract_wasm(standardized_yield::WASM),
        (escrow_address.clone(), soroban_sdk::String::from_str(&env, "SY"), soroban_sdk::String::from_str(&env, "SY"))
    );

    let pt_address = env.deployer().deploy_v2(
        &env.deployer().upload_contract_wasm(principal_token::WASM),
        (escrow_address.clone(), soroban_sdk::String::from_str(&env, "PT"), soroban_sdk::String::from_str(&env, "PT"))
    );

    let yt_address = env.deployer().deploy_v2(
        &env.deployer().upload_contract_wasm(yield_token::WASM),
        (escrow_address.clone(), soroban_sdk::String::from_str(&env, "YT"), soroban_sdk::String::from_str(&env, "YT"))
    );

    // Get clients
    let escrow_client = escrow::Client::new(&env, &escrow_address);
    let adapter_client = crate::MockYieldAdapterClient::new(&env, &adapter_address);

    // Initialize adapter
    adapter_client.__constructor(&escrow_address, &blend_pool, &token_address);

    // Manually set escrow storage since we can't use the constructor
    use soroban_sdk::IntoVal;
    env.as_contract(&escrow_address, || {
        env.storage().instance().set(&"BlendPool".into_val(&env), &blend_pool);
        env.storage().instance().set(&"Token".into_val(&env), &token_address);
        env.storage().instance().set(&"MaturityDate".into_val(&env), &maturity_date);
        env.storage().instance().set(&"Adapter".into_val(&env), &adapter_address);
        env.storage().instance().set(&"SYToken".into_val(&env), &sy_address);
        env.storage().instance().set(&"PTToken".into_val(&env), &pt_address);
        env.storage().instance().set(&"YTToken".into_val(&env), &yt_address);
    });

    (env, admin, user, escrow_address, escrow_client, token_client)
}

#[test]
fn test_escrow_initialization() {
    let (env, admin, _user, _escrow_address, escrow_client, _token_client) = setup_test_env();

    let sy_token = escrow_client.get_sy_token();
    let pt_token = escrow_client.get_pt_token();
    let yt_token = escrow_client.get_yt_token();
    let adapter = escrow_client.get_adapter();

    assert!(sy_token != Address::generate(&env));
    assert!(pt_token != Address::generate(&env));
    assert!(yt_token != Address::generate(&env));
    assert!(adapter != Address::generate(&env));
}

#[test]
fn test_deposit_basic() {
    let (env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    // Transfer tokens to user
    token_client.transfer(&admin, &user, &1000);

    // Approve escrow to spend user's tokens
    token_client.approve(&user, &escrow_address, &1000, &99999);

    // Deposit
    escrow_client.deposit(&user, &1000);

    // Verify PT and YT tokens were minted to user
    let pt_token_address = escrow_client.get_pt_token();
    let yt_token_address = escrow_client.get_yt_token();

    let pt_client = TokenClient::new(&env, &pt_token_address);
    let yt_client = TokenClient::new(&env, &yt_token_address);

    let user_pt_balance = pt_client.balance(&user);
    let user_yt_balance = yt_client.balance(&user);

    assert_eq!(user_pt_balance, 1000);
    assert_eq!(user_yt_balance, 1000);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_zero_amount() {
    let (_env, _admin, user, _escrow_address, escrow_client, _token_client) = setup_test_env();
    escrow_client.deposit(&user, &0);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_negative_amount() {
    let (_env, _admin, user, _escrow_address, escrow_client, _token_client) = setup_test_env();
    escrow_client.deposit(&user, &-100);
}

#[test]
fn test_multiple_deposits() {
    let (env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    // Transfer tokens to user
    token_client.transfer(&admin, &user, &2000);
    token_client.approve(&user, &escrow_address, &2000, &99999);

    // First deposit
    escrow_client.deposit(&user, &500);

    // Second deposit
    escrow_client.deposit(&user, &1000);

    // Verify total PT and YT tokens
    let pt_token_address = escrow_client.get_pt_token();
    let yt_token_address = escrow_client.get_yt_token();

    let pt_client = TokenClient::new(&env, &pt_token_address);
    let yt_client = TokenClient::new(&env, &yt_token_address);

    assert_eq!(pt_client.balance(&user), 1500);
    assert_eq!(yt_client.balance(&user), 1500);
}

#[test]
fn test_redeem_principal_at_maturity() {
    let (env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    // Transfer and deposit tokens
    token_client.transfer(&admin, &user, &1000);
    token_client.approve(&user, &escrow_address, &1000, &99999);
    escrow_client.deposit(&user, &1000);

    // Fast forward to maturity
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 86400 * 31; // 31 days later
    });

    // Get PT balance before redemption
    let pt_token_address = escrow_client.get_pt_token();
    let pt_client = TokenClient::new(&env, &pt_token_address);
    let pt_balance_before = pt_client.balance(&user);

    // Approve escrow to burn PT tokens
    pt_client.approve(&user, &escrow_address, &1000, &99999);

    // Redeem principal
    escrow_client.redeem_principal(&user, &1000);

    // Verify PT tokens were burned
    let pt_balance_after = pt_client.balance(&user);
    assert_eq!(pt_balance_after, pt_balance_before - 1000);

    // Verify user received underlying tokens
    let user_token_balance = token_client.balance(&user);
    assert!(user_token_balance > 0);
}

#[test]
#[should_panic(expected = "Cannot redeem before maturity")]
fn test_redeem_principal_before_maturity() {
    let (_env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    // Transfer and deposit tokens
    token_client.transfer(&admin, &user, &1000);
    token_client.approve(&user, &escrow_address, &1000, &99999);
    escrow_client.deposit(&user, &1000);

    // Try to redeem before maturity (should fail)
    escrow_client.redeem_principal(&user, &1000);
}

#[test]
fn test_exchange_index_calculation() {
    let (_env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    // Initial exchange index should be 1
    let initial_index = escrow_client.get_current_exchange_index();
    assert_eq!(initial_index, 1);

    // After deposit, index should still be 1 (no yield accumulated)
    token_client.transfer(&admin, &user, &1000);
    token_client.approve(&user, &escrow_address, &1000, &99999);
    escrow_client.deposit(&user, &1000);

    let index_after_deposit = escrow_client.get_current_exchange_index();
    assert_eq!(index_after_deposit, 1);
}

#[test]
fn test_pt_yt_equal_quantities() {
    let (env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    token_client.transfer(&admin, &user, &5000);
    token_client.approve(&user, &escrow_address, &5000, &99999);
    escrow_client.deposit(&user, &5000);

    let pt_token_address = escrow_client.get_pt_token();
    let yt_token_address = escrow_client.get_yt_token();

    let pt_client = TokenClient::new(&env, &pt_token_address);
    let yt_client = TokenClient::new(&env, &yt_token_address);

    // PT and YT should always be minted in equal quantities
    assert_eq!(pt_client.balance(&user), yt_client.balance(&user));
}

#[test]
fn test_multiple_users_deposit() {
    let (env, admin, user1, escrow_address, escrow_client, token_client) = setup_test_env();
    let user2 = Address::generate(&env);

    // User 1 deposits
    token_client.transfer(&admin, &user1, &1000);
    token_client.approve(&user1, &escrow_address, &1000, &99999);
    escrow_client.deposit(&user1, &1000);

    // User 2 deposits
    token_client.transfer(&admin, &user2, &2000);
    token_client.approve(&user2, &escrow_address, &2000, &99999);
    escrow_client.deposit(&user2, &2000);

    let pt_token_address = escrow_client.get_pt_token();
    let pt_client = TokenClient::new(&env, &pt_token_address);

    assert_eq!(pt_client.balance(&user1), 1000);
    assert_eq!(pt_client.balance(&user2), 2000);
}

#[test]
fn test_redeem_partial_amount() {
    let (env, admin, user, escrow_address, escrow_client, token_client) = setup_test_env();

    // Deposit
    token_client.transfer(&admin, &user, &1000);
    token_client.approve(&user, &escrow_address, &1000, &99999);
    escrow_client.deposit(&user, &1000);

    // Fast forward to maturity
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 86400 * 31;
    });

    let pt_token_address = escrow_client.get_pt_token();
    let pt_client = TokenClient::new(&env, &pt_token_address);
    pt_client.approve(&user, &escrow_address, &500, &99999);

    // Redeem partial amount
    escrow_client.redeem_principal(&user, &500);

    // Should still have 500 PT tokens left
    assert_eq!(pt_client.balance(&user), 500);
}

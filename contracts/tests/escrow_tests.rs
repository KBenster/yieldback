use escrow::{EscrowContract, EscrowContractClient};
use standardized_yield::StandardizedYieldClient;
use yield_pool_sim::{YieldPoolSimulator, YieldPoolSimulatorClient};
use soroban_sdk::{
    testutils::Address as _,
    token, Address, Bytes, Env, String,
};


mod standardized_yield_wasm {
    soroban_sdk::contractimport!(
        file = "../wasms/standardized_yield.wasm"
    );
}

mod principal_token_wasm {
    soroban_sdk::contractimport!(
        file = "../wasms/principal_token.wasm"
    );
}

#[test]
fn test_escrow_constructor() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let blend_pool = Address::generate(&env);
    let token = Address::generate(&env);
    let sy_wasm_hash = env.deployer().upload_contract_wasm(standardized_yield_wasm::WASM);
    let pt_wasm_hash = env.deployer().upload_contract_wasm(principal_token_wasm::WASM);
    let maturity_date = 1000000u64;

    let contract_id = env.register(
        EscrowContract,
        (&admin, &blend_pool, &token, &sy_wasm_hash, &pt_wasm_hash, &maturity_date),
    );
    let _client = EscrowContractClient::new(&env, &contract_id);
}


#[test]
fn test_deposit() {
    let env = Env::default();
    env.mock_all_auths_allowing_non_root_auth();

    // Setup token
    let token_id = env.register_stellar_asset_contract_v2(Address::generate(&env));
    let token_client = token::Client::new(&env, &token_id.address());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id.address());

    // Setup blend pool
    let interest_rate_bps = 1000u32;
    let pool_contract_id = env.register(
        YieldPoolSimulator,
        (&token_id.address(), &interest_rate_bps),
    );
    let pool_client = YieldPoolSimulatorClient::new(&env, &pool_contract_id);
    token_admin_client.set_admin(&pool_contract_id);

    // Setup escrow contract
    let admin = Address::generate(&env);
    let sy_wasm_hash = env.deployer().upload_contract_wasm(standardized_yield_wasm::WASM);
    let pt_wasm_hash = env.deployer().upload_contract_wasm(principal_token_wasm::WASM);
    let maturity_date = 1000000u64;

    let escrow_contract_id = env.register(
        EscrowContract,
        (&admin, &pool_contract_id, &token_id.address(), &sy_wasm_hash, &pt_wasm_hash, &maturity_date),
    );
    let escrow_client = EscrowContractClient::new(&env, &escrow_contract_id);

    // Get SY token address
    let sy_token_address = escrow_client.get_sy_token();
    let sy_client = StandardizedYieldClient::new(&env, &sy_token_address);

    // Create user and mint tokens
    let user = Address::generate(&env);
    token_admin_client.mint(&user, &1_000_000);

    // Deposit
    let deposit_amount = 100_000i128;
    escrow_client.deposit(&user, &deposit_amount);

    // Verify user balance in blend pool
    let pool_balance = pool_client.balance(&user);
    assert_eq!(pool_balance, deposit_amount);

    // Verify SY tokens were minted to escrow contract
    let sy_balance = sy_client.balance(&escrow_contract_id);
    assert_eq!(sy_balance, deposit_amount); // 1:1 exchange rate initially
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_zero_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let blend_pool = Address::generate(&env);
    let token = Address::generate(&env);
    let sy_wasm_hash = env.deployer().upload_contract_wasm(standardized_yield_wasm::WASM);
    let pt_wasm_hash = env.deployer().upload_contract_wasm(principal_token_wasm::WASM);
    let maturity_date = 1000000u64;

    let contract_id = env.register(
        EscrowContract,
        (&admin, &blend_pool, &token, &sy_wasm_hash, &pt_wasm_hash, &maturity_date),
    );
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&user, &0);
}

#[test]
#[should_panic(expected = "Amount must be positive")]
fn test_deposit_negative_amount() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let blend_pool = Address::generate(&env);
    let token = Address::generate(&env);
    let sy_wasm_hash = env.deployer().upload_contract_wasm(standardized_yield_wasm::WASM);
    let pt_wasm_hash = env.deployer().upload_contract_wasm(principal_token_wasm::WASM);
    let maturity_date = 1000000u64;

    let contract_id = env.register(
        EscrowContract,
        (&admin, &blend_pool, &token, &sy_wasm_hash, &pt_wasm_hash, &maturity_date),
    );
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&user, &-100);
}

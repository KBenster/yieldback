#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::{StellarAssetClient, TokenClient},
    Address, Env, String,
};
use soroban_sdk::testutils::MockAuth;

fn create_token_contract(env: &Env, admin: &Address) -> Address {
    let token_address = env.register_stellar_asset_contract(admin.clone());
    token_address
}

fn create_bond_wrapper_contract(env: &Env) -> Address {
    env.register_contract(None, BondWrapper {})
}

struct TestSetup {
    env: Env,
    admin: Address,
    sponsor: Address,
    treasury: Address,
    user1: Address,
    user2: Address,
    bond_wrapper_addr: Address,
    base_asset: Address,
    bond_token: Address,
    blend_pool: Address,
    blend_token: Address,
}

impl TestSetup {
    fn bond_wrapper(&self) -> BondWrapperClient {
        BondWrapperClient::new(&self.env, &self.bond_wrapper_addr)
    }
}

impl TestSetup {
    fn new() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let sponsor = Address::generate(&env);
        let treasury = Address::generate(&env);
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        // Create token contracts
        let base_asset = create_token_contract(&env, &admin);
        let bond_token = create_token_contract(&env, &admin);
        let blend_pool = Address::generate(&env); // Mock pool address
        let blend_token = create_token_contract(&env, &admin);

        let bond_wrapper_addr = create_bond_wrapper_contract(&env);

        // Mint some tokens for testing
        let base_admin_client = StellarAssetClient::new(&env, &base_asset);
        let blend_admin_client = StellarAssetClient::new(&env, &blend_token);

        base_admin_client.mint(&user1, &10_000_000); // 10M tokens
        base_admin_client.mint(&user2, &5_000_000);  // 5M tokens
        base_admin_client.mint(&sponsor, &20_000_000); // 20M tokens for insurance
        blend_admin_client.mint(&bond_wrapper_addr, &1_000_000); // Mock BLND rewards

        TestSetup {
            env,
            admin,
            sponsor,
            treasury,
            user1,
            user2,
            bond_wrapper_addr,
            base_asset,
            bond_token,
            blend_pool,
            blend_token,
        }
    }

    fn initialize_bond(&self, coupon_funding: i128) -> SponsorBondConfig {
        let config = SponsorBondConfig {
            sponsor: self.sponsor.clone(),
            treasury: self.treasury.clone(),
            bond_token: self.bond_token.clone(),
            base_asset: self.base_asset.clone(),
            blend_pool: self.blend_pool.clone(),
            blend_token: self.blend_token.clone(),
            fixed_rate: 500, // 5%
            bond_duration: 365 * 24 * 3600, // 1 year
            min_deposit: 1000,
            max_total_deposit: 1_000_000, // Changed from max_deposit to max_total_deposit
        };

        self.bond_wrapper().create_position(&config, &coupon_funding);
        config
    }
}

// Test client type for the bond wrapper contract
use soroban_sdk::{contractclient, BytesN};
use soroban_sdk::testutils::arbitrary::std::{dbg, println};

#[contractclient(name = "BondWrapperClient")]
pub trait BondWrapperTrait {
    fn initialize(env: Env, config: BondConfig);
    fn deposit(env: Env, user: Address, amount: i128);
    fn harvest(env: Env);
    fn redeem(env: Env, user: Address) -> i128;
    fn emergency_withdraw(env: Env, admin: Address);
    fn add_insurance_reserve(env: Env, sponsor: Address, amount: i128);
    fn get_bond_info(env: Env) -> BondInfo;
    fn get_user_position(env: Env, user: Address) -> UserPosition;
    fn calculate_coupon_payment(env: Env, bond_amount: i128) -> i128;
}
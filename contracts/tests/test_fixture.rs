use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, String as SorobanString,
};

use escrow::{EscrowContract, EscrowContractClient};
use escrow::escrow::MarketInitMeta;
use crate::{
    MockTokenClient, MOCK_TOKEN_WASM,
    MockAdapterClient, MOCK_ADAPTER_WASM,
    MockYieldProtocolClient, MOCK_YIELD_PROTOCOL_WASM,
    SY_WASM, PT_WASM, YT_WASM,
};

pub const SCALAR_7: i128 = 1_000_0000;

pub struct TestFixture<'a> {
    pub env: Env,
    pub admin: Address,
    pub user: Address,
    pub escrow: EscrowContractClient<'a>,
    pub token: MockTokenClient<'a>,
    pub token_address: Address,
    pub yield_protocol: MockYieldProtocolClient<'a>,
    pub yield_protocol_address: Address,
    pub adapter: MockAdapterClient<'a>,
    pub adapter_address: Address,
    pub sy_token_address: Address,
    pub pt_token_address: Address,
    pub yt_token_address: Address,
    pub maturity_date: u64,
}

impl TestFixture<'_> {
    pub fn create<'a>() -> TestFixture<'a> {
        let env = Env::default();
        env.mock_all_auths();
        env.cost_estimate().budget().reset_unlimited();

        let admin = Address::generate(&env);
        let user = Address::generate(&env);

        env.ledger().set(LedgerInfo {
            timestamp: 1_000_000,
            protocol_version: 22,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 500000,
            min_persistent_entry_ttl: 500000,
            max_entry_ttl: 9999999,
        });

        // Deploy mock token using WASM
        let token_address = env.register(
            MOCK_TOKEN_WASM,
            (
                &admin,
                &SorobanString::from_str(&env, "Test Token"),
                &SorobanString::from_str(&env, "TEST"),
                &7u32,
            ),
        );
        let token = MockTokenClient::new(&env, &token_address);

        // Deploy mock yield protocol with 5% APY and 100M token reserve
        let initial_reserve = 100_000_000 * SCALAR_7; // 100M tokens for paying yield

        // Mint tokens to admin for funding the protocol
        token.mint(&admin, &initial_reserve);

        let yield_protocol_address = env.register(
            MOCK_YIELD_PROTOCOL_WASM,
            (
                &token_address,
                &500u32,              // 5% APY
                &admin,
                &initial_reserve,
            ),
        );
        let yield_protocol = MockYieldProtocolClient::new(&env, &yield_protocol_address);

        // Upload contract WASMs once - following Blend's pattern
        let adapter_wasm_hash = env.deployer().upload_contract_wasm(MOCK_ADAPTER_WASM);
        let sy_wasm_hash = env.deployer().upload_contract_wasm(SY_WASM);
        let pt_wasm_hash = env.deployer().upload_contract_wasm(PT_WASM);
        let yt_wasm_hash = env.deployer().upload_contract_wasm(YT_WASM);

        // Set maturity date to 30 days from now
        let maturity_date = env.ledger().timestamp() + (30 * 24 * 60 * 60);

        // Create market metadata
        let market_meta = MarketInitMeta {
            yield_source: yield_protocol_address.clone(),
            token: token_address.clone(),
            sy_wasm_hash: sy_wasm_hash.clone(),
            pt_wasm_hash: pt_wasm_hash.clone(),
            yt_wasm_hash: yt_wasm_hash.clone(),
            adapter_wasm_hash: adapter_wasm_hash.clone(),
        };

        // Deploy escrow contract - constructor only stores config
        let escrow_address = Address::generate(&env);
        env.register_at(
            &escrow_address,
            EscrowContract,
            (&admin, &market_meta),
        );
        let escrow = EscrowContractClient::new(&env, &escrow_address);

        // Deploy the market - this actually deploys all sub-contracts
        escrow.deploy_market(
            &admin,
            &maturity_date,
            &SorobanString::from_str(&env, "Standardized Yield USDC"),
            &SorobanString::from_str(&env, "SY-USDC"),
            &SorobanString::from_str(&env, "Principal Token USDC"),
            &SorobanString::from_str(&env, "PT-USDC"),
            &SorobanString::from_str(&env, "Yield Token USDC"),
            &SorobanString::from_str(&env, "YT-USDC"),
        );

        // Get deployed contract addresses from escrow
        let adapter_address = escrow.get_adapter();
        let sy_token_address = escrow.get_sy_token();
        let pt_token_address = escrow.get_pt_token();
        let yt_token_address = escrow.get_yt_token();

        // Create adapter client
        let adapter = MockAdapterClient::new(&env, &adapter_address);

        TestFixture {
            env,
            admin,
            user,
            escrow,
            token,
            token_address,
            yield_protocol,
            yield_protocol_address,
            adapter,
            adapter_address,
            sy_token_address,
            pt_token_address,
            yt_token_address,
            maturity_date,
        }
    }

    /// Jump forward in time by the specified number of seconds
    pub fn jump(&self, time: u64) {
        self.env.ledger().set(LedgerInfo {
            timestamp: self.env.ledger().timestamp().saturating_add(time),
            protocol_version: 22,
            sequence_number: self.env.ledger().sequence(),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 999999,
            min_persistent_entry_ttl: 999999,
            max_entry_ttl: 9999999,
        });
    }

    /// Mint tokens to a specific address
    pub fn mint_tokens(&self, to: &Address, amount: i128) {
        self.token.mint(to, &amount);
    }
}
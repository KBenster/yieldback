use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger, LedgerInfo},
    Address, BytesN, Env, String,
};

use escrow::{EscrowContract, EscrowContractClient};
use standardized_yield::StandardizedYield;
use principal_token::PrincipalToken;
use yield_token::YieldToken;

use crate::mock_adapter::{MockAdapter, MockAdapterClient};
use crate::mock_token::{create_mock_token, MockTokenClient};

pub const SCALAR_7: i128 = 1_000_0000;

pub struct TestFixture<'a> {
    pub env: Env,
    pub admin: Address,
    pub user: Address,
    pub escrow: EscrowContractClient<'a>,
    pub escrow_address: Address,
    pub token: MockTokenClient<'a>,
    pub token_address: Address,
    pub blend_pool: Address,
    pub adapter: MockAdapterClient<'a>,
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

        // Create mock token
        let (token_address, token_client) = create_mock_token(&env, &admin);

        // Create blend pool address (mock - not actually used in tests but required for initialization)
        let blend_pool = Address::generate(&env);

        // Upload contract WASMs
        let adapter_wasm = MockAdapter.contract_wasm();
        let adapter_wasm_hash = env.deployer().upload_contract_wasm(adapter_wasm);

        let sy_wasm = StandardizedYield.contract_wasm();
        let sy_wasm_hash = env.deployer().upload_contract_wasm(sy_wasm);

        let pt_wasm = PrincipalToken.contract_wasm();
        let pt_wasm_hash = env.deployer().upload_contract_wasm(pt_wasm);

        let yt_wasm = YieldToken.contract_wasm();
        let yt_wasm_hash = env.deployer().upload_contract_wasm(yt_wasm);

        // Deploy escrow contract
        let escrow_address = Address::generate(&env);
        env.register_at(&escrow_address, EscrowContract, ());
        let escrow = EscrowContractClient::new(&env, &escrow_address);

        // Set maturity date to 30 days from now
        let maturity_date = env.ledger().timestamp() + (30 * 24 * 60 * 60);

        // Initialize escrow contract (this will deploy SY, PT, YT, and Adapter)
        escrow.__constructor(
            &admin,
            &blend_pool,
            &token_address,
            &sy_wasm_hash,
            &pt_wasm_hash,
            &yt_wasm_hash,
            &adapter_wasm_hash,
            &maturity_date,
        );

        // Get deployed contract addresses
        let adapter_address = escrow.get_adapter();
        let sy_token_address = escrow.get_sy_token();
        let pt_token_address = escrow.get_pt_token();
        let yt_token_address = escrow.get_yt_token();

        TestFixture {
            env,
            admin,
            user,
            escrow,
            escrow_address,
            token: token_client,
            token_address,
            blend_pool,
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
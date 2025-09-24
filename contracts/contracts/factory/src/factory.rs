use soroban_sdk::{contract, contractimpl, contractclient, Address, Env, BytesN, Bytes, Vec};
use crate::storage;

#[contract]
pub struct FactoryContract;

#[contractclient(name = "EscrowClient")]
pub trait EscrowInterface {
    fn __constructor(
        env: Env,
        admin: Address,
        token_address: Address,
        blend_pool_address: Address,
        maturity: u64,
        coupon_amount: i128,
        principal_amount: i128
    );
}

#[contractimpl]
impl FactoryContract {
    /// Create a new escrow contract instance
    pub fn create_escrow(
        env: Env,
        admin: Address,
        token_address: Address,
        blend_pool_address: Address,
        maturity: u64,
        coupon_amount: i128,
        principal_amount: i128,
    ) -> Address {
        // Generate random salt using current ledger sequence and timestamp
        let ledger_seq = env.ledger().sequence();
        let timestamp = env.ledger().timestamp();
        let mut salt_data = Bytes::new(&env);
        salt_data.extend_from_slice(&ledger_seq.to_be_bytes());
        salt_data.extend_from_slice(&timestamp.to_be_bytes());
        let salt = env.crypto().keccak256(&salt_data);

        // Make this better
        let escrow_wasm_hash = BytesN::from_array(&env, &[
            0xc9, 0x1b, 0x69, 0x0e, 0xeb, 0x90, 0x72, 0x51,
            0xd0, 0x37, 0x57, 0x22, 0x51, 0x94, 0x66, 0xc8,
            0x79, 0xca, 0x43, 0xf7, 0x8c, 0xb7, 0x17, 0xb4,
            0xb5, 0xa3, 0x57, 0xd8, 0xfb, 0x75, 0xa0, 0x87
        ]);

        // Deploy the escrow contract
        let escrow_address = env.deployer().with_current_contract(salt)
            .deploy_v2(
                escrow_wasm_hash,
                (
                    admin.clone(),
                    token_address.clone(),
                    blend_pool_address.clone(),
                    maturity,
                    coupon_amount,
                    principal_amount,
                )
            );

        // Add escrow to storage
        storage::add_escrow_to_storage(&env, escrow_address.clone());

        // Emit creation event
        env.events().publish(
            ("escrow_created", admin.clone()),
            escrow_address.clone()
        );

        escrow_address
    }

    /// Get the total number of escrow contracts created
    pub fn get_escrow_count(env: Env) -> u32 {
        storage::get_escrow_count(&env)
    }

    /// Get all escrow contract addresses
    pub fn get_all_escrows(env: Env) -> Vec<Address> {
        storage::get_all_escrows(&env)
    }

    /// Get escrow address by index
    pub fn get_escrow_by_index(env: Env, index: u32) -> Option<Address> {
        storage::get_escrow_by_index(&env, index)
    }
}
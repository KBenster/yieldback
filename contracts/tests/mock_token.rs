use soroban_sdk::testutils::Address as _;
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub struct TokenMetadata {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
}

/// Mock token contract for testing
/// Provides a simple token implementation without all the complexity of SEP-41
#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    /// Constructor - automatically called when the contract is deployed
    pub fn __constructor(env: Env, admin: Address, name: String, symbol: String, decimals: u32) {
        let metadata = TokenMetadata {
            name,
            symbol,
            decimals,
        };
        env.storage().instance().set(&"admin", &admin);
        env.storage().instance().set(&"metadata", &metadata);
        env.storage().instance().set(&"total_supply", &0i128);
    }

    /// Mint new tokens to an address
    /// In a real token this would require auth, but for testing we allow anyone to mint
    pub fn mint(env: Env, to: Address, amount: i128) {
        let balance = Self::balance(env.clone(), to.clone());
        env.storage().persistent().set(&to, &(balance + amount));

        let total_supply: i128 = env.storage().instance().get(&"total_supply").unwrap_or(0);
        env.storage().instance().set(&"total_supply", &(total_supply + amount));
    }

    /// Burn tokens from an address
    pub fn burn(env: Env, from: Address, amount: i128) {
        let balance = Self::balance(env.clone(), from.clone());
        if balance < amount {
            panic!("Insufficient balance");
        }

        env.storage().persistent().set(&from, &(balance - amount));

        let total_supply: i128 = env.storage().instance().get(&"total_supply").unwrap_or(0);
        env.storage().instance().set(&"total_supply", &(total_supply - amount));
    }

    /// Transfer tokens from one address to another
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());
        if from_balance < amount {
            panic!("Insufficient balance");
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage().persistent().set(&from, &(from_balance - amount));
        env.storage().persistent().set(&to, &(to_balance + amount));
    }

    /// Get the balance of an address
    pub fn balance(env: Env, address: Address) -> i128 {
        env.storage().persistent().get(&address).unwrap_or(0)
    }

    /// Get the total supply of tokens
    pub fn total_supply(env: Env) -> i128 {
        env.storage().instance().get(&"total_supply").unwrap_or(0)
    }

    /// Approve a spender to use tokens on behalf of the owner
    pub fn approve(env: Env, from: Address, spender: Address, amount: i128, _expiration_ledger: u32) {
        from.require_auth();

        let key = (from, spender);
        env.storage().persistent().set(&key, &amount);
    }

    /// Get the allowance for a spender
    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        let key = (from, spender);
        env.storage().persistent().get(&key).unwrap_or(0)
    }

    /// Get token name
    pub fn name(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.name
    }

    /// Get token symbol
    pub fn symbol(env: Env) -> String {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.symbol
    }

    /// Get token decimals
    pub fn decimals(env: Env) -> u32 {
        let metadata: TokenMetadata = env.storage().instance().get(&"metadata").unwrap();
        metadata.decimals
    }
}

/// Helper function to create a mock token for testing
/// This deploys the token and the constructor is automatically called
pub fn create_mock_token<'a>(env: &Env, admin: &Address) -> (Address, MockTokenClient<'a>) {
    let contract_id = Address::generate(env);

    // Register the contract with constructor arguments
    // The __constructor will be called automatically with these arguments
    env.register_at(
        &contract_id,
        MockToken,
        (
            admin,
            String::from_str(env, "Test Token"),
            String::from_str(env, "TEST"),
            7u32,
        ),
    );

    let client = MockTokenClient::new(env, &contract_id);

    (contract_id, client)
}
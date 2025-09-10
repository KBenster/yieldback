use soroban_sdk::{
    contract, contractclient, contractimpl, token, panic_with_error,
    Address, Env, BytesN, Vec, Symbol, String, IntoVal, vec,
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}
};
use blend_contract_sdk::pool;
use crate::constants::*;
use crate::errors::*;

#[contract]
pub struct FixedIncomeContract;

// Define the coupon token client interface
#[contractclient(name = "CouponTokenClient")]
pub trait CouponTokenInterface {
    fn __constructor(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
        escrow_contract: Address,
        maturity_date: u64,
    );
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
    fn balance(env: Env, id: Address) -> i128;
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);
    fn decimals(env: Env) -> u32;
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn total_supply(env: Env) -> i128;
    fn mint(env: Env, to: Address, amount: i128);
    fn set_admin(env: Env, new_admin: Address);
    fn admin(env: Env) -> Address;
    fn authorized(env: Env, id: Address) -> bool;
    fn set_authorized(env: Env, id: Address, authorize: bool);
    fn get_escrow_contract(env: Env) -> Address;
    fn get_maturity_date(env: Env) -> u64;
}

// Define the principal token client interface
#[contractclient(name = "PrincipalTokenClient")]
pub trait PrincipalTokenInterface {
    fn __constructor(
        env: Env,
        admin: Address,
        decimal: u32,
        name: String,
        symbol: String,
        escrow_contract: Address,
        maturity_date: u64,
    );
    fn allowance(env: Env, from: Address, spender: Address) -> i128;
    fn approve(env: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32);
    fn balance(env: Env, id: Address) -> i128;
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);
    fn burn(env: Env, from: Address, amount: i128);
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);
    fn decimals(env: Env) -> u32;
    fn name(env: Env) -> String;
    fn symbol(env: Env) -> String;
    fn total_supply(env: Env) -> i128;
    fn mint(env: Env, to: Address, amount: i128);
    fn set_admin(env: Env, new_admin: Address);
    fn admin(env: Env) -> Address;
    fn authorized(env: Env, id: Address) -> bool;
    fn set_authorized(env: Env, id: Address, authorize: bool);
    fn get_escrow_contract(env: Env) -> Address;
    fn get_maturity_date(env: Env) -> u64;
}

// Define the trait (interface) for the contract
#[contractclient(name = "FixedIncomeClient")]
pub trait FixedIncome {

    /// Deposit coupon into the contract's token balance
    fn deposit_coupon(env: Env, from: Address);

    /// Deposit principal into the contract's token balance
    fn deposit_principal(env: Env, from: Address);

    /// Lend token balance to Blend
    fn lend_to_blend(env:Env) -> i128;

    /// Withdraw token balance from Blend
    fn withdraw_from_blend(env: Env) -> i128;

    /// Get total contract balance
    fn get_contract_balance(env: Env) -> i128;

    /// Get contract admin
    fn get_admin(env: Env) -> Address;

    /// Get token address
    fn get_token(env: Env) -> Address;

    /// Get coupon token address
    fn get_coupon_token(env: Env) -> Address;

    /// Get principal token address
    fn get_principal_token(env: Env) -> Address;

    /// Get maturity date
    fn get_maturity(env: Env) -> u64;

    /// Get coupon amount
    fn get_coupon_amount(env: Env) -> i128;

    /// Get principal amount
    fn get_principal_amount(env: Env) -> i128;

    fn get_blend_positions(env: Env) -> pool::Positions;

    fn withdraw_amount_from_blend(env: Env, amount: i128) -> i128;

    fn redeem_principal(env: Env, from: Address, principal_tokens_to_burn: i128);
    fn redeem_coupon(env: Env, from: Address, coupon_tokens_to_burn: i128);

}

#[contractimpl]
impl FixedIncomeContract {
    /// Initialize the contract with admin and token address
    pub fn __constructor(
        env: Env,
        admin: Address,
        token_address: Address,
        blend_pool_address: Address,
        maturity: u64,
        coupon_amount: i128,
        principal_amount: i128
    ) {
        // Ensure contract is not already initialized
        if env.storage().instance().has(&ADMIN) {
            panic_with_error!(&env, &EscrowError::AlreadyInitialized);
        }

        // Validate fixed income parameters
        if coupon_amount <= 0 {
            panic_with_error!(&env, &EscrowError::InvalidCouponAmount);
        }
        if principal_amount <= 0 {
            panic_with_error!(&env, &EscrowError::InvalidPrincipalAmount);
        }
        if maturity <= env.ledger().timestamp() {
            panic_with_error!(&env, &EscrowError::MaturityInPast);
        }

        // Deploy coupon token contract
        let coupon_token_wasm_hash = BytesN::from_array(&env, &COUPON_TOKEN_WASM_HASH);
        let coupon_salt = BytesN::from_array(&env, &COUPON_TOKEN_SALT);
        let coupon_token_address = env.deployer().with_current_contract(coupon_salt)
            .deploy_v2(
                coupon_token_wasm_hash,
                (
                    env.current_contract_address(), // admin (escrow contract)
                    7u32, // decimals
                    String::from_str(&env, "Escrow Coupon Token"),
                    String::from_str(&env, "ECT"),
                    env.current_contract_address(), // escrow contract address
                    maturity,
                )
            );

        // Deploy principal token contract
        let principal_token_wasm_hash = BytesN::from_array(&env, &PRINCIPAL_TOKEN_WASM_HASH);
        let principal_salt = BytesN::from_array(&env, &PRINCIPAL_TOKEN_SALT);
        let principal_token_address = env.deployer().with_current_contract(principal_salt)
            .deploy_v2(
                principal_token_wasm_hash,
                (
                    env.current_contract_address(), // admin (escrow contract)
                    7u32, // decimals
                    String::from_str(&env, "Escrow Principal Token"),
                    String::from_str(&env, "EPT"),
                    env.current_contract_address(), // escrow contract address
                    maturity,
                )
            );
        // Set admin and token address
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&TOKEN, &token_address);
        env.storage().instance().set(&BLEND_POOL, &blend_pool_address);
        env.storage().instance().set(&COUPON_TOKEN, &coupon_token_address);
        env.storage().instance().set(&PRINCIPAL_TOKEN, &principal_token_address);
        env.storage().instance().set(&MATURITY, &maturity);
        env.storage().instance().set(&COUPON_AMOUNT, &coupon_amount);
        env.storage().instance().set(&PRINCIPAL_AMOUNT, &principal_amount);

        // Emit event for visibility
        env.events().publish((Symbol::new(&env, "constructor_called"), admin), env.ledger().sequence());


    }
}

// Trait implementation for the main contract functions
#[contractimpl]
impl FixedIncome for FixedIncomeContract {
    fn deposit_coupon(env: Env, from: Address) {
        // Authenticate the user
        from.require_auth();

        // Check if coupon has already been deposited
        if env.storage().instance().has(&COUPON_DEPOSITED) {
            panic_with_error!(&env, &EscrowError::CouponAlreadyDeposited);
        }

        // Get the coupon amount from storage
        let coupon_amount: i128 = env.storage().instance().get(&COUPON_AMOUNT).unwrap();

        // Get token contract
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Get coupon token contract
        let coupon_token_address: Address = env.storage().instance().get(&COUPON_TOKEN).unwrap();
        let coupon_token = CouponTokenClient::new(&env, &coupon_token_address);

        // Transfer tokens from user to contract
        token.transfer(&from, &env.current_contract_address(), &coupon_amount);

        // Mark coupon as deposited
        env.storage().instance().set(&COUPON_DEPOSITED, &true);

        // Authorize the escrow contract to mint coupon tokens
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: coupon_token_address.clone(),
                    fn_name: Symbol::new(&env, "mint"),
                    args: (
                        from.clone(),
                        coupon_amount, // Mint 1:1 ratio, adjust as needed
                    ).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Mint coupon tokens to the depositor (1:1 ratio with coupon amount)
        coupon_token.mint(&from, &coupon_amount);

        // Emit event
        env.events().publish((Symbol::new(&env, "deposit"), from.clone()), coupon_amount);
        env.events().publish((Symbol::new(&env, "shares_minted"), from), coupon_amount);
    }

    fn deposit_principal(env: Env, from: Address) {
        // Authenticate the user
        from.require_auth();

        // Check if principal has already been deposited
        if env.storage().instance().has(&PRINCIPAL_DEPOSITED) {
            panic_with_error!(&env, &EscrowError::PrincipalAlreadyDeposited);
        }

        // Get the principal amount from storage
        let principal_amount: i128 = env.storage().instance().get(&PRINCIPAL_AMOUNT).unwrap();

        // Get token contract
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Get principal token contract
        let principal_token_address: Address = env.storage().instance().get(&PRINCIPAL_TOKEN).unwrap();
        let principal_token = PrincipalTokenClient::new(&env, &principal_token_address);

        // Transfer tokens from user to contract
        token.transfer(&from, &env.current_contract_address(), &principal_amount);

        // Mark principal as deposited
        env.storage().instance().set(&PRINCIPAL_DEPOSITED, &true);

        // Authorize the escrow contract to mint principal tokens
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: principal_token_address.clone(),
                    fn_name: Symbol::new(&env, "mint"),
                    args: (
                        from.clone(),
                        principal_amount,
                    ).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Mint principal tokens to the depositor (1:1 ratio with principal amount)
        principal_token.mint(&from, &principal_amount);

        // Emit event
        env.events().publish((Symbol::new(&env, "deposit_principal"), from.clone()), principal_amount);
        env.events().publish((Symbol::new(&env, "principal_shares_minted"), from), principal_amount);
    }

    fn lend_to_blend(env: Env) -> i128 {
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

        let token = token::Client::new(&env, &token_address);
        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        // Get current contract balance
        let contract_balance = token.balance(&env.current_contract_address());

        if contract_balance <= 0 {
            panic_with_error!(&env, &EscrowError::NoTokensToLend);
        }

        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: token_address.clone(),
                    fn_name: Symbol::new(&env, "transfer"),
                    args: (
                        env.current_contract_address(),
                        blend_pool_address.clone(),
                        contract_balance,
                    ).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        let supply_request = pool::Request {
            request_type: BLEND_SUPPLY_REQUEST,
            address: token_address.clone(),
            amount: contract_balance,
        };

        let requests = Vec::from_array(&env, [supply_request]);

        blend_pool.submit(
            &env.current_contract_address(), // from (this contract)
            &env.current_contract_address(), // spender (this contract)
            &env.current_contract_address(), // to (bTokens recipient - this contract)
            &requests
        );

        // Emit lending event
        env.events().publish(
            (Symbol::new(&env, "lent_to_blend"), env.current_contract_address()),
            contract_balance
        );

        contract_balance
    }

    fn withdraw_from_blend(env: Env) -> i128 {

        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        // Get current positions to withdraw entire balance
        let positions = blend_pool.get_positions(&env.current_contract_address());
        let total_supply = positions.supply.get(0).unwrap_or(0); // Assuming reserve_id 0, adjust as needed

        if total_supply <= 0 {
            panic_with_error!(&env, &EscrowError::NoPositionInBlend);
        }

        // Create withdrawal request for entire position
        let withdraw_request = pool::Request {
            request_type: BLEND_WITHDRAW_REQUEST,
            address: token_address.clone(),
            amount: total_supply,
        };

        let requests = Vec::from_array(&env, [withdraw_request]);

        // Submit withdrawal request
        blend_pool.submit(
            &env.current_contract_address(), // from (this contract)
            &env.current_contract_address(), // spender (this contract)
            &env.current_contract_address(), // to (withdrawal recipient - this contract)
            &requests
        );

        // Emit withdrawal event
        env.events().publish(
            (Symbol::new(&env, "withdrawn_from_blend"), env.current_contract_address()),
            total_supply
        );

        total_supply
    }

    fn withdraw_amount_from_blend(env: Env, amount: i128) -> i128 {
        if amount <= 0 {
            panic_with_error!(&env, &EscrowError::AmountMustBePositive);
        }

        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();

        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        // Get current positions to check available balance
        let positions = blend_pool.get_positions(&env.current_contract_address());
        let total_supply = positions.supply.get(DEFAULT_RESERVE_ID).unwrap_or(0);

        if total_supply <= 0 {
            panic_with_error!(&env, &EscrowError::NoPositionInBlend);
        }

        if amount > total_supply {
            panic_with_error!(&env, &EscrowError::InsufficientFundsInBlend);
        }

        // Create withdrawal request for specified amount
        let withdraw_request = pool::Request {
            request_type: 1, // Withdraw request type
            address: token_address.clone(),
            amount: amount,
        };

        let requests = Vec::from_array(&env, [withdraw_request]);

        // Submit withdrawal request
        blend_pool.submit(
            &env.current_contract_address(), // from (this contract)
            &env.current_contract_address(), // spender (this contract)
            &env.current_contract_address(), // to (withdrawal recipient - this contract)
            &requests
        );

        // Emit withdrawal event
        env.events().publish(
            (Symbol::new(&env, "withdrawn_amount_from_blend"), env.current_contract_address()),
            amount
        );

        amount
    }

    fn redeem_principal(env: Env, from: Address, principal_tokens_to_burn: i128) {
        // Authenticate the user
        from.require_auth();

        if principal_tokens_to_burn <= 0 {
            panic_with_error!(&env, &EscrowError::AmountMustBePositive);
        }

        // Get principal token contract
        let principal_token_address: Address = env.storage().instance().get(&PRINCIPAL_TOKEN).unwrap();
        let principal_token = PrincipalTokenClient::new(&env, &principal_token_address);

        // Check if user has sufficient principal tokens
        let user_principal_balance = principal_token.balance(&from);
        if user_principal_balance < principal_tokens_to_burn {
            panic_with_error!(&env, &EscrowError::InsufficientPrincipalBalance);
        }

        // Get the principal and coupon amounts from storage
        let principal_amount: i128 = env.storage().instance().get(&PRINCIPAL_AMOUNT).unwrap();
        let coupon_amount: i128 = env.storage().instance().get(&COUPON_AMOUNT).unwrap();

        // Calculate the total redemption amount (principal + coupon)
        let total_redemption_amount = principal_amount + coupon_amount;

        // Calculate the proportion of tokens being redeemed
        let principal_supply = principal_token.total_supply();
        let redemption_ratio = if principal_supply > 0 {
            (principal_tokens_to_burn as f64) / (principal_supply as f64)
        } else {
            panic_with_error!(&env, &EscrowError::NoPrincipalTokensInCirculation);
        };

        // Calculate the actual amount to withdraw (proportional to tokens being burned)
        let amount_to_withdraw = ((total_redemption_amount as f64) * redemption_ratio) as i128;

        // Get token contract for underlying asset
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);

        // Check current contract balance of underlying tokens
        let contract_balance_before = token.balance(&env.current_contract_address());

        // Withdraw the required amount from Blend
        let withdrawn_amount = Self::withdraw_amount_from_blend(env.clone(), amount_to_withdraw);

        // Verify we actually received the tokens
        let contract_balance_after = token.balance(&env.current_contract_address());
        let received_amount = contract_balance_after - contract_balance_before;

        if received_amount < amount_to_withdraw {
            panic_with_error!(&env, &EscrowError::InsufficientFundsAfterWithdrawal);
        }

        // Authorize the escrow contract to burn principal tokens
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: principal_token_address.clone(),
                    fn_name: Symbol::new(&env, "burn"),
                    args: (
                        from.clone(),
                        principal_tokens_to_burn,
                    ).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Burn the principal tokens
        principal_token.burn(&from, &principal_tokens_to_burn);

        // 🔹 CHANGED: Transfer the total redemption amount (principal + coupon) to the user
        token.transfer(&env.current_contract_address(), &from, &amount_to_withdraw);

        // Emit events
        env.events().publish(
            (Symbol::new(&env, "principal_redeemed"), from.clone()),
            amount_to_withdraw // 🔹 CHANGED: This now includes both principal and coupon
        );
        env.events().publish(
            (Symbol::new(&env, "principal_tokens_burned"), from.clone()),
            principal_tokens_to_burn
        );
        // 🔹 NEW: Additional event for transparency
        env.events().publish(
            (Symbol::new(&env, "total_redemption_amount"), from),
            amount_to_withdraw
        );
    }

    fn redeem_coupon(env: Env, from: Address, coupon_tokens_to_burn: i128) {
        // Authenticate the user
        from.require_auth();

        if coupon_tokens_to_burn <= 0 {
            panic_with_error!(&env, &EscrowError::AmountMustBePositive);
        }

        // Get coupon token contract
        let coupon_token_address: Address = env.storage().instance().get(&COUPON_TOKEN).unwrap();
        let coupon_token = CouponTokenClient::new(&env, &coupon_token_address);

        // Check if user has sufficient coupon tokens
        let user_coupon_balance = coupon_token.balance(&from);
        if user_coupon_balance < coupon_tokens_to_burn {
            panic_with_error!(&env, &EscrowError::InsufficientCouponBalance);
        }

        // Get the principal and coupon amounts from storage
        let principal_amount: i128 = env.storage().instance().get(&PRINCIPAL_AMOUNT).unwrap();
        let coupon_amount: i128 = env.storage().instance().get(&COUPON_AMOUNT).unwrap();
        let total_reserved_amount = principal_amount + coupon_amount;

        // Get current total balance in Blend pool
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();
        let blend_pool = pool::Client::new(&env, &blend_pool_address);
        let positions = blend_pool.get_positions(&env.current_contract_address());
        let total_balance_in_blend = positions.supply.get(0).unwrap_or(0);

        // Get token contract for direct balance check
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);
        let contract_direct_balance = token.balance(&env.current_contract_address());

        // Calculate total available funds (Blend + direct balance)
        let total_available_funds = total_balance_in_blend + contract_direct_balance;

        // Calculate excess yield (what's available above the reserved amount)
        if total_available_funds <= total_reserved_amount {
            panic_with_error!(&env, &EscrowError::NoExcessYieldAvailable);
        }

        let total_excess_yield = total_available_funds - total_reserved_amount;

        // Calculate proportional share for the coupon holder
        let coupon_supply = coupon_token.total_supply();
        if coupon_supply == 0 {
            panic_with_error!(&env, &EscrowError::NoCouponTokensInCirculation);
        }

        let redemption_ratio = (coupon_tokens_to_burn as f64) / (coupon_supply as f64);
        let amount_to_withdraw = ((total_excess_yield as f64) * redemption_ratio) as i128;

        if amount_to_withdraw <= 0 {
            panic_with_error!(&env, &EscrowError::NoYieldAvailableForRedemption);
        }

        // Withdraw the calculated amount from Blend first if needed
        let mut withdrawn_from_blend = 0i128;
        if amount_to_withdraw > contract_direct_balance {
            let needed_from_blend = amount_to_withdraw - contract_direct_balance;
            withdrawn_from_blend = Self::withdraw_amount_from_blend(env.clone(), needed_from_blend);
        }

        // Verify we have sufficient funds after withdrawal
        let final_contract_balance = token.balance(&env.current_contract_address());
        if final_contract_balance < amount_to_withdraw {
            panic_with_error!(&env, &EscrowError::InsufficientFundsAfterWithdrawal);
        }

        // Authorize the escrow contract to burn coupon tokens
        env.authorize_as_current_contract(vec![
            &env,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: coupon_token_address.clone(),
                    fn_name: Symbol::new(&env, "burn"),
                    args: (
                        from.clone(),
                        coupon_tokens_to_burn,
                    ).into_val(&env),
                },
                sub_invocations: vec![&env],
            }),
        ]);

        // Burn the coupon tokens
        coupon_token.burn(&from, &coupon_tokens_to_burn);

        // Transfer the excess yield to the coupon holder (sponsor)
        token.transfer(&env.current_contract_address(), &from, &amount_to_withdraw);

        // Emit events
        env.events().publish(
            (Symbol::new(&env, "coupon_redeemed"), from.clone()),
            amount_to_withdraw
        );
        env.events().publish(
            (Symbol::new(&env, "coupon_tokens_burned"), from.clone()),
            coupon_tokens_to_burn
        );
        env.events().publish(
            (Symbol::new(&env, "excess_yield_claimed"), from.clone()),
            amount_to_withdraw
        );
        env.events().publish(
            (Symbol::new(&env, "total_excess_yield_available"), Symbol::new(&env, "system")),
            total_excess_yield
        );
    }

    fn get_contract_balance(env: Env) -> i128 {
        let token_address: Address = env.storage().instance().get(&TOKEN).unwrap();
        let token = token::Client::new(&env, &token_address);

        token.balance(&env.current_contract_address())
    }

    fn get_blend_positions(env: Env) -> pool::Positions {
        let blend_pool_address: Address = env.storage().instance().get(&BLEND_POOL).unwrap();
        let blend_pool = pool::Client::new(&env, &blend_pool_address);

        blend_pool.get_positions(&env.current_contract_address())
    }

    fn get_admin(env: Env) -> Address {
        env.storage().instance().get(&ADMIN).unwrap()
    }

    fn get_token(env: Env) -> Address {
        env.storage().instance().get(&TOKEN).unwrap()
    }

    fn get_maturity(env: Env) -> u64 {
        env.storage().instance().get(&MATURITY).unwrap()
    }

    fn get_coupon_amount(env: Env) -> i128 {
        env.storage().instance().get(&COUPON_AMOUNT).unwrap()
    }

    fn get_principal_amount(env: Env) -> i128 {
        env.storage().instance().get(&PRINCIPAL_AMOUNT).unwrap()
    }

    fn get_coupon_token(env: Env) -> Address {
        env.storage().instance().get(&COUPON_TOKEN).unwrap()
    }

    fn get_principal_token(env: Env) -> Address {
        env.storage().instance().get(&PRINCIPAL_TOKEN).unwrap()
    }
}
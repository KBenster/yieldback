//TODO: THIS IS FUCKED! FIX THE ESCROW CONTRACT THEN FIX THIS LATER! THERES A LOT OF ISSUES!
import { Address, Contract } from '@stellar/stellar-sdk';

import { Spec as ContractSpec, u64, u32, i128 } from '@stellar/stellar-sdk/contract';

export type Option<T> = T | undefined;

export interface Positions {
    // Based on Blend protocol documentation and contract usage
    supply: Map<number, i128>;        // Map from reserve_id to supplied amount (bTokens)
    liabilities: Map<number, i128>;   // Map from reserve_id to borrowed amount (dTokens)
    collateral: Map<number, i128>;    // Map from reserve_id to collateral amount
}

export interface DepositCouponArgs {
    from: Address | string;
}

export interface DepositPrincipalArgs {
    from: Address | string;
}

export interface WithdrawAmountFromBlendArgs {
    amount: i128;
}

export interface RedeemPrincipalArgs {
    from: Address | string;
    principal_tokens_to_burn: i128;
}

export interface RedeemCouponArgs {
    from: Address | string;
    coupon_tokens_to_burn: i128;
}

export class EscrowContract extends Contract {
    static spec: ContractSpec = new ContractSpec([
        "AAAAAAAAADRJbml0aWFsaXplIHRoZSBjb250cmFjdCB3aXRoIGFkbWluIGFuZCB0b2tlbiBhZGRyZXNzAAAADV9fY29uc3RydWN0b3IAAAAAAAAGAAAAAAAAAAVhZG1pbgAAAAAAABMAAAAAAAAADXRva2VuX2FkZHJlc3MAAAAAAAATAAAAAAAAABJibGVuZF9wb29sX2FkZHJlc3MAAAAAABMAAAAAAAAACG1hdHVyaXR5AAAABgAAAAAAAAANY291cG9uX2Ftb3VudAAAAAAAAAsAAAAAAAAAEHByaW5jaXBhbF9hbW91bnQAAAALAAAAAA==",
        "AAAAAAAAAAAAAAAOZGVwb3NpdF9jb3Vwb24AAAAAAAEAAAAAAAAABGZyb20AAAATAAAAAA==",
        "AAAAAAAAAAAAAAARZGVwb3NpdF9wcmluY2lwYWwAAAAAAAABAAAAAAAAAARmcm9tAAAAEwAAAAA=",
        "AAAAAAAAAAAAAAANbGVuZF90b19ibGVuZAAAAAAAAAAAAAABAAAACw==",
        "AAAAAAAAAAAAAAATd2l0aGRyYXdfZnJvbV9ibGVuZAAAAAAAAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAAad2l0aGRyYXdfYW1vdW50X2Zyb21fYmxlbmQAAAAAAAEAAAAAAAAABmFtb3VudAAAAAAACwAAAAEAAAAL",
        "AAAAAAAAAAAAAAAQcmVkZWVtX3ByaW5jaXBhbAAAAAIAAAAAAAAABGZyb20AAAATAAAAAAAAABhwcmluY2lwYWxfdG9rZW5zX3RvX2J1cm4AAAALAAAAAA==",
        "AAAAAAAAAAAAAAAUZ2V0X2NvbnRyYWN0X2JhbGFuY2UAAAAAAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAATZ2V0X2JsZW5kX3Bvc2l0aW9ucwAAAAAAAAAAAQAAB9AAAAAJUG9zaXRpb25zAAAA",
        "AAAAAAAAAAAAAAAJZ2V0X2FkbWluAAAAAAAAAAAAAAEAAAAT",
        "AAAAAAAAAAAAAAAJZ2V0X3Rva2VuAAAAAAAAAAAAAAEAAAAT",
        "AAAAAAAAAAAAAAAMZ2V0X21hdHVyaXR5AAAAAAAAAAEAAAAG",
        "AAAAAAAAAAAAAAARZ2V0X2NvdXBvbl9hbW91bnQAAAAAAAAAAAAAAQAAAAs=",
        "AAAAAAAAAAAAAAAUZ2V0X3ByaW5jaXBhbF9hbW91bnQAAAAAAAAAAQAAAAs=",
        // Note: The following XDRs would need to be generated from your contract for these additional methods:
        // "get_coupon_token", "get_principal_token", "redeem_coupon"
        // For now, using placeholder XDRs - replace with actual generated XDRs
        "AAAAAAAAAAAAAAARZ2V0X2NvdXBvbl90b2tlbgAAAAAAAAAAAAEAAAAT",
        "AAAAAAAAAAAAAAAUZ2V0X3ByaW5jaXBhbF90b2tlbgAAAAAAAAAAAQAAABM=",
        "AAAAAAAAAAAAAAANcmVkZWVtX2NvdXBvbgAAAAAAAAIAAAAAAAAABGZyb20AAAATAAAAAAAAABdjb3Vwb25fdG9rZW5zX3RvX2J1cm4AAAAACwAAAAA="
    ]);

    static readonly parsers = {
        depositCoupon: (result: string): null =>
            EscrowContract.spec.funcResToNative('deposit_coupon', result) as null,

        depositPrincipal: (result: string): null =>
            EscrowContract.spec.funcResToNative('deposit_principal', result) as null,

        lendToBlend: (result: string): i128 =>
            EscrowContract.spec.funcResToNative('lend_to_blend', result) as i128,

        withdrawFromBlend: (result: string): i128 =>
            EscrowContract.spec.funcResToNative('withdraw_from_blend', result) as i128,

        withdrawAmountFromBlend: (result: string): i128 =>
            EscrowContract.spec.funcResToNative('withdraw_amount_from_blend', result) as i128,

        redeemPrincipal: (result: string): null =>
            EscrowContract.spec.funcResToNative('redeem_principal', result) as null,

        getContractBalance: (result: string): i128 =>
            EscrowContract.spec.funcResToNative('get_contract_balance', result) as i128,

        getBlendPositions: (result: string): Positions =>
            EscrowContract.spec.funcResToNative('get_blend_positions', result) as Positions,

        getAdmin: (result: string): string =>
            EscrowContract.spec.funcResToNative('get_admin', result) as string,

        getToken: (result: string): string =>
            EscrowContract.spec.funcResToNative('get_token', result) as string,

        getMaturity: (result: string): u64 =>
            EscrowContract.spec.funcResToNative('get_maturity', result) as u64,

        getCouponAmount: (result: string): i128 =>
            EscrowContract.spec.funcResToNative('get_coupon_amount', result) as i128,

        getPrincipalAmount: (result: string): i128 =>
            EscrowContract.spec.funcResToNative('get_principal_amount', result) as i128,

        getCouponToken: (result: string): string =>
            EscrowContract.spec.funcResToNative('get_coupon_token', result) as string,

        getPrincipalToken: (result: string): string =>
            EscrowContract.spec.funcResToNative('get_principal_token', result) as string,

        redeemCoupon: (result: string): null =>
            EscrowContract.spec.funcResToNative('redeem_coupon', result) as null,
    };

    // Transaction methods
    depositCoupon(args: DepositCouponArgs) {
        return this.call(
            'deposit_coupon',
            ...EscrowContract.spec.funcArgsToScVals('deposit_coupon', {
                from: args.from
            })
        ).toXDR('base64');
    }

    depositPrincipal(args: DepositPrincipalArgs) {
        return this.call(
            'deposit_principal',
            ...EscrowContract.spec.funcArgsToScVals('deposit_principal', {
                from: args.from
            })
        ).toXDR('base64');
    }

    lendToBlend() {
        return this.call(
            'lend_to_blend',
            ...EscrowContract.spec.funcArgsToScVals('lend_to_blend', {})
        ).toXDR('base64');
    }

    withdrawFromBlend() {
        return this.call(
            'withdraw_from_blend',
            ...EscrowContract.spec.funcArgsToScVals('withdraw_from_blend', {})
        ).toXDR('base64');
    }

    withdrawAmountFromBlend(args: WithdrawAmountFromBlendArgs) {
        return this.call(
            'withdraw_amount_from_blend',
            ...EscrowContract.spec.funcArgsToScVals('withdraw_amount_from_blend', {
                amount: args.amount
            })
        ).toXDR('base64');
    }

    redeemPrincipal(args: RedeemPrincipalArgs) {
        return this.call(
            'redeem_principal',
            ...EscrowContract.spec.funcArgsToScVals('redeem_principal', {
                from: args.from,
                principal_tokens_to_burn: args.principal_tokens_to_burn
            })
        ).toXDR('base64');
    }

    // View methods
    getContractBalance() {
        return this.call(
            'get_contract_balance',
            ...EscrowContract.spec.funcArgsToScVals('get_contract_balance', {})
        ).toXDR('base64');
    }

    getBlendPositions() {
        return this.call(
            'get_blend_positions',
            ...EscrowContract.spec.funcArgsToScVals('get_blend_positions', {})
        ).toXDR('base64');
    }

    getAdmin() {
        return this.call(
            'get_admin',
            ...EscrowContract.spec.funcArgsToScVals('get_admin', {})
        ).toXDR('base64');
    }

    getToken() {
        return this.call(
            'get_token',
            ...EscrowContract.spec.funcArgsToScVals('get_token', {})
        ).toXDR('base64');
    }

    getMaturity() {
        return this.call(
            'get_maturity',
            ...EscrowContract.spec.funcArgsToScVals('get_maturity', {})
        ).toXDR('base64');
    }

    getCouponAmount() {
        return this.call(
            'get_coupon_amount',
            ...EscrowContract.spec.funcArgsToScVals('get_coupon_amount', {})
        ).toXDR('base64');
    }

    getPrincipalAmount() {
        return this.call(
            'get_principal_amount',
            ...EscrowContract.spec.funcArgsToScVals('get_principal_amount', {})
        ).toXDR('base64');
    }

    getCouponToken() {
        return this.call(
            'get_coupon_token',
            ...EscrowContract.spec.funcArgsToScVals('get_coupon_token', {})
        ).toXDR('base64');
    }

    getPrincipalToken() {
        return this.call(
            'get_principal_token',
            ...EscrowContract.spec.funcArgsToScVals('get_principal_token', {})
        ).toXDR('base64');
    }

    redeemCoupon(args: RedeemCouponArgs) {
        return this.call(
            'redeem_coupon',
            ...EscrowContract.spec.funcArgsToScVals('redeem_coupon', {
                from: args.from,
                coupon_tokens_to_burn: args.coupon_tokens_to_burn
            })
        ).toXDR('base64');
    }
}
import { Address, Contract } from '@stellar/stellar-sdk';
import { Spec as ContractSpec, u64, u32, i128 } from '@stellar/stellar-sdk/contract';

export type Option<T> = T | undefined;

export interface CreateEscrowArgs {
    admin: Address | string;
    token_address: Address | string;
    blend_pool_address: Address | string;
    maturity: u64;
    coupon_amount: i128;
    principal_amount: i128;
}

export class FactoryContract extends Contract {
    static spec: ContractSpec = new ContractSpec([
        // You need to regenerate these XDR strings to match your actual deployed contract
        // Run: stellar contract bindings typescript --contract-id CBK3DV272E5J3ZR4AHDFUOW7CHUWD23AYUV3UAW6QEFNP6HFAZGIKYP4 --network testnet --output-dir ./src/lib/generated
        "AAAAAAAAACVDcmVhdGUgYSBuZXcgZXNjcm93IGNvbnRyYWN0IGluc3RhbmNlAAAAAAAADWNyZWF0ZV9lc2Nyb3cAAAAAAAAGAAAAAAAAAAVhZG1pbgAAAAAAABMAAAAAAAAADXRva2VuX2FkZHJlc3MAAAAAAAATAAAAAAAAABJibGVuZF9wb29sX2FkZHJlc3MAAAAAABMAAAAAAAAACG1hdHVyaXR5AAAABgAAAAAAAAANY291cG9uX2Ftb3VudAAAAAAAAAsAAAAAAAAAEHByaW5jaXBhbF9hbW91bnQAAAALAAAAAQAAABM=",
        "AAAAAAAAADBHZXQgdGhlIHRvdGFsIG51bWJlciBvZiBlc2Nyb3cgY29udHJhY3RzIGNyZWF0ZWQAAAAQZ2V0X2VzY3Jvd19jb3VudAAAAAAAAAABAAAABA==",
        "AAAAAAAAACFHZXQgYWxsIGVzY3JvdyBjb250cmFjdCBhZGRyZXNzZXMAAAAAAAAPZ2V0X2FsbF9lc2Nyb3dzAAAAAAAAAAABAAAD6gAAABM=",
        "AAAAAAAAABtHZXQgZXNjcm93IGFkZHJlc3MgYnkgaW5kZXgAAAAAE2dldF9lc2Nyb3dfYnlfaW5kZXgAAAAAAQAAAAAAAAAFaW5kZXgAAAAAAAAEAAAAAQAAA+gAAAAT"
    ]);

    static readonly parsers = {
        // Updated to match actual return types
        createEscrow: (result: string): string =>
            FactoryContract.spec.funcResToNative('create_escrow', result) as string,

        getEscrowCount: (result: string): u32 =>
            FactoryContract.spec.funcResToNative('get_escrow_count', result) as u32,

        getAllEscrows: (result: string): string[] =>
            FactoryContract.spec.funcResToNative('get_all_escrows', result) as string[],

        getEscrowByIndex: (result: string): Option<string> =>
            FactoryContract.spec.funcResToNative('get_escrow_by_index', result) as Option<string>,
    };

    createEscrow(contractArgs: CreateEscrowArgs) {
        return this.call(
            'create_escrow',
            ...FactoryContract.spec.funcArgsToScVals('create_escrow', {
                admin: contractArgs.admin,
                token_address: contractArgs.token_address,
                blend_pool_address: contractArgs.blend_pool_address,
                maturity: contractArgs.maturity,
                coupon_amount: contractArgs.coupon_amount,
                principal_amount: contractArgs.principal_amount
            })
        ).toXDR('base64');
    }

    getEscrowCount() {
        return this.call(
            'get_escrow_count',
            ...FactoryContract.spec.funcArgsToScVals('get_escrow_count', {})
        ).toXDR('base64');
    }

    getAllEscrows() {
        return this.call(
            'get_all_escrows',
            ...FactoryContract.spec.funcArgsToScVals('get_all_escrows', {})
        ).toXDR('base64');
    }

    getEscrowByIndex(index: u32) {
        return this.call(
            'get_escrow_by_index',
            ...FactoryContract.spec.funcArgsToScVals('get_escrow_by_index', { index })
        ).toXDR('base64');
    }
}
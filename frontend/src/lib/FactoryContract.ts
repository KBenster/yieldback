

import { Buffer } from "buffer";
import { Address } from '@stellar/stellar-sdk';
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Result,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i32,
  u64,
  i64,
  u128,
  i128,
  u256,
  i256,
  Option,
  Typepoint,
  Duration,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'


export const networks = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    contractId: "CBK3DV272E5J3ZR4AHDFUOW7CHUWD23AYUV3UAW6QEFNP6HFAZGIKYP4",
  }
} as const


export interface ClientInterface {
  /**
   * Construct and simulate a create_escrow transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Create a new escrow contract instance
   */
  create_escrow: ({admin, token_address, blend_pool_address, maturity, coupon_amount, principal_amount}: {admin: string, token_address: string, blend_pool_address: string, maturity: u64, coupon_amount: i128, principal_amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a get_escrow_count transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get the total number of escrow contracts created
   */
  get_escrow_count: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<u32>>

  /**
   * Construct and simulate a get_all_escrows transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get all escrow contract addresses
   */
  get_all_escrows: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<string>>>

  /**
   * Construct and simulate a get_escrow_by_index transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get escrow address by index
   */
  get_escrow_by_index: ({index}: {index: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Option<string>>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
    /** Options for initializing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy(null, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAACVDcmVhdGUgYSBuZXcgZXNjcm93IGNvbnRyYWN0IGluc3RhbmNlAAAAAAAADWNyZWF0ZV9lc2Nyb3cAAAAAAAAGAAAAAAAAAAVhZG1pbgAAAAAAABMAAAAAAAAADXRva2VuX2FkZHJlc3MAAAAAAAATAAAAAAAAABJibGVuZF9wb29sX2FkZHJlc3MAAAAAABMAAAAAAAAACG1hdHVyaXR5AAAABgAAAAAAAAANY291cG9uX2Ftb3VudAAAAAAAAAsAAAAAAAAAEHByaW5jaXBhbF9hbW91bnQAAAALAAAAAQAAABM=",
        "AAAAAAAAADBHZXQgdGhlIHRvdGFsIG51bWJlciBvZiBlc2Nyb3cgY29udHJhY3RzIGNyZWF0ZWQAAAAQZ2V0X2VzY3Jvd19jb3VudAAAAAAAAAABAAAABA==",
        "AAAAAAAAACFHZXQgYWxsIGVzY3JvdyBjb250cmFjdCBhZGRyZXNzZXMAAAAAAAAPZ2V0X2FsbF9lc2Nyb3dzAAAAAAAAAAABAAAD6gAAABM=",
        "AAAAAAAAABtHZXQgZXNjcm93IGFkZHJlc3MgYnkgaW5kZXgAAAAAE2dldF9lc2Nyb3dfYnlfaW5kZXgAAAAAAQAAAAAAAAAFaW5kZXgAAAAAAAAEAAAAAQAAA+gAAAAT" ]),
      options
    )
  }
  public readonly fromJSON = {
    create_escrow: this.txFromJSON<string>,
        get_escrow_count: this.txFromJSON<u32>,
        get_all_escrows: this.txFromJSON<Array<string>>,
        get_escrow_by_index: this.txFromJSON<Option<string>>
  }
}
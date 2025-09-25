"use client"
import React, { createContext, useContext, useEffect, useState, useRef } from "react";
import {
    FreighterModule,
    ISupportedWallet,
    StellarWalletsKit,
    WalletNetwork,
    xBullModule,
    LobstrModule,
    AlbedoModule,
    HotWalletModule,
    HanaModule,
    XBULL_ID
} from "@creit.tech/stellar-wallets-kit";
import {
    rpc,
    Account,
    TransactionBuilder,
    xdr,
    Transaction,
    TimeoutInfinite,
} from "@stellar/stellar-sdk";
import { TxStatus } from "@/lib/types";
import { config } from "@/lib/config";
import { txToast } from "@/lib/toast";
import { Id } from "react-toastify";
import { FactoryContract, CreateEscrowArgs } from "@/lib/FactoryContract";
import { LedgerModule } from "@creit.tech/stellar-wallets-kit/modules/ledger.module";

const walletKit: StellarWalletsKit = new StellarWalletsKit({
    network: config.network.passphrase as WalletNetwork,
    selectedWalletId: XBULL_ID,
    modules: [
        new xBullModule(),
        new FreighterModule(),
        new LobstrModule(),
        new AlbedoModule(),
        new HanaModule(),
        new LedgerModule(),
        new HotWalletModule(),
    ],
});

// Wallet context interface
interface WalletContextValue {
    connected: boolean;
    walletAddress: string;
    txStatus: TxStatus;
    txHash: string | undefined;
    txError: string | undefined;
    isLoading: boolean;
    walletId: string | undefined;

    connect: (handleSuccess?: (success: boolean) => void) => Promise<void>;
    disconnect: () => void;
    clearTxStatus: () => void;

    // Contract Functions
    createEscrow: (args: CreateEscrowArgs) => Promise<string | null>;
}

const WalletContext = createContext<WalletContextValue | undefined>(undefined);

export const WalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    // State variables
    const [connected, setConnected] = useState<boolean>(false);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [txStatus, setTxStatus] = useState<TxStatus>(TxStatus.NONE);
    const [txHash, setTxHash] = useState<string | undefined>(undefined);
    const [txError, setTxError] = useState<string | undefined>(undefined);
    const [walletAddress, setWalletAddress] = useState<string>('');
    const [walletId, setWalletId] = useState<string | undefined>(undefined);

    // Toast reference to track active transaction toast
    const activeToastId = useRef<Id | null>(null);

    // Initialize RPC server and contract using orbit-swap style
    const rpcServer = new rpc.Server(config.network.sorobanRpcUrl, { allowHttp: true });
    const [sorobanRpc] = useState<rpc.Server>(rpcServer);

    // Initialize YieldBack factory contract (orbit-swap style)
    const factoryContract = new FactoryContract(config.contracts.factory);
    const [contract] = useState<FactoryContract>(factoryContract);

    /**
     * Gets the public key from the connected wallet
     */
    const handleGetWalletAddress = async (): Promise<boolean> => {
        try {
            const { address } = await walletKit.getAddress();
            if (!address) {
                console.error('Unable to load wallet key');
                return false;
            }

            console.log('Wallet address:', address);
            setWalletAddress(address);
            setConnected(true);

            return true;
        } catch (error) {
            console.error('Unable to load wallet information:', error);
            return false;
        }
    };

    // Track transaction status changes and update toast
    useEffect(() => {
        if (txStatus !== TxStatus.NONE && activeToastId.current) {
            txToast.update(activeToastId.current, txStatus, txHash, txError);

            // Clear the toast reference if transaction is done
            if (txStatus === TxStatus.SUCCESS || txStatus === TxStatus.FAIL) {
                // Don't set to null immediately to allow for toast updates
                setTimeout(() => {
                    activeToastId.current = null;
                }, 500);
            }
        }
    }, [txStatus, txHash, txError]);

    /**
     * Opens wallet selector modal and connects selected wallet
     */
    const connect = async (handleSuccess?: (success: boolean) => void) => {
        try {
            setIsLoading(true);
            await walletKit.openModal({
                onWalletSelected: async (option: ISupportedWallet) => {
                    walletKit.setWallet(option.id);
                    setWalletId(option.id);

                    const result = await handleGetWalletAddress();
                    if (handleSuccess) handleSuccess(result);
                },
            });
            setIsLoading(false);
        } catch (error) {
            setIsLoading(false);
            if (handleSuccess) handleSuccess(false);
            console.error('Unable to connect wallet:', error);
        }
    };

    /**
     * Disconnects the current wallet
     */
    const disconnect = () => {
        setWalletAddress('');
        setConnected(false);
        setWalletId(undefined);
        clearTxStatus();
    };

    /**
     * Clears transaction status
     */
    const clearTxStatus = () => {
        setTxStatus(TxStatus.NONE);
        setTxHash(undefined);
        setTxError(undefined);

        // Dismiss active toast if exists
        if (activeToastId.current) {
            txToast.dismiss(activeToastId.current);
            activeToastId.current = null;
        }
    };

    /**
     * Sign a transaction XDR with the connected wallet
     * @param xdr - The XDR to sign
     * @returns - The signed XDR
     */
    async function sign(xdrString: string): Promise<string> {
        if (connected) {
            setTxStatus(TxStatus.SIGNING);

            try {
                const { signedTxXdr } = await walletKit.signTransaction(xdrString, {
                    networkPassphrase: config.network.passphrase
                });
                setTxStatus(TxStatus.SUBMITTING);
                return signedTxXdr;
            } catch (e: unknown) {
                if (e === 'User declined access') {
                    setTxError('Transaction rejected by wallet.');
                    txToast.error('Transaction rejected by wallet.');
                } else if (typeof e === 'string') {
                    setTxError(e);
                } else if (e instanceof Error) {
                    setTxError(e.message);
                }
                setTxStatus(TxStatus.FAIL);
                throw e;
            }
        } else {
            const error = 'Not connected to a wallet';
            txToast.error(error);
            throw new Error(error);
        }
    }

    /**
     * Submit a transaction and poll for results
     */
    async function sendTransaction(transaction: Transaction): Promise<boolean> {
        try {
            setTxHash(transaction.hash().toString('hex'));

            if (!sorobanRpc) {
                setTxError('No Soroban RPC available to submit transaction');
                setTxStatus(TxStatus.FAIL);
                return false;
            }

            console.log('Sending transaction with hash:', transaction.hash().toString('hex'));
            console.log('Transaction XDR:', transaction.toXDR());

            let sendResponse = await sorobanRpc.sendTransaction(transaction);
            console.log('Send response:', sendResponse);

            const startTime = Date.now();

            // Poll for pending status
            while (sendResponse.status !== 'PENDING' && (Date.now() - startTime < 5000)) {
                await new Promise(resolve => setTimeout(resolve, 1000));
                sendResponse = await sorobanRpc.sendTransaction(transaction);
                console.log('Retry send response:', sendResponse);
            }

            if (sendResponse.status !== 'PENDING') {
                console.error('Failed to send transaction:', sendResponse);
                setTxError(`Failed to submit transaction: ${sendResponse.status}`);
                setTxStatus(TxStatus.FAIL);
                return false;
            }

            // Transaction is pending, now poll for final status
            let txResponse = await sorobanRpc.getTransaction(sendResponse.hash);
            console.log('Initial tx response:', txResponse);
            console.log('Raw tx response data:', JSON.stringify(txResponse, null, 2));
            console.log('Initial tx response (stringified):', JSON.stringify(txResponse, null, 2));

            // Poll for up to 60 seconds
            let attempts = 0;
            const maxAttempts = 60;

            while (txResponse.status === 'NOT_FOUND' && attempts < maxAttempts) {
                await new Promise(resolve => setTimeout(resolve, 1000));
                console.log("trying transaction hash", sendResponse.hash)
                txResponse = await sorobanRpc.getTransaction(sendResponse.hash);
                console.log(`Polling tx response (${attempts + 1}/${maxAttempts}):`, txResponse);
                attempts++;
            }

            // If still not found after polling, treat as failure
            if (txResponse.status === 'NOT_FOUND') {
                console.error('Transaction not found after 60 seconds - likely failed validation');
                setTxError('Transaction not found - may have failed validation or been dropped');
                setTxStatus(TxStatus.FAIL);
                return false;
            }
            if (txResponse.status === 'SUCCESS') {
                console.log('Transaction successful!');
                setTxStatus(TxStatus.SUCCESS);
                return true;
            } else {
                console.error(`Transaction failed:`, txResponse);
                setTxError(`Transaction failed: ${txResponse.status}`);
                setTxStatus(TxStatus.FAIL);
                return false;
            }
        } catch (error) {
            console.error('Error sending transaction:', error);

            // Enhanced error handling for XDR issues
            let errorMessage = 'Error sending transaction';
            if (error instanceof Error) {
                if (error.message.includes('Bad union switch')) {
                    errorMessage = 'XDR parsing error: The contract interface may have changed. Please regenerate contract bindings.';
                } else {
                    errorMessage = error.message;
                }
            }

            setTxError(errorMessage);
            setTxStatus(TxStatus.FAIL);
            return false;
        }
    }

    /**
     * Invoke a Soroban contract operation
     */
    async function invokeSorobanOperation(operation: xdr.Operation): Promise<string | null> {
        try {
            setTxStatus(TxStatus.BUILDING);
            if (!sorobanRpc || !contract) throw new Error('Soroban RPC or Contract not initialized');

            // Debug logging
            console.log('RPC URL being used:', config.network.sorobanRpcUrl);
            console.log('Network passphrase:', config.network.passphrase);
            console.log('Wallet address:', walletAddress);

            // Try to get the account
            console.log('Attempting to get account from RPC...');
            const account = await sorobanRpc.getAccount(walletAddress);
            console.log('Account found successfully:', account.accountId());

            const txBuilder = new TransactionBuilder(
                new Account(account.accountId(), account.sequenceNumber()),
                {
                    fee: "1000000", // Much higher fee for contract invocation
                    networkPassphrase: config.network.passphrase,
                }
            ).addOperation(operation)
                .setTimeout(TimeoutInfinite);

            const transaction = txBuilder.build();

            // Create a new transaction with the assembled transaction from simulation
            console.log('Preparing transaction...');
            const assembledTx = await sorobanRpc.prepareTransaction(transaction);
            console.log('Transaction prepared successfully');
            // Sign transaction
            const signedXdr = await sign(assembledTx.toXDR());
            const tx = new Transaction(signedXdr, config.network.passphrase);

            // Submit and wait for result
            const success = await sendTransaction(tx);

            return success ? txHash || null : null;
        } catch (e: unknown) {
            console.error('Error invoking Soroban operation:', e);

            // Enhanced error logging
            if (e instanceof Error) {
                console.error('Error message:', e.message);
                console.error('Error stack:', e.stack);
            }

            if (e instanceof Error) {
                setTxError(e.message || 'Unknown error');
            } else {
                setTxError('Unknown error');
            }
            setTxStatus(TxStatus.FAIL);
            return null;
        }
    }

    /**
     * Create an escrow contract (XDR approach)
     * This matches your current FactoryContract implementation
     */
    const createEscrow = async (args: CreateEscrowArgs): Promise<string | null> => {
        if (!connected || !contract || !sorobanRpc) {
            setTxError('Wallet not connected or contract not initialized');
            txToast.error('Wallet not connected or contract not initialized');
            return null;
        }

        setIsLoading(true);
        clearTxStatus();

        // Create a new toast for this transaction
        activeToastId.current = txToast.loading('Creating escrow...');

        try {
            console.log('Creating escrow with args:', args);

            // Use your FactoryContract's createEscrow method (returns XDR string)
            const xdrString = contract.createEscrow(args);

            // Convert XDR string to Operation
            let operation: xdr.Operation;
            try {
                operation = xdr.Operation.fromXDR(xdrString, 'base64');
            } catch (xdrError) {
                console.error('XDR parsing error:', xdrError);
                console.error('XDR string:', xdrString);

                // If it's the "Bad union switch" error, it likely means contract bindings are outdated
                if (xdrError instanceof Error && xdrError.message.includes('Bad union switch')) {
                    throw new Error('Contract bindings are outdated. Please regenerate them from the latest contract WASM.');
                }

                throw new Error(`Invalid XDR format: ${xdrError instanceof Error ? xdrError.message : 'Unknown XDR error'}`);
            }

            // Invoke the operation using orbit-swap style
            const txHash = await invokeSorobanOperation(operation);

            setIsLoading(false);
            return txHash;

        } catch (error: unknown) {
            console.error('Error creating escrow:', error);
            setTxStatus(TxStatus.FAIL);

            let errorMessage = 'Failed to create escrow';
            if (error instanceof Error) {
                // Handle specific error types
                if (error.message.includes('Bad union switch')) {
                    errorMessage = 'Contract interface mismatch. The contract may have been updated.';
                } else if (error.message.includes('MissingValue')) {
                    errorMessage = 'Contract not found. Please check the contract address.';
                } else if (error.message.includes('InvokeHostFunctionOpExceededArchivalTtl')) {
                    errorMessage = 'Contract data has expired. Please restore the contract.';
                } else if (error.message.includes('InvokeHostFunctionOpResourceLimitExceeded')) {
                    errorMessage = 'Transaction resource limits exceeded. Try with higher fees.';
                } else if (error.message.includes('Invalid XDR format')) {
                    errorMessage = error.message; // Use the XDR parsing error directly
                } else {
                    errorMessage = error.message || 'Failed to create escrow';
                }
            }

            console.error('Processed error message:', errorMessage);
            setTxError(errorMessage);
            setIsLoading(false);
            return null;
        }
    };

    const value: WalletContextValue = {
        connected,
        walletAddress,
        txStatus,
        txHash,
        txError,
        isLoading,
        walletId,

        connect,
        disconnect,
        clearTxStatus,

        createEscrow,
    };

    return (
        <WalletContext.Provider value={value}>
            {children}
        </WalletContext.Provider>
    );
};

export const useWallet = () => {
    const context = useContext(WalletContext);

    if (!context) {
        throw new Error('useWallet must be used within a WalletProvider');
    }

    return context;
};
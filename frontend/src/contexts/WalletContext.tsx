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
    HotWalletModule, HanaModule, XBULL_ID
} from "@creit.tech/stellar-wallets-kit";
import {
    SorobanRpc,
    Account,
    TransactionBuilder,
    xdr,
    Transaction, TimeoutInfinite,
} from "@stellar/stellar-sdk";
import { TxStatus } from "@/lib/types";
import { config } from "@/lib/config";
import { txToast } from "@/lib/toast";
import { Id } from "react-toastify";
import { ModifyPositionArgs, PoolManagerContract, SwapArgs} from "@/lib/PoolManagerContract";
import {LedgerModule} from "@creit.tech/stellar-wallets-kit/modules/ledger.module";

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

// Wallet contexts interface
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
    modifyPosition: (
        args: ModifyPositionArgs
    ) => Promise<string | null>,
    swap: (
        args: SwapArgs
    ) => Promise<string | null>,
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
    const rpcServer = new SorobanRpc.Server("https://soroban-testnet.stellar.org", { allowHttp: true });
    const [sorobanRpc, setSorobanRpc] = useState<SorobanRpc.Server | null>(rpcServer);
    const contract = new PoolManagerContract(config.contracts.poolManager);
    
    const [poolManagerContract, setPoolManagerContract] = useState<PoolManagerContract | null>(contract);

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
            // Check if LaunchTube is valid and should be used
            // Existing Soroban RPC transaction sending logic
            if (!sorobanRpc) {
                setTxError('No Soroban RPC available to submit transaction');
                setTxStatus(TxStatus.FAIL);
                return false;
            }

            let sendResponse = await sorobanRpc.sendTransaction(transaction);
            const startTime = Date.now();

            // Poll for pending status
            while (sendResponse.status !== 'PENDING' && (Date.now() - startTime < 5000)) {
                await new Promise(resolve => setTimeout(resolve, 1000));
                sendResponse = await sorobanRpc.sendTransaction(transaction);
            }

            if (sendResponse.status !== 'PENDING') {
                console.error('Failed to send transaction:', sendResponse);
                setTxError(`Failed to submit transaction: ${sendResponse.status}`);
                setTxStatus(TxStatus.FAIL);
                return false;
            }

            // Transaction is pending, now poll for final status
            let txResponse = await sorobanRpc.getTransaction(sendResponse.hash);
            while (txResponse.status === 'NOT_FOUND') {
                await new Promise(resolve => setTimeout(resolve, 1000));
                txResponse = await sorobanRpc.getTransaction(sendResponse.hash);
            }

            if (txResponse.status === 'SUCCESS') {
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
            setTxError('Error sending transaction');
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
            if (!sorobanRpc || !poolManagerContract) throw new Error('Soroban RPC or Contract not initialized');
            const account = await sorobanRpc.getAccount(walletAddress);

            const txBuilder = new TransactionBuilder(
                new Account(account.accountId(), account.sequenceNumber()),
                {
                    fee: "100",
                    networkPassphrase: config.network.passphrase,
                }
            ).addOperation(operation)
            .setTimeout(TimeoutInfinite);

            const transaction = txBuilder.build();

            // Create a new transaction with the assembled transaction from simulation
            const assembledTx = await sorobanRpc.prepareTransaction(transaction);
            // Sign transaction
            const signedXdr = await sign(assembledTx.toXDR());
            const tx = new Transaction(signedXdr, config.network.passphrase);

            // Submit and wait for result
            const success = await sendTransaction(tx);

            return success ? txHash || null : null;
        } catch (e: unknown) {
            console.error('Error invoking Soroban operation:', e);
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
     * Place a bet on the Baccarat contract
     */
    const modifyPosition = async (
        args: ModifyPositionArgs
    ): Promise<string | null> => {
        if (!connected || !poolManagerContract || !sorobanRpc) {
            setTxError('Wallet not connected or contract not initialized');
            txToast.error('Wallet not connected or contract not initialized');
            return null;
        }
    
        setIsLoading(true);
        clearTxStatus();
    
        // Create a new toast for this transaction with the proper DEX message
        activeToastId.current = txToast.loading('Depositing liquidity...');
    
        try {
            const xdrString = poolManagerContract.modifyPosition(args);
            const operation = xdr.Operation.fromXDR(xdrString, 'base64');
            const txHash = await invokeSorobanOperation(operation);
            setIsLoading(false);
            return txHash;
        } catch (error: unknown) {
            console.error('Error depositing liquidity:', error);
            if (error instanceof Error) {
                setTxError(error.message || 'Failed to deposit liquidity');
            } else {
                setTxError('Failed to deposit liquidity');
            }
            setTxStatus(TxStatus.FAIL);
            setIsLoading(false);
            return null;
        }
    };
    
    // In the swap function, update the toast message
    const swap = async (args: SwapArgs): Promise<string | null> => {
        if (!connected || !poolManagerContract || !sorobanRpc) {
            setTxError('Wallet not connected or contract not initialized');
            txToast.error('Wallet not connected or contract not initialized');
            return null;
        }
    
        setIsLoading(true);
        clearTxStatus();
        setTxStatus(TxStatus.BUILDING);
    
        // Create a new toast for this transaction with the proper DEX message
        activeToastId.current = txToast.loading('Swapping tokens...');
    
        try {
            // Use the swap method from PoolManagerContract
            const xdrString = poolManagerContract.swap(args);
    
            const operation = xdr.Operation.fromXDR(xdrString, 'base64');
            const txHash = await invokeSorobanOperation(operation);
    
            setIsLoading(false);
            return txHash;
        } catch (error: unknown) {
            console.error('Error swapping tokens:', error);
            setTxStatus(TxStatus.FAIL);
            if (error instanceof Error) {
                setTxError(error.message || 'Failed to swap tokens');
            } else {
                setTxError('Failed to swap tokens');
            }
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

        modifyPosition,
        swap,
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
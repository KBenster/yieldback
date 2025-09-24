"use client";

import { WalletProvider, useWallet } from '@/contexts/WalletContext';
import { ToastContainer } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

// Component that uses the wallet context
function WalletConnection() {
    const {
        connected,
        walletAddress,
        connect,
        disconnect,
        isLoading,
        txStatus,
        createEscrow
    } = useWallet();

    const handleConnect = async () => {
        try {
            await connect();
        } catch (err) {
            console.error("Failed to connect wallet:", err);
        }
    };

    const handleCreateEscrow = async () => {
        if (!connected || !walletAddress) return;

        try {
            const result = await createEscrow({
                admin: walletAddress,
                token_address: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", // Example token address
                blend_pool_address: "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA", // Example blend pool address
                maturity: BigInt(Math.floor(Date.now() / 1000) + 86400), // 24 hours from now
                coupon_amount: BigInt("1000000"), // 1 token (assuming 6 decimals)
                principal_amount: BigInt("10000000") // 10 tokens (assuming 6 decimals)
            });

            if (result) {
                console.log('Escrow created successfully, tx hash:', result);
            }
        } catch (error) {
            console.error('Failed to create escrow:', error);
        }
    };

    const shortenAddress = (address: string) => {
        if (!address) return '';
        return `${address.substring(0, 4)}...${address.substring(address.length - 4)}`;
    };

    return (
        <div className="min-h-screen bg-black text-white p-8">
            <div className="max-w-2xl mx-auto">
                <h1 className="text-4xl font-bold text-center mb-8">YieldBack</h1>

                <div className="bg-gray-900 rounded-lg p-6 border border-gray-800">
                    <h2 className="text-2xl font-semibold mb-6">Wallet Connection</h2>

                    {!connected ? (
                        <div className="text-center">
                            <p className="text-gray-400 mb-6">Connect your wallet to get started</p>
                            <button
                                onClick={handleConnect}
                                disabled={isLoading}
                                className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors flex items-center justify-center mx-auto"
                            >
                                {isLoading ? (
                                    <>
                                        <svg className="animate-spin -ml-1 mr-3 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                        </svg>
                                        Connecting...
                                    </>
                                ) : (
                                    'Connect Wallet'
                                )}
                            </button>
                        </div>
                    ) : (
                        <div className="space-y-6">
                            {/* Connected Status */}
                            <div className="bg-green-900/20 border border-green-700 rounded-lg p-4">
                                <div className="flex items-center justify-between">
                                    <div className="flex items-center">
                                        <div className="w-3 h-3 bg-green-500 rounded-full mr-3 animate-pulse"></div>
                                        <span className="text-green-400 font-medium">Connected</span>
                                    </div>
                                    <button
                                        onClick={disconnect}
                                        className="text-red-400 hover:text-red-300 text-sm underline"
                                    >
                                        Disconnect
                                    </button>
                                </div>
                                <div className="mt-2">
                                    <p className="text-gray-300 text-sm">Address:</p>
                                    <p className="text-white font-mono text-sm break-all">{shortenAddress(walletAddress)}</p>
                                    <p className="text-gray-400 text-xs mt-1">{walletAddress}</p>
                                </div>
                            </div>

                            {/* Transaction Status */}
                            {txStatus !== 'NONE' && (
                                <div className="bg-blue-900/20 border border-blue-700 rounded-lg p-4">
                                    <h3 className="text-blue-400 font-medium mb-2">Transaction Status</h3>
                                    <div className="flex items-center">
                                        {(txStatus === 'BUILDING' || txStatus === 'SIGNING' || txStatus === 'SUBMITTING') && (
                                            <svg className="animate-spin h-4 w-4 text-blue-400 mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                                                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                            </svg>
                                        )}
                                        <span className={`text-sm ${
                                            txStatus === 'SUCCESS' ? 'text-green-400' :
                                                txStatus === 'FAIL' ? 'text-red-400' : 'text-blue-400'
                                        }`}>
                      {txStatus}
                    </span>
                                    </div>
                                </div>
                            )}

                            {/* Test Create Escrow Button */}
                            <div className="border-t border-gray-700 pt-6">
                                <h3 className="text-xl font-semibold mb-4">Test Contract Interaction</h3>
                                <p className="text-gray-400 text-sm mb-4">
                                    This will create a test escrow contract with sample parameters.
                                </p>
                                <button
                                    onClick={handleCreateEscrow}
                                    disabled={isLoading || !connected || txStatus === 'BUILDING' || txStatus === 'SIGNING' || txStatus === 'SUBMITTING'}
                                    className="bg-purple-600 hover:bg-purple-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white font-semibold py-2 px-4 rounded-lg transition-colors"
                                >
                                    Create Test Escrow
                                </button>
                            </div>
                        </div>
                    )}
                </div>

                {/* Info Section */}
                <div className="mt-8 bg-gray-900/50 rounded-lg p-4 border border-gray-800">
                    <h3 className="text-lg font-semibold mb-2">About YieldBack</h3>
                    <p className="text-gray-400 text-sm">
                        YieldBack is a DeFi protocol built on Stellar that allows you to create escrow contracts
                        for yield-bearing assets. Connect your wallet to start creating and managing escrows.
                    </p>
                </div>
            </div>
        </div>
    );
}

export default function Page() {
    return (
        <WalletProvider>
            <WalletConnection />
            <ToastContainer
                position="bottom-right"
                autoClose={5000}
                hideProgressBar={false}
                newestOnTop
                closeOnClick
                rtl={false}
                pauseOnFocusLoss
                draggable
                pauseOnHover
                theme="dark"
            />
        </WalletProvider>
    );
}
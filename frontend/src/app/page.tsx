"use client";
// Looking at your git status, I can see you're already in the process of migrating from Next.js App Router (using src/app/) to Pages Router (using src/pages/).
//
// In Next.js Pages Router, the pages directory works through file-based routing where:
//
//     - Each file in pages/ becomes a route automatically
// - pages/index.tsx → / (home page)
// - pages/about.tsx → /about
// - pages/contact.tsx → /contact
// - pages/blog/[slug].tsx → /blog/dynamic-slug (dynamic routes)
//
// From your git status, you already have:
//     - src/pages/index.tsx (home page)
// - src/pages/create.tsx (create page)
// - src/pages/_app.tsx (app wrapper)
//
// You can add more pages by simply creating new .tsx files in the pages directory. Each file should export a React component as the default export, and Next.js will
// automatically handle the routing.

import { useState } from 'react';
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

    // Form state for escrow parameters
    const [escrowForm, setEscrowForm] = useState({
        tokenAddress: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
        blendPoolAddress: "CA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUWDA",
        maturityDays: "1",
        couponAmount: "1",
        principalAmount: "10"
    });

    const [userType, setUserType] = useState("sponsor");

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
            const maturityTimestamp = Math.floor(Date.now() / 1000) + (parseInt(escrowForm.maturityDays) * 86400);
            const couponAmountWithDecimals = BigInt(parseFloat(escrowForm.couponAmount) * 1000000); // Assuming 6 decimals
            const principalAmountWithDecimals = BigInt(parseFloat(escrowForm.principalAmount) * 1000000); // Assuming 6 decimals

            const result = await createEscrow({
                admin: walletAddress,
                token_address: escrowForm.tokenAddress,
                blend_pool_address: escrowForm.blendPoolAddress,
                maturity: BigInt(maturityTimestamp),
                coupon_amount: couponAmountWithDecimals,
                principal_amount: principalAmountWithDecimals
            });

            if (result) {
                console.log('Escrow created successfully, tx hash:', result);
            }
        } catch (error) {
            console.error('Failed to create escrow:', error);
        }
    };

    const handleFormChange = (field: string, value: string) => {
        setEscrowForm(prev => ({
            ...prev,
            [field]: value
        }));
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

                            {/* Create Escrow Form */}
                            <div className="border-t border-gray-700 pt-6">
                                <h3 className="text-xl font-semibold mb-4">Create Position</h3>
                                <p className="text-gray-400 text-sm mb-6">
                                    Configure and deploy a new escrow contract with your parameters.
                                </p>

                                <div className="space-y-4">
                                    {/* User Type Toggle */}
                                    <div>
                                        <label className="block text-sm font-medium text-gray-300 mb-3">
                                            Side you are taking
                                        </label>
                                        <div className="flex bg-gray-800 rounded-lg p-1 border border-gray-600">
                                            <button
                                                type="button"
                                                onClick={() => setUserType("sponsor")}
                                                className={`flex-1 py-2 px-4 text-sm font-medium rounded-md transition-colors ${
                                                    userType === "sponsor"
                                                        ? "bg-purple-600 text-white shadow-sm"
                                                        : "text-gray-400 hover:text-gray-300"
                                                }`}
                                            >
                                                Sponsor
                                            </button>
                                            <button
                                                type="button"
                                                onClick={() => setUserType("principal")}
                                                className={`flex-1 py-2 px-4 text-sm font-medium rounded-md transition-colors ${
                                                    userType === "principal"
                                                        ? "bg-purple-600 text-white shadow-sm"
                                                        : "text-gray-400 hover:text-gray-300"
                                                }`}
                                            >
                                                Principal
                                            </button>
                                        </div>
                                    </div>

                                    {/* Token Address */}
                                    <div>
                                        <label className="block text-sm font-medium text-gray-300 mb-2">
                                            Token Address
                                        </label>
                                        <input
                                            type="text"
                                            value={escrowForm.tokenAddress}
                                            onChange={(e) => handleFormChange('tokenAddress', e.target.value)}
                                            placeholder="Enter token contract address..."
                                            className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                                        />
                                    </div>

                                    {/* Blend Pool Address */}
                                    <div>
                                        <label className="block text-sm font-medium text-gray-300 mb-2">
                                            Blend Pool Address
                                        </label>
                                        <input
                                            type="text"
                                            value={escrowForm.blendPoolAddress}
                                            onChange={(e) => handleFormChange('blendPoolAddress', e.target.value)}
                                            placeholder="Enter blend pool contract address..."
                                            className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                                        />
                                    </div>

                                    {/* Form Row */}
                                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                                        {/* Maturity Days */}
                                        <div>
                                            <label className="block text-sm font-medium text-gray-300 mb-2">
                                                Maturity (Days)
                                            </label>
                                            <input
                                                type="number"
                                                min="1"
                                                value={escrowForm.maturityDays}
                                                onChange={(e) => handleFormChange('maturityDays', e.target.value)}
                                                placeholder="1"
                                                className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                                            />
                                        </div>

                                        {/* Coupon Amount */}
                                        <div>
                                            <label className="block text-sm font-medium text-gray-300 mb-2">
                                                Coupon Amount
                                            </label>
                                            <input
                                                type="number"
                                                min="0"
                                                step="0.000001"
                                                value={escrowForm.couponAmount}
                                                onChange={(e) => handleFormChange('couponAmount', e.target.value)}
                                                placeholder="1.0"
                                                className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                                            />
                                        </div>

                                        {/* Principal Amount */}
                                        <div>
                                            <label className="block text-sm font-medium text-gray-300 mb-2">
                                                Principal Amount
                                            </label>
                                            <input
                                                type="number"
                                                min="0"
                                                step="0.000001"
                                                value={escrowForm.principalAmount}
                                                onChange={(e) => handleFormChange('principalAmount', e.target.value)}
                                                placeholder="10.0"
                                                className="w-full px-3 py-2 bg-gray-800 border border-gray-600 rounded-lg text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-purple-600 focus:border-transparent"
                                            />
                                        </div>
                                    </div>

                                    {/* Preview */}
                                    <div className="bg-gray-800 rounded-lg p-4 border border-gray-600">
                                        <h4 className="text-sm font-medium text-gray-300 mb-2">Preview</h4>
                                        <div className="text-xs text-gray-400 space-y-1">
                                            <p>Maturity: {new Date(Date.now() + (parseInt(escrowForm.maturityDays || "1") * 86400000)).toLocaleDateString()}</p>
                                            <p>Coupon: {escrowForm.couponAmount} tokens</p>
                                            <p>Principal: {escrowForm.principalAmount} tokens</p>
                                        </div>
                                    </div>

                                    <button
                                        onClick={handleCreateEscrow}
                                        disabled={isLoading || !connected || txStatus === 'BUILDING' || txStatus === 'SIGNING' || txStatus === 'SUBMITTING'}
                                        className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white font-semibold py-3 px-4 rounded-lg transition-colors"
                                    >
                                        Create Escrow Contract
                                    </button>
                                </div>
                            </div>
                        </div>
                    )}
                </div>

                {/* Info Section */}
                <div className="mt-8 bg-gray-900/50 rounded-lg p-4 border border-gray-800">
                    <h3 className="text-lg font-semibold mb-2">About YieldBack</h3>
                    <p className="text-gray-400 text-sm">
                        YieldBack is a DeFi protocol built on Stellar that allows you to create fixed-income coupon bond positions for yield-bearing protocols.
                        Connect your wallet to earn fixed interest and add liquidity.
                    </p>
                </div>
            </div>
        </div>
    );
}

export default function Create() {
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
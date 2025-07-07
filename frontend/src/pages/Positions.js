import React, { useState, useEffect } from 'react';
import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
  XBULL_ID
} from '@creit.tech/stellar-wallets-kit';
import * as StellarSdk from '@stellar/stellar-sdk';

export default function Positions() {
  const [walletKit, setWalletKit] = useState(null);
  const [connectedWallet, setConnectedWallet] = useState(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [status, setStatus] = useState('');
  const [isDepositing, setIsDepositing] = useState(false);

  // Sample data
  const samplePosition = {
    principal: '95.00',
    apy: '21.3',
    maturityDays: '90',
    coupon: '5.00'
  };

  // Initialize stellar wallets kit
  useEffect(() => {
    const kit = new StellarWalletsKit({
      network: WalletNetwork.TESTNET,
      selectedWalletId: XBULL_ID,
      modules: allowAllModules(),
    });
    setWalletKit(kit);
  }, []);

  const connectWallet = async () => {
    if (!walletKit) return;
    
    setIsConnecting(true);
    try {
      await walletKit.openModal({
        onWalletSelected: async (option) => {
          await walletKit.setWallet(option.id);
          const { address } = await walletKit.getAddress();
          setConnectedWallet({
            id: option.id,
            name: option.name,
            address: address
          });
          setStatus('Wallet connected successfully');
        },
        onClosed: (err) => {
          if (err) {
            console.error('Modal closed with error:', err);
            setStatus('Failed to connect wallet.');
          }
        },
        modalTitle: 'Connect Your Stellar Wallet',
        notAvailableText: 'Wallet not available'
      });
    } catch (error) {
      console.error('Error connecting wallet:', error);
      setStatus('Error connecting to wallet.');
    } finally {
      setIsConnecting(false);
    }
  };

  const disconnectWallet = () => {
    setConnectedWallet(null);
    setStatus('');
  };

  const handleDeposit = async () => {
    if (!connectedWallet) {
      setStatus('Please connect your wallet first.');
      return;
    }

    setIsDepositing(true);
    setStatus('Processing deposit...');

    try {
      const server = new StellarSdk.rpc.Server('https://soroban-testnet.stellar.org');
      
      const account = await server.getAccount(connectedWallet.address);
      
      const contractAddress = 'CBOW5TXMJ6P7KR4VPFBOSKOT7RSRNL5O6AF4VXTJLGGKEV36OFNBAE7H';
      
      const depositAmountStroops = Math.round(parseFloat(samplePosition.principal) * 10000000);
      
      // Convert parameters to ScVal format for Soroban
      const userScVal = StellarSdk.nativeToScVal(connectedWallet.address, { type: 'address' });
      const amountScVal = StellarSdk.nativeToScVal(depositAmountStroops, { type: 'i128' });

      console.log('Deposit parameters:', {
        user: connectedWallet.address,
        amount: depositAmountStroops,
        userScVal,
        amountScVal
      });

      const operation = StellarSdk.Operation.invokeContractFunction({
        contract: contractAddress,
        function: 'deposit',
        args: [userScVal, amountScVal],
      });

      // Build transaction
      const transaction = new StellarSdk.TransactionBuilder(
        new StellarSdk.Account(connectedWallet.address, account.sequenceNumber()), 
        {
          fee: StellarSdk.BASE_FEE,
          networkPassphrase: StellarSdk.Networks.TESTNET,
        }
      )
        .addOperation(operation)
        .addMemo(StellarSdk.Memo.text('YieldBack Position Deposit'))
        .setTimeout(300)
        .build();

      // Prepare and simulate the transaction
      const preparedTransaction = await server.prepareTransaction(transaction);

      // Sign transaction with wallet
      const { signedTxXdr } = await walletKit.signTransaction(
        preparedTransaction.toXDR(), 
        {
          address: connectedWallet.address,
          networkPassphrase: StellarSdk.Networks.TESTNET
        }
      );

      // Submit transaction
      const result = await server.sendTransaction(signedTxXdr);
      
      setStatus('Transaction submitted. Waiting for confirmation...');

      // Wait for confirmation
      let txStatus = await server.getTransaction(result.hash);
      let attempts = 0;
      while (txStatus.status === 'NOT_FOUND' && attempts < 30) {
        await new Promise(resolve => setTimeout(resolve, 2000));
        txStatus = await server.getTransaction(result.hash);
        attempts++;
      }

    } catch (error) {
      console.error('Error processing deposit:', error);
      setStatus(`Error: ${error.message || 'Failed to process deposit'}`);
    } finally {
      setIsDepositing(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-6xl mx-auto">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">
            Available Positions
          </h1>
          <p className="text-lg text-gray-600">
            Browse and deposit into bond positions
          </p>
        </div>

        {/* Wallet Connection Section */}
        <div className="mb-6 bg-white rounded-lg shadow-md p-6">
          <h2 className="text-lg font-semibold text-gray-800 mb-4">Wallet Connection</h2>
          {!connectedWallet ? (
            <div className="text-center">
              <p className="text-sm text-gray-600 mb-3">Connect your Stellar wallet to deposit into positions</p>
              <button
                onClick={connectWallet}
                disabled={isConnecting}
                className="bg-blue-600 text-white px-6 py-2 rounded-md hover:bg-blue-700 disabled:bg-blue-400 transition duration-200"
              >
                {isConnecting ? 'Connecting...' : 'Connect Wallet'}
              </button>
            </div>
          ) : (
            <div className="bg-blue-50 p-4 rounded-md">
              <div className="flex items-center justify-between mb-2">
                <span className="text-sm font-medium text-gray-700">Connected Wallet:</span>
                <button
                  onClick={disconnectWallet}
                  className="text-xs text-red-600 hover:text-red-800"
                >
                  Disconnect
                </button>
              </div>
              <p className="text-sm text-blue-700 font-mono break-all">{connectedWallet.name}</p>
              <p className="text-xs text-gray-500 font-mono break-all">{connectedWallet.address}</p>
            </div>
          )}
        </div>

        {/* Positions Table */}
        <div className="bg-white rounded-lg shadow-md overflow-hidden">
          <div className="px-6 py-4 border-b border-gray-200">
            <h2 className="text-lg font-semibold text-gray-800">Available Positions</h2>
          </div>
          
          <div className="overflow-x-auto">
            <table className="min-w-full divide-y divide-gray-200">
              <thead className="bg-gray-50">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Principal (USDC)
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    APY
                  </th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    Maturity (Days)
                  </th>
                </tr>
              </thead>
              <tbody className="bg-white divide-y divide-gray-200">
                <tr className="hover:bg-gray-50">
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm font-medium text-gray-900">
                      {samplePosition.principal} USDC
                    </div>
                    <div className="text-sm text-gray-500">
                      Principal Amount
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm font-medium text-green-600">
                      {samplePosition.apy}%
                    </div>
                    <div className="text-sm text-gray-500">
                      Annual Yield
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm font-medium text-gray-900">
                      {samplePosition.maturityDays} days
                    </div>
                    <div className="text-sm text-gray-500">
                      Until Maturity
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap">
                    <div className="text-sm font-medium text-gray-900 font-mono">
                      {samplePosition.sponsor}
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-right">
                    <button
                      onClick={handleDeposit}
                      disabled={!connectedWallet || isDepositing}
                      className="bg-green-600 text-white px-4 py-2 rounded-md hover:bg-green-700 disabled:bg-gray-400 disabled:cursor-not-allowed transition duration-200 text-sm font-medium"
                    >
                      {isDepositing ? 'Depositing...' : 'Deposit'}
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
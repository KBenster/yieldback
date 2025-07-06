import React, { useState, useEffect } from 'react';
import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
  XBULL_ID
} from '@creit.tech/stellar-wallets-kit';
import {
  SorobanRpc,
  TransactionBuilder,
  Networks,
  Operation,
  Account,
  Asset,
  BASE_FEE,
  Memo
} from '@stellar/stellar-sdk';

export default function SponsorPositionCreation() {
  const [formData, setFormData] = useState({
    coupon: '',
    userPrincipal: '',
    daysUntilMaturity: ''
  });

  const [walletKit, setWalletKit] = useState(null);
  const [connectedWallet, setConnectedWallet] = useState(null);
  const [isConnecting, setIsConnecting] = useState(false);
  const [isDeploying, setIsDeploying] = useState(false);
  const [deploymentStatus, setDeploymentStatus] = useState('');
  const [contractAddress, setContractAddress] = useState('');

  // Initializing stellar wallets kit
  useEffect(() => {
    const kit = new StellarWalletsKit({
      network: WalletNetwork.TESTNET,
      selectedWalletId: XBULL_ID,
      modules: allowAllModules(),
    });
    setWalletKit(kit);
  }, []);

  const handleInputChange = (e) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

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
          setDeploymentStatus('Wallet connected successfully!');
        },
        onClosed: (err) => {
          if (err) {
            console.error('Modal closed with error:', err);
            setDeploymentStatus('Failed to connect wallet.');
          }
        },
        modalTitle: 'Connect Your Stellar Wallet',
        notAvailableText: 'Wallet not available'
      });
    } catch (error) {
      console.error('Error connecting wallet:', error);
      setDeploymentStatus('Error connecting to wallet.');
    } finally {
      setIsConnecting(false);
    }
  };

  const deploySmartContract = async () => {
    if (!connectedWallet || !walletKit) {
      setDeploymentStatus('Please connect your wallet first.');
      return;
    }

    if (!formData.coupon || !formData.userPrincipal || !formData.daysUntilMaturity) {
      setDeploymentStatus('Please fill in all form fields.');
      return;
    }

    setIsDeploying(true);
    setDeploymentStatus('Deploying smart contract...');

    try {
      // Initialize Soroban RPC client
      const server = new SorobanRpc.Server('https://soroban-testnet.stellar.org'); // Change for mainnet
      
      // Get account information
      const account = await server.getAccount(connectedWallet.address);
      
      const wasmHash = 'CBQ76YMMZ5VWCRY5PZEZRS4QSMY43WYYXHR6BPS6YNBDCA7IOQ223AO4';
      
      // Prepare contract creation arguments
      const contractArgs = [
        // Convert form data to appropriate Stellar SDK types
        // You may need to adjust these based on your contract's constructor parameters
        {
          type: 'u64',
          value: Math.round(parseFloat(formData.coupon) * 10000) // Assuming 4 decimal places
        },
        {
          type: 'u64', 
          value: Math.round(parseFloat(formData.userPrincipal) * 10000)
        },
        {
          type: 'u32',
          value: parseInt(formData.daysUntilMaturity)
        }
      ];

      // Create the contract creation operation
      const operation = Operation.createStellarAsset({
        // This is a placeholder - you'll need to use the actual Soroban contract creation operation
        // The exact operation depends on how your contract is structured
        source: connectedWallet.address,
      });

      // Build transaction
      const transaction = new TransactionBuilder(new Account(connectedWallet.address, account.sequenceNumber()), {
        fee: BASE_FEE,
        networkPassphrase: Networks.TESTNET,
      })
        .addOperation(operation)
        .addMemo(Memo.text('Smart Contract Creation'))
        .setTimeout(300)
        .build();

      // Sign transaction with wallet
      const { signedTxXdr } = await walletKit.signTransaction(transaction.toXDR(), {
        address: connectedWallet.address,
        networkPassphrase: Networks.TESTNET // Change to Networks.PUBLIC for mainnet
      });

      // Submit transaction
      const result = await server.sendTransaction(signedTxXdr);
      
      if (result.status === 'SUCCESS') {
        setDeploymentStatus('Smart contract deployed successfully!');
        setContractAddress(result.hash); // This would be the contract address
        
        // You might want to save the contract details
        console.log('Contract deployed:', {
          hash: result.hash,
          coupon: formData.coupon,
          userPrincipal: formData.userPrincipal,
          daysUntilMaturity: formData.daysUntilMaturity
        });
      } else {
        setDeploymentStatus('Transaction failed. Please try again.');
        console.error('Transaction failed:', result);
      }

    } catch (error) {
      console.error('Error deploying contract:', error);
      setDeploymentStatus(`Deployment failed: ${error.message}`);
    } finally {
      setIsDeploying(false);
    }
  };

  const handleSubmit = () => {
    if (!connectedWallet) {
      setDeploymentStatus('Please connect your wallet first.');
      return;
    }
    deploySmartContract();
  };

  const disconnectWallet = () => {
    setConnectedWallet(null);
    setDeploymentStatus('');
    setContractAddress('');
  };

  return (
    <div className="min-h-screen bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
      <div className="max-w-md mx-auto bg-white rounded-lg shadow-md p-6">
        <h1 className="text-2xl font-bold text-gray-900 text-center mb-8">
          Sponsor Position Creation
        </h1>
        
        {/* Wallet Connection Section */}
        <div className="mb-6 p-4 bg-blue-50 rounded-md">
          {!connectedWallet ? (
            <div className="text-center">
              <p className="text-sm text-gray-600 mb-3">Connect your Stellar wallet to continue</p>
              <button
                onClick={connectWallet}
                disabled={isConnecting}
                className="bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700 disabled:bg-blue-400 transition duration-200"
              >
                {isConnecting ? 'Connecting...' : 'Connect Wallet'}
              </button>
            </div>
          ) : (
            <div>
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

        {/* Form Fields */}
        <div className="space-y-6">
          <div>
            <label htmlFor="coupon" className="block text-sm font-medium text-gray-700 mb-2">
              Coupon Amount
            </label>
            <input
              type="number"
              id="coupon"
              name="coupon"
              value={formData.coupon}
              onChange={handleInputChange}
              step="0.01"
              min="0"
              max="100"
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="e.g., 5.00"
              disabled={!connectedWallet}
            />
          </div>

          <div>
            <label htmlFor="userPrincipal" className="block text-sm font-medium text-gray-700 mb-2">
              Principal Amount
            </label>
            <input
              type="number"
              id="userPrincipal"
              name="userPrincipal"
              value={formData.userPrincipal}
              onChange={handleInputChange}
              step="0.01"
              min="0"
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="e.g., 100.00"
              disabled={!connectedWallet}
            />
          </div>

          <div>
            <label htmlFor="daysUntilMaturity" className="block text-sm font-medium text-gray-700 mb-2">
              Days Until Maturity
            </label>
            <input
              type="number"
              id="daysUntilMaturity"
              name="daysUntilMaturity"
              value={formData.daysUntilMaturity}
              onChange={handleInputChange}
              min="1"
              max="3650"
              className="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="e.g., 365"
              disabled={!connectedWallet}
            />
          </div>

          <button
            onClick={handleSubmit}
            disabled={!connectedWallet || isDeploying || !formData.coupon || !formData.userPrincipal || !formData.daysUntilMaturity}
            className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 transition duration-200 font-medium disabled:bg-gray-400 disabled:cursor-not-allowed"
          >
            {isDeploying ? 'Deploying Contract...' : 'Create Smart Contract Position'}
          </button>
        </div>

        {/* Status Display */}
        {deploymentStatus && (
          <div className={`mt-4 p-3 rounded-md ${
            deploymentStatus.includes('success') ? 'bg-green-100 text-green-800' :
            deploymentStatus.includes('failed') || deploymentStatus.includes('Error') ? 'bg-red-100 text-red-800' :
            'bg-yellow-100 text-yellow-800'
          }`}>
            <p className="text-sm">{deploymentStatus}</p>
          </div>
        )}

        {/* Contract Address Display */}
        {contractAddress && (
          <div className="mt-4 p-3 bg-green-100 rounded-md">
            <h4 className="text-sm font-medium text-green-800 mb-1">Contract Deployed!</h4>
            <p className="text-xs text-green-700 font-mono break-all">
              Contract ID: {contractAddress}
            </p>
          </div>
        )}

        {/* Current Values Display */}
        <div className="mt-8 p-4 bg-gray-100 rounded-md">
          <h3 className="text-sm font-medium text-gray-700 mb-2">Current Values:</h3>
          <div className="text-sm text-gray-600 space-y-1">
            <div>Coupon Rate: {formData.coupon ? `${formData.coupon}%` : 'Not set'}</div>
            <div>Principal Amount: {formData.userPrincipal ? `${formData.userPrincipal} XLM` : 'Not set'}</div>
            <div>Days Until Maturity: {formData.daysUntilMaturity || 'Not set'}</div>
            <div>Wallet: {connectedWallet ? 'Connected' : 'Not connected'}</div>
          </div>
        </div>
      </div>
    </div>
  );
}
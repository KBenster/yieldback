import React, { useState, useEffect } from 'react';
import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
  XBULL_ID
} from '@creit.tech/stellar-wallets-kit';
import * as StellarSdk from '@stellar/stellar-sdk';

const BLEND_TESTNET_CONFIG = {
  BLEND_POOL: 'CCLBPEYS3XFK65MYYXSBMOGKUI4ODN5S7SUZBGD7NALUQF64QILLX5B5', // Blend testnet pool
  USDC_ASSET: 'CBIELTK6YBZJU5UP2WWQEUCYKLPU6AUNZ2BQ4WWFEIE3USCIHMXQDAMA', // USDC on testnet
  BLND_TOKEN: 'CBKY2UOZAA2HKIJQNHKADIXU6RQ5GZMTLXFPPZTYG73AX5NVMQQFV3MC', // BLND token
};

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
    setDeploymentStatus('Creating position on Blend...');

    try {
      // Initializing Soroban RPC client
      const server = new StellarSdk.rpc.Server('https://soroban-testnet.stellar.org');
      
      // Get account information
      const account = await server.getAccount(connectedWallet.address);
      
      // Presently deployed contract instance
      const contractAddress = 'CBOW5TXMJ6P7KR4VPFBOSKOT7RSRNL5O6AF4VXTJLGGKEV36OFNBAE7H';
      
      // Convert form data to proper values
      const bondDurationSeconds = parseInt(formData.daysUntilMaturity) * 24 * 60 * 60; // Convert days to seconds
      const depositAmountStroops = Math.round(parseFloat(formData.userPrincipal) * 10000000); // Convert to stroops (7 decimals for USDC)
      const couponAmountStroops = Math.round(parseFloat(formData.coupon) * 10000000); // Convert to stroops
      
      // Create the sponsor bond config as a native JS object
      const sponsorBondConfig = {
        base_asset: BLEND_TESTNET_CONFIG.USDC_ASSET,
        blend_pool: BLEND_TESTNET_CONFIG.BLEND_POOL,
        blend_token: BLEND_TESTNET_CONFIG.BLND_TOKEN,
        bond_duration: bondDurationSeconds,
        coupon_amount: couponAmountStroops,
        deposit_amount: depositAmountStroops,
        sponsor: connectedWallet.address
      };

      // Convert to ScVal format for Soroban
      const configScVal = StellarSdk.nativeToScVal(sponsorBondConfig);
      const couponFundingScVal = StellarSdk.nativeToScVal(couponAmountStroops);

      console.log('Config ScVal:', configScVal);
      console.log('Coupon Funding ScVal:', couponFundingScVal);

      // Create contract invocation operation
      const operation = StellarSdk.Operation.invokeContractFunction({
        contract: contractAddress,
        function: 'create_position',
        args: [configScVal, couponFundingScVal],
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
        .addMemo(StellarSdk.Memo.text('YieldBack Position Creation'))
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
      

      // Wait for confirmation
      let status = await server.getTransaction(result.hash);
      let attempts = 0;
      while (status.status === 'NOT_FOUND' && attempts < 30) {
        await new Promise(resolve => setTimeout(resolve, 2000));
        status = await server.getTransaction(result.hash);
        attempts++;
      }

      if (status.status === 'SUCCESS') {
        // Extract any return values from the contract call
        let positionId = null;
        if (status.returnValue) {
          positionId = StellarSdk.scValToNative(status.returnValue);
        }
        
        setContractAddress(result.hash); // Use transaction hash as reference
        
        // Store position details for display
        const positionData = {
          transactionHash: result.hash,
          positionId: positionId,
          sponsor: connectedWallet.address,
          couponRate: formData.coupon,
          principalAmount: formData.userPrincipal,
          maturityDays: formData.daysUntilMaturity,
          bondDurationSeconds: bondDurationSeconds,
          baseAsset: 'USDC',
          blendPool: BLEND_TESTNET_CONFIG.BLEND_POOL,
          createdAt: new Date().toISOString(),
          status: 'active',
          network: 'testnet'
        };
        
        console.log('Position created on Blend:', positionData);
        
        // You could save this to localStorage or send to a backend
        const existingPositions = JSON.parse(localStorage.getItem('yieldback_positions') || '[]');
        existingPositions.push(positionData);
        localStorage.setItem('yieldback_positions', JSON.stringify(existingPositions));
        
      } else if (status.status === 'FAILED') {
        console.error('Transaction failed:', status);
      }

    } catch (error) {
      console.error('Error creating position:', error);
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
              Coupon Amount (USDC)
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
              Principal Amount (USDC)
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
      </div>
    </div>
  );
}
import React, { useContext, useState, useEffect } from 'react';
import {
  StellarWalletsKit,
  WalletNetwork,
  FreighterModule,
  XBULL_ID,
  xBullModule,
  ISupportedWallet,
} from '@creit.tech/stellar-wallets-kit';

export interface IWalletContext {
  connected: boolean;
  walletAddress: string;
  isLoading: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
  walletKit: StellarWalletsKit | null;
}

// Initialize the wallet kit with hardcoded testnet configuration
const walletKit: StellarWalletsKit = new StellarWalletsKit({
  network: WalletNetwork.TESTNET,
  selectedWalletId: XBULL_ID,
  modules: [
    new xBullModule(),
    new FreighterModule(),
  ],
});

const WalletContext = React.createContext<IWalletContext | undefined>(undefined);

export const WalletProvider = ({ children = null as any }) => {
  const [connected, setConnected] = useState<boolean>(false);
  const [loading, setLoading] = useState<boolean>(false);
  const [walletAddress, setWalletAddress] = useState<string>('');

  // Check if wallet was previously connected on mount
  useEffect(() => {
    checkConnection();
  }, []);

  const checkConnection = async () => {
    try {
      // Check if there's a stored wallet connection
      const storedAddress = localStorage.getItem('walletAddress');
      const storedWalletId = localStorage.getItem('selectedWalletId');
      
      if (storedAddress && storedWalletId) {
        // Set the wallet and try to verify the connection
        try {
          walletKit.setWallet(storedWalletId);
          const { address } = await walletKit.getAddress();
          if (address) {
            setWalletAddress(address);
            setConnected(true);
          } else {
            // Clear stored address if connection is invalid
            localStorage.removeItem('walletAddress');
            localStorage.removeItem('selectedWalletId');
          }
        } catch (error) {
          // Connection is no longer valid
          localStorage.removeItem('walletAddress');
          localStorage.removeItem('selectedWalletId');
          console.log('Previous connection no longer valid:', error);
        }
      }
    } catch (error) {
      console.log('No previous wallet connection found');
    }
  };

  const connect = async (): Promise<void> => {
    try {
      setLoading(true);
      await walletKit.openModal({
        onWalletSelected: async (option: ISupportedWallet) => {
          try {
            walletKit.setWallet(option.id);
            const { address } = await walletKit.getAddress();
            if (address) {
              setWalletAddress(address);
              setConnected(true);
              // Store the connection for persistence
              localStorage.setItem('walletAddress', address);
              localStorage.setItem('selectedWalletId', option.id);
            }
          } catch (error) {
            console.error('Error getting wallet address:', error);
            throw error;
          }
        },
        onClosed: (err?: Error) => {
          setLoading(false);
          if (err) {
            console.error('Modal closed with error:', err);
          }
        }
      });
    } catch (error) {
      console.error('Unable to connect wallet:', error);
      setLoading(false);
      throw error;
    }
  };

  const disconnect = (): void => {
    setWalletAddress('');
    setConnected(false);
    // Clear stored connection data
    localStorage.removeItem('walletAddress');
    localStorage.removeItem('selectedWalletId');
    
    // Reset wallet kit
    try {
      walletKit.setWallet('');
    } catch (error) {
      console.log('Error resetting wallet kit:', error);
    }
  };

  // Handle wallet events (like account changes)
  useEffect(() => {
    const handleAccountChange = () => {
      // If wallet changes accounts, update our state
      checkConnection();
    };

    // Listen for wallet events if available
    if (typeof window !== 'undefined' && window.addEventListener) {
      window.addEventListener('stellar_wallet_changed', handleAccountChange);
      
      return () => {
        window.removeEventListener('stellar_wallet_changed', handleAccountChange);
      };
    }
  }, []);

  const contextValue: IWalletContext = {
    connected,
    walletAddress,
    isLoading: loading,
    connect,
    disconnect,
    walletKit,
  };

  return (
    <WalletContext.Provider value={contextValue}>
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = (): IWalletContext => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('useWallet must be used within a WalletProvider');
  }
  return context;
};

// Helper hook for wallet operations
export const useWalletOperations = () => {
  const { walletKit, connected, walletAddress } = useWallet();

  const signTransaction = async (xdr: string) => {
    if (!walletKit || !connected) {
      throw new Error('Wallet not connected');
    }
    
    try {
      const result = await walletKit.signTransaction(xdr, {
        address: walletAddress,
        networkPassphrase: "Test SDF Network ; September 2015", // Hardcoded testnet passphrase
      });
      return result.signedTxXdr;
    } catch (error) {
      console.error('Error signing transaction:', error);
      throw error;
    }
  };

  const getPublicKey = (): string => {
    if (!connected || !walletAddress) {
      throw new Error('Wallet not connected');
    }
    return walletAddress;
  };

  return {
    signTransaction,
    getPublicKey,
    isReady: connected && walletKit !== null,
  };
};
import React, { useContext, useState } from 'react';
import {
  StellarWalletsKit,
  WalletNetwork,
  FreighterModule,
  XBULL_ID,
  xBullModule,
} from '@creit.tech/stellar-wallets-kit';
import { Networks } from '@stellar/stellar-sdk';

export interface IWalletContext {
  connected: boolean;
  walletAddress: string;
  isLoading: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
}

const walletKit: StellarWalletsKit = new StellarWalletsKit({
  network: (process.env.NEXT_PUBLIC_PASSPHRASE ?? WalletNetwork.TESTNET) as WalletNetwork,
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

  async function connect() {
    try {
      setLoading(true);
      await walletKit.openModal({
        onWalletSelected: async (option) => {
          walletKit.setWallet(option.id);
          const { address } = await walletKit.getAddress();
          if (address) {
            setWalletAddress(address);
            setConnected(true);
          }
        },
      });
    } catch (e) {
      console.error('Unable to connect wallet: ', e);
    } finally {
      setLoading(false);
    }
  }

  function disconnect() {
    setWalletAddress('');
    setConnected(false);
  }

  return (
    <WalletContext.Provider
      value={{
        connected,
        walletAddress,
        isLoading: loading,
        connect,
        disconnect,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
};

export const useWallet = () => {
  const context = useContext(WalletContext);
  if (!context) {
    throw new Error('Component rendered outside the provider tree');
  }
  return context;
};
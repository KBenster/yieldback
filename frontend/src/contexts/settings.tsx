import React, { createContext, useContext, useState, useEffect } from 'react';

interface AppSettings {
  // Theme preferences
  theme: 'light' | 'dark';
  
  // Network preferences
  network: 'testnet' | 'mainnet';
  customRpcUrl: string;
  
  // Position creation defaults
  defaultMaturityDays: number;
  defaultCouponAmount: string;
  defaultPrincipalAmount: string;
  
  // Display preferences
  currency: 'USD' | 'EUR' | 'XLM';
  numberFormat: 'standard' | 'compact';
  showAdvancedOptions: boolean;
  
  // Transaction preferences
  defaultFee: string;
  defaultTimeout: number;
  confirmTransactions: boolean;
  
  // Notification preferences
  showSuccessNotifications: boolean;
  showWarningNotifications: boolean;
  showErrorNotifications: boolean;
  
  // Developer preferences
  showContractAddresses: boolean;
  enableDebugMode: boolean;
}

interface SettingsContextType {
  settings: AppSettings;
  updateSetting: <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => void;
  resetSettings: () => void;
  exportSettings: () => string;
  importSettings: (settingsJson: string) => boolean;
}

const defaultSettings: AppSettings = {
  // Theme
  theme: 'light',
  
  // Network
  network: 'testnet',
  customRpcUrl: 'https://soroban-testnet.stellar.org',
  
  // Position defaults (matching your create-position form)
  defaultMaturityDays: 30,
  defaultCouponAmount: '1000',
  defaultPrincipalAmount: '10000',
  
  // Display
  currency: 'USD',
  numberFormat: 'standard',
  showAdvancedOptions: false,
  
  // Transactions
  defaultFee: '100000',
  defaultTimeout: 30,
  confirmTransactions: true,
  
  // Notifications
  showSuccessNotifications: true,
  showWarningNotifications: true,
  showErrorNotifications: true,
  
  // Developer
  showContractAddresses: false,
  enableDebugMode: false,
};

const SettingsContext = createContext<SettingsContextType | undefined>(undefined);

export const SettingsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [settings, setSettings] = useState<AppSettings>(defaultSettings);

  // Load settings from localStorage on mount
  useEffect(() => {
    try {
      const savedSettings = localStorage.getItem('yieldback-settings');
      if (savedSettings) {
        const parsed = JSON.parse(savedSettings);
        // Merge with defaults to handle new settings added in updates
        setSettings(prev => ({ ...defaultSettings, ...parsed }));
      }
    } catch (error) {
      console.warn('Failed to load settings from localStorage:', error);
    }
  }, []);

  // Save settings to localStorage whenever they change
  useEffect(() => {
    try {
      localStorage.setItem('yieldback-settings', JSON.stringify(settings));
    } catch (error) {
      console.warn('Failed to save settings to localStorage:', error);
    }
  }, [settings]);

  const updateSetting = <K extends keyof AppSettings>(key: K, value: AppSettings[K]) => {
    setSettings(prev => ({
      ...prev,
      [key]: value,
    }));
  };

  const resetSettings = () => {
    setSettings(defaultSettings);
    try {
      localStorage.removeItem('yieldback-settings');
    } catch (error) {
      console.warn('Failed to clear settings from localStorage:', error);
    }
  };

  const exportSettings = (): string => {
    return JSON.stringify(settings, null, 2);
  };

  const importSettings = (settingsJson: string): boolean => {
    try {
      const imported = JSON.parse(settingsJson);
      // Validate that it's a valid settings object
      if (typeof imported === 'object' && imported !== null) {
        setSettings({ ...defaultSettings, ...imported });
        return true;
      }
      return false;
    } catch (error) {
      console.error('Failed to import settings:', error);
      return false;
    }
  };

  return (
    <SettingsContext.Provider value={{ 
      settings, 
      updateSetting, 
      resetSettings, 
      exportSettings, 
      importSettings 
    }}>
      {children}
    </SettingsContext.Provider>
  );
};

export const useSettings = (): SettingsContextType => {
  const context = useContext(SettingsContext);
  if (!context) {
    throw new Error('useSettings must be used within a SettingsProvider');
  }
  return context;
};

// Specialized hooks for common use cases
export const useTheme = () => {
  const { settings, updateSetting } = useSettings();
  return {
    theme: settings.theme,
    isDarkMode: settings.theme === 'dark',
    toggleTheme: () => updateSetting('theme', settings.theme === 'light' ? 'dark' : 'light'),
    setTheme: (theme: 'light' | 'dark') => updateSetting('theme', theme),
  };
};

export const useNetwork = () => {
  const { settings, updateSetting } = useSettings();
  return {
    network: settings.network,
    rpcUrl: settings.customRpcUrl,
    isTestnet: settings.network === 'testnet',
    setNetwork: (network: 'testnet' | 'mainnet') => updateSetting('network', network),
    setRpcUrl: (url: string) => updateSetting('customRpcUrl', url),
    getNetworkConfig: () => ({
      networkPassphrase: settings.network === 'testnet' 
        ? "Test SDF Network ; September 2015" 
        : "Public Global Stellar Network ; September 2015",
      rpcUrl: settings.customRpcUrl,
    }),
  };
};

export const usePositionDefaults = () => {
  const { settings, updateSetting } = useSettings();
  return {
    defaultMaturityDays: settings.defaultMaturityDays,
    defaultCouponAmount: settings.defaultCouponAmount,
    defaultPrincipalAmount: settings.defaultPrincipalAmount,
    setDefaultMaturityDays: (days: number) => updateSetting('defaultMaturityDays', days),
    setDefaultCouponAmount: (amount: string) => updateSetting('defaultCouponAmount', amount),
    setDefaultPrincipalAmount: (amount: string) => updateSetting('defaultPrincipalAmount', amount),
  };
};

export const useTransactionSettings = () => {
  const { settings, updateSetting } = useSettings();
  return {
    defaultFee: settings.defaultFee,
    defaultTimeout: settings.defaultTimeout,
    confirmTransactions: settings.confirmTransactions,
    setDefaultFee: (fee: string) => updateSetting('defaultFee', fee),
    setDefaultTimeout: (timeout: number) => updateSetting('defaultTimeout', timeout),
    setConfirmTransactions: (confirm: boolean) => updateSetting('confirmTransactions', confirm),
  };
};

export const useNotifications = () => {
  const { settings, updateSetting } = useSettings();
  return {
    showSuccess: settings.showSuccessNotifications,
    showWarning: settings.showWarningNotifications,
    showError: settings.showErrorNotifications,
    setShowSuccess: (show: boolean) => updateSetting('showSuccessNotifications', show),
    setShowWarning: (show: boolean) => updateSetting('showWarningNotifications', show),
    setShowError: (show: boolean) => updateSetting('showErrorNotifications', show),
  };
};
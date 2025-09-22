import { Box, Typography, Button, TextField, Card, CardContent, Alert, CircularProgress } from '@mui/material';
import type { NextPage } from 'next';
import { useState, useEffect } from 'react';
import { useWallet } from '../contexts/wallet';
import { Client, networks } from '../index';

interface PositionFormData {
  tokenAddress: string;
  blendPoolAddress: string;
  maturityDays: string;
  couponAmount: string;
  principalAmount: string;
}

const CreatePosition: NextPage = () => {
  const { connected, walletAddress, connect } = useWallet();
  
  const [formData, setFormData] = useState<PositionFormData>({
    tokenAddress: '',
    blendPoolAddress: '',
    maturityDays: '30',
    couponAmount: '1000',
    principalAmount: '10000'
  });
  
  const [isDeploying, setIsDeploying] = useState(false);
  const [deployedAddress, setDeployedAddress] = useState<string>('');
  const [error, setError] = useState<string>('');

  // Update admin when wallet connects
  const [admin, setAdmin] = useState<string>('');
  useEffect(() => {
    if (walletAddress) {
      setAdmin(walletAddress);
    }
  }, [walletAddress]);

  const handleInputChange = (field: keyof PositionFormData) => (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    setFormData(prev => ({
      ...prev,
      [field]: event.target.value
    }));
    // Clear error when user starts typing
    if (error) setError('');
  };

  const validateForm = (): boolean => {
    if (!connected) {
      setError('Please connect your wallet first');
      return false;
    }
    if (!formData.tokenAddress) {
      setError('Token address is required');
      return false;
    }
    if (!formData.blendPoolAddress) {
      setError('Blend pool address is required');
      return false;
    }
    if (!formData.maturityDays || parseInt(formData.maturityDays) <= 0) {
      setError('Valid maturity period is required');
      return false;
    }
    if (!formData.couponAmount || parseFloat(formData.couponAmount) <= 0) {
      setError('Valid coupon amount is required');
      return false;
    }
    if (!formData.principalAmount || parseFloat(formData.principalAmount) <= 0) {
      setError('Valid principal amount is required');
      return false;
    }
    return true;
  };

  const handleCreatePosition = async () => {
    if (!validateForm()) return;
    
    setIsDeploying(true);
    setError('');
    
    try {
      // Calculate maturity timestamp
      const maturityTimestamp = Math.floor(Date.now() / 1000) + (parseInt(formData.maturityDays) * 24 * 60 * 60);
      
      // Deploy the contract
      const result = await Client.deploy({
        admin,
        token_address: formData.tokenAddress,
        blend_pool_address: formData.blendPoolAddress,
        maturity: BigInt(maturityTimestamp),
        coupon_amount: BigInt(Math.floor(parseFloat(formData.couponAmount) * 10**7)),
        principal_amount: BigInt(Math.floor(parseFloat(formData.principalAmount) * 10**7))
      }, {
        ...networks.testnet,
        rpcUrl: 'https://soroban-testnet.stellar.org',
        allowHttp: false,
        fee: "100000",
        timeoutInSeconds: 30,
        wasmHash: "c91b690eeb907251d0375722519466c879ca43f78cb717b4b5a357d8fb75a087"
      });

      if (result.result) {
        setDeployedAddress(result.result.toString());
        // Reset form after successful deployment
        setFormData({
          tokenAddress: '',
          blendPoolAddress: '',
          maturityDays: '30',
          couponAmount: '1000',
          principalAmount: '10000'
        });
      }
    } catch (error) {
      console.error('Position creation failed:', error);
      setError(error instanceof Error ? error.message : 'Failed to create position');
    } finally {
      setIsDeploying(false);
    }
  };

  return (
    <Box sx={{ 
      padding: '24px', 
      maxWidth: '800px', 
      margin: '0 auto',
      minHeight: '100vh',
      backgroundColor: 'background.default'
    }}>
      <Typography variant="h3" component="h1" sx={{ 
        marginBottom: '32px', 
        textAlign: 'center',
        fontWeight: 600
      }}>
        Create New Position
      </Typography>

      <Card elevation={3} sx={{ marginBottom: '24px' }}>
        <CardContent sx={{ padding: '32px' }}>
          {!connected && (
            <Alert 
              severity="warning" 
              sx={{ marginBottom: '24px' }}
              action={
                <Button color="inherit" size="small" onClick={connect}>
                  Connect Wallet
                </Button>
              }
            >
              Please connect your wallet to create a position
            </Alert>
          )}

          {error && (
            <Alert severity="error" sx={{ marginBottom: '24px' }}>
              {error}
            </Alert>
          )}

          <Box sx={{ display: 'flex', flexDirection: 'column', gap: '24px' }}>
            <TextField
              label="Admin Address"
              value={admin}
              onChange={(e) => setAdmin(e.target.value)}
              placeholder="Contract admin address"
              fullWidth
              disabled={!connected}
              helperText="The address that will have admin privileges for this position"
            />

            <TextField
              label="Token Address"
              value={formData.tokenAddress}
              onChange={handleInputChange('tokenAddress')}
              placeholder="Underlying token contract address"
              fullWidth
              disabled={!connected}
              required
              helperText="The Stellar token contract address for this position"
            />

            <TextField
              label="Blend Pool Address"
              value={formData.blendPoolAddress}
              onChange={handleInputChange('blendPoolAddress')}
              placeholder="Blend pool contract address"
              fullWidth
              disabled={!connected}
              required
              helperText="The Blend protocol pool address where funds will be deposited"
            />

            <Box sx={{ display: 'flex', gap: '16px', flexWrap: 'wrap' }}>
              <TextField
                label="Maturity (Days)"
                type="number"
                value={formData.maturityDays}
                onChange={handleInputChange('maturityDays')}
                sx={{ flex: 1, minWidth: '200px' }}
                disabled={!connected}
                required
                inputProps={{ min: 1, max: 365 }}
                helperText="Position duration in days"
              />

              <TextField
                label="Coupon Amount"
                type="number"
                value={formData.couponAmount}
                onChange={handleInputChange('couponAmount')}
                sx={{ flex: 1, minWidth: '200px' }}
                disabled={!connected}
                required
                inputProps={{ min: 0, step: 0.01 }}
                helperText="Interest/coupon payment amount"
              />

              <TextField
                label="Principal Amount"
                type="number"
                value={formData.principalAmount}
                onChange={handleInputChange('principalAmount')}
                sx={{ flex: 1, minWidth: '200px' }}
                disabled={!connected}
                required
                inputProps={{ min: 0, step: 0.01 }}
                helperText="Initial principal investment"
              />
            </Box>

            <Button
              variant="contained"
              size="large"
              onClick={handleCreatePosition}
              disabled={!connected || isDeploying}
              sx={{ 
                padding: '16px 32px',
                fontSize: '18px',
                fontWeight: 600,
                borderRadius: '8px'
              }}
              startIcon={isDeploying ? <CircularProgress size={20} color="inherit" /> : null}
            >
              {isDeploying ? 'Creating Position...' : 'Create Position'}
            </Button>
          </Box>
        </CardContent>
      </Card>

      {deployedAddress && (
        <Card elevation={2}>
          <CardContent>
            <Alert severity="success" sx={{ marginBottom: '16px' }}>
              <Typography variant="h6" component="div" sx={{ marginBottom: '8px' }}>
                Position Created Successfully!
              </Typography>
              <Typography variant="body2">
                Your position contract has been deployed and is ready to use.
              </Typography>
            </Alert>
            
            <Box sx={{ 
              backgroundColor: 'background.paper', 
              padding: '16px', 
              borderRadius: '4px',
              border: '1px solid',
              borderColor: 'success.main'
            }}>
              <Typography variant="body2" color="text.secondary" sx={{ marginBottom: '4px' }}>
                Contract Address:
              </Typography>
              <Typography variant="body1" sx={{ 
                fontFamily: 'monospace', 
                wordBreak: 'break-all',
                fontWeight: 500
              }}>
                {deployedAddress}
              </Typography>
            </Box>

            <Box sx={{ marginTop: '16px', display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
              <Button 
                variant="outlined" 
                size="small"
                onClick={() => {
                  navigator.clipboard.writeText(deployedAddress);
                }}
              >
                Copy Address
              </Button>
              <Button 
                variant="outlined" 
                size="small"
                onClick={() => {
                  // Reset to create another position
                  setDeployedAddress('');
                }}
              >
                Create Another Position
              </Button>
            </Box>
          </CardContent>
        </Card>
      )}
    </Box>
  );
};

export default CreatePosition;
import { Box, Typography, useTheme, Input } from '@mui/material';
import type { NextPage } from 'next';
import { useState } from 'react';
import { useWallet } from '../contexts/wallet';
import { Client, networks } from '../index';

const CreateEscrow: NextPage = () => {
  const theme = useTheme();
  const { connected, walletAddress } = useWallet();
  
  const [admin, setAdmin] = useState<string>(walletAddress || '');
  const [tokenAddress, setTokenAddress] = useState<string>('');
  const [blendPoolAddress, setBlendPoolAddress] = useState<string>('');
  const [maturityDays, setMaturityDays] = useState<string>('30');
  const [couponAmount, setCouponAmount] = useState<string>('1000');
  const [principalAmount, setPrincipalAmount] = useState<string>('10000');
  const [isDeploying, setIsDeploying] = useState(false);
  const [deployedAddress, setDeployedAddress] = useState<string>('');

  const handleDeploy = async () => {
    if (!connected) return;
    
    setIsDeploying(true);
    try {
      const maturityTimestamp = Math.floor(Date.now() / 1000) + (parseInt(maturityDays) * 24 * 60 * 60);
      
      const result = await Client.deploy({
        admin,
        token_address: tokenAddress,
        blend_pool_address: blendPoolAddress,
        maturity: BigInt(maturityTimestamp),
        coupon_amount: BigInt(parseInt(couponAmount) * 10**7),
        principal_amount: BigInt(parseInt(principalAmount) * 10**7)
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
      }
    } catch (error) {
      console.error('Deployment failed:', error);
    } finally {
      setIsDeploying(false);
    }
  };

  return (
    <Box sx={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
      <Typography variant="h2" sx={{ marginBottom: '24px' }}>
        Create Escrow Contract
      </Typography>

      <Box sx={{ display: 'flex', flexDirection: 'column', gap: '16px' }}>
        <Box>
          <Typography variant="body2" sx={{ marginBottom: '8px' }}>
            Admin Address
          </Typography>
          <Input
            fullWidth
            value={admin}
            onChange={(e) => setAdmin(e.target.value)}
            placeholder="Contract admin address"
          />
        </Box>

        <Box>
          <Typography variant="body2" sx={{ marginBottom: '8px' }}>
            Token Address
          </Typography>
          <Input
            fullWidth
            value={tokenAddress}
            onChange={(e) => setTokenAddress(e.target.value)}
            placeholder="Underlying token contract address"
          />
        </Box>

        <Box>
          <Typography variant="body2" sx={{ marginBottom: '8px' }}>
            Blend Pool Address
          </Typography>
          <Input
            fullWidth
            value={blendPoolAddress}
            onChange={(e) => setBlendPoolAddress(e.target.value)}
            placeholder="Blend pool contract address"
          />
        </Box>

        <Box>
          <Typography variant="body2" sx={{ marginBottom: '8px' }}>
            Maturity (Days)
          </Typography>
          <Input
            fullWidth
            type="number"
            value={maturityDays}
            onChange={(e) => setMaturityDays(e.target.value)}
            placeholder="30"
          />
        </Box>

        <Box>
          <Typography variant="body2" sx={{ marginBottom: '8px' }}>
            Coupon Amount
          </Typography>
          <Input
            fullWidth
            type="number"
            value={couponAmount}
            onChange={(e) => setCouponAmount(e.target.value)}
            placeholder="1000"
          />
        </Box>

        <Box>
          <Typography variant="body2" sx={{ marginBottom: '8px' }}>
            Principal Amount
          </Typography>
          <Input
            fullWidth
            type="number"
            value={principalAmount}
            onChange={(e) => setPrincipalAmount(e.target.value)}
            placeholder="10000"
          />
        </Box>

        <button
          onClick={handleDeploy}
          disabled={!connected || isDeploying || !tokenAddress || !blendPoolAddress}
          style={{
            padding: '12px 24px',
            backgroundColor: connected ? theme.palette.primary.main : theme.palette.grey[500],
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: connected ? 'pointer' : 'not-allowed',
            fontSize: '16px'
          }}
        >
          {isDeploying ? 'Deploying...' : 'Deploy Contract'}
        </button>

        {deployedAddress && (
          <Box sx={{ marginTop: '16px', padding: '12px', backgroundColor: theme.palette.success.light }}>
            <Typography variant="body2">
              Contract deployed at: {deployedAddress}
            </Typography>
          </Box>
        )}

        {!connected && (
          <Typography variant="body2" color="warning.main">
            Please connect your wallet to deploy a contract
          </Typography>
        )}
      </Box>
    </Box>
  );
};

export default CreateEscrow;
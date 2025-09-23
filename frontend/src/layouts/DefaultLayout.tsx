import React from 'react';
import { Box, AppBar, Toolbar, Typography, Button, Container } from '@mui/material';
import { useWallet } from '../contexts/wallet';

interface DefaultLayoutProps {
  children: React.ReactNode;
}

const DefaultLayout: React.FC<DefaultLayoutProps> = ({ children }) => {
  const { connected, walletAddress, connect, disconnect } = useWallet();

  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar position="static">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            YieldBack
          </Typography>
          {connected ? (
            <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
              <Typography variant="body2">
                {`${walletAddress.slice(0, 6)}...${walletAddress.slice(-4)}`}
              </Typography>
              <Button color="inherit" onClick={disconnect}>
                Disconnect
              </Button>
            </Box>
          ) : (
            <Button color="inherit" onClick={connect}>
              Connect Wallet
            </Button>
          )}
        </Toolbar>
      </AppBar>
      <Container maxWidth="lg" sx={{ mt: 4, mb: 4 }}>
        {children}
      </Container>
    </Box>
  );
};

export default DefaultLayout;
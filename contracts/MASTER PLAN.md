Factory for yield markets
once maturity is reached, new yield manager contract is deployed
current active yield manager/market is selected to the new one. so the front end queries the factory contract for the current yield manager contract to use

effectively, every ~120 days for example a new yield manager contract is deployed.
the alternative to this system would be to let users create their own markets as they choose with their own maturities. I dont know how that would work that would be kind of hard


AMM 
since we know exactly what the price of 1 PT will be at maturity (=1) and 1 YT will be worth 0 at maturity, closer to maturity capital efficiency would be really bad

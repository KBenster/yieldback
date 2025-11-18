Yield Redemption for yield tokens

each user has
    index: the exchange rate when they last claimed/interacted
    accrued: the amount of yield they've earned but not yet claimed

whenever a user interacts (transfers, claims, etc)
    gets current exchange rate
    calculates yield earned since their last interaction (userYTBalance × (currentIndex - userIndex)) / (userIndex × currentIndex)
    adds this to their accrued balance
    updates their index to current exchange rate
their interest needs to be updated during mints, burns, token transfers, redeeming PT and YT

User claiming process
    First update distribution, like calculate all unclaimed interest
    calculate amount / gets users accrued balance
    (this is where you would apply a fee)
    transfer vault shares back to user


Yield redemption for principal tokens
    After maturity, PT is redeemable
    the redemption ratio is locked at the exchange rate at expiry.
    
PRO RATA DISTRIBUTION!!
    
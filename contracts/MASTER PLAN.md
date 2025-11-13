Yield Manager
user deposits, 100 PT 100 YT get minted to user (yield manager will be the token contract admin)

YT contract
Each user, in the YT contract, has their last exchange rate stored.
source of trush for this exchange rate is in the vault contract
a stored value is in the YT contract for each user

TODO: clean up functions in the yield manager contract, there should not be this many
yield manager needs to access and cache 
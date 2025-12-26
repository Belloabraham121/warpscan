# Deployment Information

## Deployed Contract

**Contract Address:** `0x5FbDB2315678afecb367f032d93F642f64180aa3`

**Contract Name:** SimpleStorage

**Initial Value:** 42

## View in Warpscan

1. Make sure Warpscan is running and connected to the local Anvil node
2. Search for the contract address: `0x5FbDB2315678afecb367f032d93F642f64180aa3`
3. You should see:
   - Contract creation transaction
   - Contract code
   - Contract storage (initial value of 42)

## Test the Contract

You can interact with the contract using Foundry:

```bash
# Get the stored value
cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "getValue()(uint256)" --rpc-url http://127.0.0.1:8545

# Set a new value (using the first Anvil account)
cast send 0x5FbDB2315678afecb367f032d93F642f64180aa3 "setValue(uint256)" 100 --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://127.0.0.1:8545
```

## Anvil Accounts

The first Anvil account (used for deployment):

- **Address:** `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`
- **Private Key:** `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`
- **Balance:** 10000 ETH

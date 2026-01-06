# Foundry Test Project for Warpscan

This Foundry project is used to deploy smart contracts to the local Anvil node for testing Warpscan.

## Prerequisites

1. Anvil must be running on `http://127.0.0.1:8545`
2. Foundry is already installed at `~/.foundry/bin/forge`

## Setup

1. Make sure Anvil is running:
   ```bash
   anvil
   ```

2. The `.env` file is already configured with the first Anvil account private key:
   - Address: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`
   - Private Key: `0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80`

## Deploy the Contract

To deploy the SimpleStorage contract:

```bash
forge script script/DeploySimpleStorage.s.sol:DeploySimpleStorage --rpc-url http://127.0.0.1:8545 --broadcast
```

## Call Contract Functions

### Option 1: Using Foundry Script

To call the `setValue` function on the deployed contract:

```bash
forge script script/CallSimpleStorage.s.sol:CallSimpleStorage --rpc-url http://127.0.0.1:8545 --broadcast
```

This will:
- Get the current value (should be 42 initially)
- Set a new value to 100
- Verify the value was updated

### Option 2: Using Cast Commands

You can also use `cast` commands directly:

```bash
# Get current value
cast call 0x5FbDB2315678afecb367f032d93F642f64180aa3 "getValue()(uint256)" --rpc-url http://127.0.0.1:8545

# Set a new value
cast send 0x5FbDB2315678afecb367f032d93F642f64180aa3 "setValue(uint256)" 999 \
    --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 \
    --rpc-url http://127.0.0.1:8545
```

### Option 3: Using the Shell Script

```bash
./scripts/call_contract.sh
```

## View in Warpscan

After deployment or function calls, you can:
1. Open Warpscan and select "Local Node" mode
2. Search for the contract address: `0x5FbDB2315678afecb367f032d93F642f64180aa3`
3. View:
   - Contract creation transaction
   - Function call transactions (setValue calls)
   - Contract code
   - Latest transactions list showing all interactions

## Contracts

### SimpleStorage

A simple storage contract that stores a uint256 value. It has:
- `getValue()` - Returns the stored value (view function, no gas cost)
- `setValue(uint256)` - Updates the stored value (state-changing function, costs gas)
- Events emitted on value changes

**Deployed Address:** `0x5FbDB2315678afecb367f032d93F642f64180aa3`

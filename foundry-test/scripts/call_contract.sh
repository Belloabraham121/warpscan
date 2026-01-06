#!/bin/bash

# Script to call SimpleStorage contract functions
# Make sure Anvil is running before executing this script

CONTRACT_ADDRESS="0x5FbDB2315678afecb367f032d93F642f64180aa3"
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
RPC_URL="http://127.0.0.1:8545"

echo "=== Calling SimpleStorage Contract ==="
echo "Contract Address: $CONTRACT_ADDRESS"
echo ""

# Get current value
echo "1. Getting current value..."
cast call $CONTRACT_ADDRESS "getValue()(uint256)" --rpc-url $RPC_URL

# Set a new value
echo ""
echo "2. Setting new value to 999..."
cast send $CONTRACT_ADDRESS "setValue(uint256)" 999 \
    --private-key $PRIVATE_KEY \
    --rpc-url $RPC_URL

# Get the updated value
echo ""
echo "3. Getting updated value..."
cast call $CONTRACT_ADDRESS "getValue()(uint256)" --rpc-url $RPC_URL

echo ""
echo "=== Transaction complete! Check Warpscan for the transaction ==="


#!/usr/bin/env bash
set -euo pipefail

# Helper to start an Anvil mainnet fork for local testing
# Requirements:
# - Foundry anvil installed: curl -L https://foundry.paradigm.xyz | bash
#
# Usage:
#   ETH_RPC_URL=https://mainnet.infura.io/v3/KEY ./scripts/anvil-mainnet-fork.sh
#   ETH_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/KEY FORK_BLOCK=21000000 ./scripts/anvil-mainnet-fork.sh
#
# Env vars:
# - ETH_RPC_URL (required): Upstream mainnet RPC to fork from
# - FORK_BLOCK (optional):  Block number to pin the fork at (default: latest)
# - ANVIL_PORT (optional):  Local port to bind (default: 8545)
# - ANVIL_CHAIN_ID (optional): Chain id to use (default: 1)
#
# Notes:
# - Starts anvil with useful dev flags for tracing and determinism

if [[ -z "${ETH_RPC_URL:-}" ]]; then
  echo "ERROR: ETH_RPC_URL must be set to an upstream mainnet RPC (Infura/Alchemy/etc.)" >&2
  exit 1
fi

ANVIL_PORT="${ANVIL_PORT:-8545}"
ANVIL_CHAIN_ID="${ANVIL_CHAIN_ID:-1}"
FORK_BLOCK_ARG=()
if [[ -n "${FORK_BLOCK:-}" ]]; then
  FORK_BLOCK_ARG=(--fork-block-number "$FORK_BLOCK")
fi

echo "Starting anvil mainnet fork on http://127.0.0.1:${ANVIL_PORT}"
echo "Upstream: ${ETH_RPC_URL}"
if [[ -n "${FORK_BLOCK:-}" ]]; then
  echo "Fork block: ${FORK_BLOCK}"
else
  echo "Fork block: latest"
fi

exec anvil \
  --port "${ANVIL_PORT}" \
  --chain-id "${ANVIL_CHAIN_ID}" \
  --fork-url "${ETH_RPC_URL}" \
  "${FORK_BLOCK_ARG[@]}" \
  --steps-tracing \
  --no-storage-caching \
  --silent



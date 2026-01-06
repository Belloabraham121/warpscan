//! Local node detection for Anvil and Hardhat
//!
//! This module provides functionality to automatically detect and configure
//! local development nodes (Anvil/Hardhat) running on common ports.

use crate::error::{Error, Result};
use ethers::providers::{Http, Middleware, Provider};
use std::time::Duration;

/// Common RPC ports for local development nodes
#[allow(dead_code)]
const ANVIL_DEFAULT_PORT: u16 = 8545;
#[allow(dead_code)]
const HARDHAT_DEFAULT_PORT: u16 = 8545;
const LOCALHOST_URLS: &[&str] = &[
    "http://127.0.0.1:8545",
    "http://localhost:8545",
    "http://127.0.0.1:8546",
    "http://localhost:8546",
];

/// Detected node information
#[derive(Debug, Clone)]
pub struct DetectedNode {
    /// RPC URL of the detected node
    pub rpc_url: String,
    /// Node type (anvil, hardhat, or unknown)
    pub node_type: String,
    /// Chain ID of the detected network
    pub chain_id: u64,
    /// Network name
    pub network_name: String,
}

/// Auto-detect local development nodes (Anvil/Hardhat)
pub async fn detect_local_node() -> Result<Option<DetectedNode>> {
    // Try each common localhost URL
    for url in LOCALHOST_URLS {
        tracing::debug!(target: "warpscan", "Trying to detect local node at {}...", url);

        match test_node(url).await {
            Ok(node) => {
                tracing::info!(
                    target: "warpscan",
                    "Found local node at {}: {} (Chain ID: {})",
                    url,
                    node.node_type,
                    node.chain_id
                );
                return Ok(Some(node));
            }
            Err(e) => {
                tracing::debug!(target: "warpscan", "No node at {}: {}", url, e);
            }
        }
    }

    tracing::debug!(target: "warpscan", "No local nodes found on common ports");
    Ok(None)
}

/// Test if a node is available and determine its type
async fn test_node(url: &str) -> Result<DetectedNode> {
    // Create a provider with a short timeout
    let provider = Provider::<Http>::try_from(url)
        .map_err(|e| Error::network(format!("Invalid RPC URL {}: {}", url, e)))?;

    // Test connection with timeout (increased to 5 seconds for reliability)
    let chain_id_result =
        tokio::time::timeout(Duration::from_secs(5), provider.get_chainid()).await;

    let chain_id = match chain_id_result {
        Ok(Ok(id)) => id.as_u64(),
        Ok(Err(e)) => {
            return Err(Error::network(format!(
                "Failed to get chain ID from {}: {}",
                url, e
            )));
        }
        Err(_) => {
            return Err(Error::network(format!("Timeout connecting to {}", url)));
        }
    };

    // Try to determine node type by checking specific RPC methods
    let node_type = determine_node_type(&provider, chain_id).await;

    // Generate network name based on chain ID and node type
    let network_name = match (chain_id, node_type.as_str()) {
        (31337, "anvil") => "Anvil Local".to_string(),
        (31337, "hardhat") => "Hardhat Local".to_string(),
        (1337, _) => format!("{} Local (Chain 1337)", node_type),
        (id, _) => format!("{} (Chain {})", node_type, id),
    };

    Ok(DetectedNode {
        rpc_url: url.to_string(),
        node_type,
        chain_id,
        network_name,
    })
}

/// Determine the node type by checking RPC capabilities
async fn determine_node_type(provider: &Provider<Http>, chain_id: u64) -> String {
    // Anvil and Hardhat both typically use chain ID 31337 for local networks
    // We can try to distinguish by checking for Anvil-specific features
    // For now, we'll use chain ID as a hint and default to "anvil" for 31337

    // Try to get block number to verify it's working
    if provider.get_block_number().await.is_ok() {
        // Check if it's likely Anvil (chain ID 31337 is common for Anvil)
        if chain_id == 31337 {
            // Try to detect Anvil by checking for specific behavior
            // Anvil often has predictable account behavior
            return "anvil".to_string();
        }

        // Hardhat also uses 31337, but we can check other indicators
        // For now, if chain_id is 31337 and we can't determine, default to "anvil"
        if chain_id == 31337 {
            return "anvil".to_string();
        }

        // Other local networks
        if chain_id == 1337 {
            return "hardhat".to_string();
        }

        return "local".to_string();
    }

    "unknown".to_string()
}

/// Check if a URL points to a local node
pub fn is_local_url(url: &str) -> bool {
    url.starts_with("http://127.0.0.1") || url.starts_with("http://localhost")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_local_url() {
        assert!(is_local_url("http://127.0.0.1:8545"));
        assert!(is_local_url("http://localhost:8545"));
        assert!(!is_local_url("https://mainnet.infura.io/v3/abc"));
        assert!(!is_local_url("https://eth-mainnet.alchemyapi.io/v2/xyz"));
    }
}

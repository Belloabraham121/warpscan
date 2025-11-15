//! Configuration data types and structures
//!
//! This module defines the data structures used for application configuration.

use serde::{Deserialize, Serialize};

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Network configuration
    pub network: NetworkConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// UI configuration
    pub ui: UiConfig,
    /// Gas tracking configuration
    pub gas: GasConfig,
    /// Optional Etherscan API key (overrides env if set in file)
    pub etherscan_api_key: Option<String>,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Network name
    pub name: String,
    /// RPC URL for Ethereum node
    pub rpc_url: String,
    /// Chain ID (1 for mainnet, 11155111 for sepolia)
    pub chain_id: u64,
    /// Request timeout in seconds
    pub timeout_seconds: u64,
    /// Preferred node type (anvil, hardhat, infura, alchemy, custom)
    pub node_type: Option<String>,
}

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Maximum cache size in MB
    pub max_size_mb: u64,
    /// Time-to-live for cache entries in seconds
    pub ttl_seconds: u64,
    /// Block cache TTL in seconds
    pub block_ttl_seconds: u64,
    /// Transaction cache TTL in seconds
    pub transaction_ttl_seconds: u64,
    /// Address cache TTL in seconds
    pub address_ttl_seconds: u64,
    /// Contract cache TTL in seconds
    pub contract_ttl_seconds: u64,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// UI theme (dark, light, ethereum)
    pub theme: String,
    /// Refresh interval in milliseconds
    pub refresh_interval_ms: u64,
    /// Maximum results per page
    pub max_results_per_page: usize,
    /// Log level for the application
    pub log_level: String,
}

/// Gas tracking configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasConfig {
    /// Update interval for gas prices in seconds
    pub update_interval_seconds: u64,
    /// Number of days to keep gas price history
    pub history_days: u32,
}

/// Supported Ethereum networks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Goerli,
    Sepolia,
    Custom(String),
}

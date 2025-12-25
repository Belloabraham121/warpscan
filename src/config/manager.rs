//! Configuration manager implementation
//!
//! This module provides functionality for loading, saving, and validating
//! application configuration.

use super::node_detection;
use super::types::{CacheConfig, Config, GasConfig, NetworkConfig, UiConfig};
use crate::error::{Error, Result};
use dotenvy::dotenv;
use std::path::PathBuf;

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                name: "Ethereum Mainnet".to_string(),
                rpc_url: "https://eth-sepolia.g.alchemy.com/v2/fvAedZJFIuaMcbNp1wSLQ".to_string(),
                chain_id: 1,
                timeout_seconds: 30,
                node_type: Some("custom".to_string()),
            },
            cache: CacheConfig {
                enabled: true,
                max_size_mb: 100,
                ttl_seconds: 3600,
                block_ttl_seconds: 3600,
                transaction_ttl_seconds: 7200,
                address_ttl_seconds: 1800,
                contract_ttl_seconds: 86400,
                // Long TTL for static data (transactions don't change once confirmed)
                address_transactions_ttl_seconds: 3600, // 1 hour
                token_transfers_ttl_seconds: 3600,      // 1 hour
                // Short TTL for dynamic data (balances change frequently)
                token_balances_ttl_seconds: 300,         // 5 minutes
                internal_transactions_ttl_seconds: 3600, // 1 hour
                // Very long TTL for ENS (rarely changes)
                ens_names_ttl_seconds: 86400, // 24 hours
            },
            ui: UiConfig {
                theme: "dark".to_string(),
                refresh_interval_ms: 5000,
                max_results_per_page: 20,
                log_level: "info".to_string(),
            },
            gas: GasConfig {
                update_interval_seconds: 15,
                history_days: 7,
            },
            etherscan_api_key: std::env::var("ETHERSCAN_API_KEY").ok(),
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        // Load environment from .env if present
        let _ = dotenv();
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            let mut config: Config = toml::from_str(&config_str)
                .map_err(|e| Error::parse(format!("Failed to parse config: {}", e)))?;
            // Prefer environment variable over file value if present
            if let Ok(key) = std::env::var("ETHERSCAN_API_KEY") {
                config.etherscan_api_key = Some(key);
            }
            Ok(config)
        } else {
            let default_config = Config::default();
            default_config.save()?;
            Ok(default_config)
        }
    }

    /// Load configuration and auto-detect local nodes
    pub async fn load_with_auto_detect() -> Result<Self> {
        let mut config = Self::load()?;

        // Auto-detect local nodes if current RPC is not local
        if !node_detection::is_local_url(&config.network.rpc_url) {
            tracing::info!(
                target: "warpscan",
                "Current RPC is not local ({}), attempting auto-detection...",
                config.network.rpc_url
            );
            
            match node_detection::detect_local_node().await {
                Ok(Some(detected)) => {
                    // Update config with detected node
                    config.network.rpc_url = detected.rpc_url.clone();
                    config.network.chain_id = detected.chain_id;
                    config.network.name = detected.network_name.clone();
                    config.network.node_type = Some(detected.node_type.clone());
                    
                    tracing::info!(
                        target: "warpscan",
                        "Auto-detected local node: {} at {} (Chain ID: {})",
                        detected.node_type,
                        detected.rpc_url,
                        detected.chain_id
                    );
                }
                Ok(None) => {
                    tracing::warn!(
                        target: "warpscan",
                        "No local node detected, using configured RPC: {}",
                        config.network.rpc_url
                    );
                }
                Err(e) => {
                    tracing::warn!(
                        target: "warpscan",
                        "Error during local node detection: {}. Using configured RPC: {}",
                        e,
                        config.network.rpc_url
                    );
                }
            }
        } else {
            tracing::info!(
                target: "warpscan",
                "Already using local RPC: {}",
                config.network.rpc_url
            );
        }

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let config_str = toml::to_string_pretty(self)
            .map_err(|e| Error::parse(format!("Failed to serialize config: {}", e)))?;

        std::fs::write(&config_path, config_str)?;
        Ok(())
    }

    /// Get the configuration file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| Error::app("Could not determine config directory"))?;

        Ok(config_dir.join("warpscan").join("config.toml"))
    }

    /// Get the cache directory path
    pub fn cache_dir() -> Result<PathBuf> {
        let cache_dir =
            dirs::cache_dir().ok_or_else(|| Error::app("Could not determine cache directory"))?;

        Ok(cache_dir.join("warpscan"))
    }

    /// Get the logs directory path
    pub fn logs_dir() -> Result<PathBuf> {
        let cache_dir = Self::cache_dir()?;
        Ok(cache_dir.join("logs"))
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        // Validate RPC URL
        if self.network.rpc_url.is_empty() {
            return Err(Error::validation("RPC URL cannot be empty"));
        }

        // Validate chain ID
        if self.network.chain_id == 0 {
            return Err(Error::validation("Chain ID must be greater than 0"));
        }

        // Validate timeout
        if self.network.timeout_seconds == 0 {
            return Err(Error::validation("Timeout must be greater than 0"));
        }

        // Validate cache size
        if self.cache.max_size_mb == 0 {
            return Err(Error::validation("Cache size must be greater than 0"));
        }

        // Validate TTL
        if self.cache.ttl_seconds == 0 {
            return Err(Error::validation("Cache TTL must be greater than 0"));
        }

        Ok(())
    }
}

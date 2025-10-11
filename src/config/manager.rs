//! Configuration manager implementation
//!
//! This module provides functionality for loading, saving, and validating
//! application configuration.

use std::path::PathBuf;
use crate::error::{Error, Result};
use super::types::{Config, NetworkConfig, CacheConfig, UiConfig, GasConfig};
use dotenvy::dotenv;

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
        let config_dir = dirs::config_dir()
            .ok_or_else(|| Error::app("Could not determine config directory"))?;
        
        Ok(config_dir.join("warpscan").join("config.toml"))
    }

    /// Get the cache directory path
    pub fn cache_dir() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| Error::app("Could not determine cache directory"))?;
        
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
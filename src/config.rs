//! Configuration management for WarpScan
//!
//! This module handles loading and managing application configuration
//! from TOML files and environment variables.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::{Error, Result};

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

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                name: "Ethereum Mainnet".to_string(),
                rpc_url: "https://eth-sepolia.g.alchemy.com/v2/fvAedZJFIuaMcbNp1wSLQ".to_string(),
                chain_id: 1,
                timeout_seconds: 30,
            },
            cache: CacheConfig {
                enabled: true,
                max_size_mb: 100,
                ttl_seconds: 3600,
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
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if config_path.exists() {
            let config_str = std::fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&config_str)
                .map_err(|e| Error::parse(format!("Failed to parse config: {}", e)))?;
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
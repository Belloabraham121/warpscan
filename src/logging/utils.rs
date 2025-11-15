//! Logging utility functions
//!
//! This module provides utility functions for logging application events and errors.

use crate::config::Config;

/// Log application startup information
pub fn log_startup_info() {
    tracing::info!("WarpScan Terminal Etherscan starting up");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    tracing::debug!("Rust version: {}", option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown"));
    tracing::debug!("Target: {}", std::env::consts::ARCH);
}

/// Log application shutdown information
pub fn log_shutdown_info() {
    tracing::info!("WarpScan shutting down gracefully");
}

/// Log configuration information
pub fn log_config_info(config: &Config) {
    tracing::info!("Configuration loaded successfully");
    tracing::debug!("Network: {:?}", config.network.name);
    tracing::debug!("RPC URL: {}", config.network.rpc_url);
    if let Some(node_type) = &config.network.node_type {
        tracing::debug!("Node type: {}", node_type);
    }
    tracing::debug!("Cache enabled: {}", config.cache.enabled);
    tracing::debug!("Cache TTL: {}s", config.cache.ttl_seconds);
    tracing::debug!("Theme: {}", config.ui.theme);
    // Log whether an Etherscan API key is configured without revealing it
    let has_key = config.etherscan_api_key.as_ref().map(|k| !k.is_empty()).unwrap_or(false);
    tracing::debug!("Etherscan API key configured: {}", has_key);
}

/// Log error with context
pub fn log_error_with_context(error: &dyn std::error::Error, context: &str) {
    tracing::error!("Error in {}: {}", context, error);
    
    let mut source = error.source();
    let mut level = 1;
    while let Some(err) = source {
        tracing::error!("  Caused by (level {}): {}", level, err);
        source = err.source();
        level += 1;
    }
}
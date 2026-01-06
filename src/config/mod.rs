//! Configuration management for WarpScan
//!
//! This module handles loading and managing application configuration
//! from TOML files and environment variables.

pub mod manager;
pub mod node_detection;
pub mod types;

// Re-export commonly used types and structs
pub use types::{CacheConfig, Config, GasConfig, Network, NetworkConfig, UiConfig};

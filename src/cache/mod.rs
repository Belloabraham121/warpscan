//! Caching system for WarpScan
//!
//! This module provides caching functionality for blockchain data
//! to improve performance and reduce API calls.

pub mod manager;
pub mod types;

// Re-export commonly used types and structs
pub use manager::CacheManager;
pub use types::{AddressInfo, CacheEntry, CacheStats, ContractInfo, TokenInfo};

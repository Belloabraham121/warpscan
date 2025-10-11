//! Caching system for WarpScan
//!
//! This module provides caching functionality for blockchain data
//! to improve performance and reduce API calls.

pub mod types;
pub mod manager;

// Re-export commonly used types and structs
pub use types::{CacheEntry, AddressInfo, ContractInfo, TokenInfo, CacheStats};
pub use manager::CacheManager;
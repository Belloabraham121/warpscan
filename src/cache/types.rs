//! Cache data types and structures
//!
//! This module defines the data structures used for caching blockchain data.

use serde::{Deserialize, Serialize};

/// Cache entry with timestamp for TTL management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub data: T,
    pub timestamp: u64,
    pub ttl_seconds: u64,
}

/// Address information for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: String,
    pub balance: String, // Using String to avoid U256 serialization issues
    pub transaction_count: u64,
    pub is_contract: bool,
    pub last_updated: u64,
}

/// Contract information for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub address: String,
    pub name: Option<String>,
    pub source_code: Option<String>,
    pub abi: Option<String>,
    pub compiler_version: Option<String>,
    pub is_verified: bool,
    pub last_updated: u64,
}

/// Token information for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Option<String>,
    pub last_updated: u64,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub blocks_count: usize,
    pub transactions_count: usize,
    pub addresses_count: usize,
    pub contracts_count: usize,
    pub tokens_count: usize,
    pub total_entries: usize,
}
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

/// Serializable token transfer for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTokenTransfer {
    pub token_id: Option<String>,
    pub txn_hash: String,
    pub from: String,
    pub to: String,
    pub token_name: String,
    pub token_symbol: String,
    pub amount: f64,
    pub timestamp: u64,
}

/// Serializable internal transaction for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableInternalTransaction {
    pub parent_tx_hash: String,
    pub block: u64,
    pub from: String,
    pub to: String,
    pub value: f64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub tx_type: String,
    pub timestamp: u64,
}

/// Serializable token balance for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTokenBalance {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub balance: f64,
}

/// Serializable address transaction for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableAddressTx {
    pub tx_hash: String,
    pub method: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub from: String,
    pub to: String,
    pub value_eth: f64,
    pub fee_eth: f64,
    pub status: String, // "Pending", "Success", "Failed", "Unknown"
}

/// Address transactions list for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedAddressTransactions {
    pub address: String,
    pub transactions: Vec<SerializableAddressTx>,
    pub last_updated: u64,
}

/// Token transfers list for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTokenTransfers {
    pub address: String,
    pub transfers: Vec<SerializableTokenTransfer>,
    pub last_updated: u64,
}

/// Token balances list for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTokenBalances {
    pub address: String,
    pub balances: Vec<SerializableTokenBalance>,
    pub last_updated: u64,
}

/// Internal transactions list for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedInternalTransactions {
    pub address: String,
    pub transactions: Vec<SerializableInternalTransaction>,
    pub last_updated: u64,
}

/// ENS name mapping for caching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedEnsName {
    pub address: String,
    pub ens_name: Option<String>,
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
    pub address_transactions_count: usize,
    pub token_transfers_count: usize,
    pub token_balances_count: usize,
    pub internal_transactions_count: usize,
    pub ens_names_count: usize,
    pub total_entries: usize,
}

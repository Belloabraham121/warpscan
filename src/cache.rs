//! Caching system for WarpScan
//!
//! This module provides caching functionality for blockchain data
//! to improve performance and reduce API calls.

use lru::LruCache;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use ethers::types::{Block, Transaction, H256};
use crate::error::{ Result};
use crate::config::Config;

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

/// Main cache manager
#[derive(Clone)]
pub struct CacheManager {
    /// In-memory LRU caches
    blocks: Arc<Mutex<LruCache<u64, CacheEntry<Block<H256>>>>>,
    transactions: Arc<Mutex<LruCache<String, CacheEntry<Transaction>>>>,
    addresses: Arc<Mutex<LruCache<String, CacheEntry<AddressInfo>>>>,
    contracts: Arc<Mutex<LruCache<String, CacheEntry<ContractInfo>>>>,
    tokens: Arc<Mutex<LruCache<String, CacheEntry<TokenInfo>>>>,
    
    /// Configuration
    config: Config,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(config: Config) -> Result<Self> {
        let cache_dir = Config::cache_dir()?;
        std::fs::create_dir_all(&cache_dir)?;
        
        let cache_size = NonZeroUsize::new(1000).unwrap(); // Default cache size
        
        Ok(Self {
            blocks: Arc::new(Mutex::new(LruCache::new(cache_size))),
            transactions: Arc::new(Mutex::new(LruCache::new(cache_size))),
            addresses: Arc::new(Mutex::new(LruCache::new(cache_size))),
            contracts: Arc::new(Mutex::new(LruCache::new(cache_size))),
            tokens: Arc::new(Mutex::new(LruCache::new(cache_size))),
            config,
        })
    }

    /// Get block from cache
    pub fn get_block(&self, block_number: u64) -> Option<Block<H256>> {
        if !self.config.cache.enabled {
            return None;
        }
        
        let mut cache = self.blocks.lock().unwrap();
        if let Some(entry) = cache.get(&block_number) {
            if !self.is_expired(entry) {
                return Some(entry.data.clone());
            } else {
                cache.pop(&block_number);
            }
        }
        None
    }

    /// Store block in cache
    pub fn store_block(&self, block_number: u64, block: Block<H256>) {
        if !self.config.cache.enabled {
            return;
        }
        
        let entry = CacheEntry {
            data: block,
            timestamp: chrono::Utc::now().timestamp() as u64,
            ttl_seconds: self.config.cache.ttl_seconds,
        };
        
        let mut cache = self.blocks.lock().unwrap();
        cache.put(block_number, entry);
    }

    /// Get transaction from cache
    pub fn get_transaction(&self, tx_hash: &str) -> Option<Transaction> {
        if !self.config.cache.enabled {
            return None;
        }
        
        let mut cache = self.transactions.lock().unwrap();
        if let Some(entry) = cache.get(tx_hash) {
            if !self.is_expired(entry) {
                return Some(entry.data.clone());
            } else {
                cache.pop(tx_hash);
            }
        }
        None
    }

    /// Store transaction in cache
    pub fn store_transaction(&self, tx_hash: String, transaction: Transaction) {
        if !self.config.cache.enabled {
            return;
        }
        
        let entry = CacheEntry {
            data: transaction,
            timestamp: chrono::Utc::now().timestamp() as u64,
            ttl_seconds: self.config.cache.ttl_seconds,
        };
        
        let mut cache = self.transactions.lock().unwrap();
        cache.put(tx_hash, entry);
    }

    /// Get address info from cache
    pub fn get_address_info(&self, address: &str) -> Option<AddressInfo> {
        if !self.config.cache.enabled {
            return None;
        }
        
        let mut cache = self.addresses.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                return Some(entry.data.clone());
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store address info in cache
    pub fn store_address_info(&self, address: String, info: AddressInfo) {
        if !self.config.cache.enabled {
            return;
        }
        
        let entry = CacheEntry {
            data: info,
            timestamp: chrono::Utc::now().timestamp() as u64,
            ttl_seconds: self.config.cache.ttl_seconds,
        };
        
        let mut cache = self.addresses.lock().unwrap();
        cache.put(address, entry);
    }

    /// Get contract info from cache
    pub fn get_contract_info(&self, address: &str) -> Option<ContractInfo> {
        if !self.config.cache.enabled {
            return None;
        }
        
        let mut cache = self.contracts.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                return Some(entry.data.clone());
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store contract info in cache
    pub fn store_contract_info(&self, address: String, info: ContractInfo) {
        if !self.config.cache.enabled {
            return;
        }
        
        let entry = CacheEntry {
            data: info,
            timestamp: chrono::Utc::now().timestamp() as u64,
            ttl_seconds: self.config.cache.ttl_seconds,
        };
        
        let mut cache = self.contracts.lock().unwrap();
        cache.put(address, entry);
    }

    /// Check if cache entry is expired
    fn is_expired<T>(&self, entry: &CacheEntry<T>) -> bool {
        let now = chrono::Utc::now().timestamp() as u64;
        now > entry.timestamp + entry.ttl_seconds
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.blocks.lock().unwrap().clear();
        self.transactions.lock().unwrap().clear();
        self.addresses.lock().unwrap().clear();
        self.contracts.lock().unwrap().clear();
        self.tokens.lock().unwrap().clear();
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let blocks = self.blocks.lock().unwrap();
        let transactions = self.transactions.lock().unwrap();
        let addresses = self.addresses.lock().unwrap();
        let contracts = self.contracts.lock().unwrap();
        let tokens = self.tokens.lock().unwrap();
        
        CacheStats {
            blocks_count: blocks.len(),
            transactions_count: transactions.len(),
            addresses_count: addresses.len(),
            contracts_count: contracts.len(),
            tokens_count: tokens.len(),
            total_entries: blocks.len() + transactions.len() + addresses.len() + contracts.len() + tokens.len(),
        }
    }
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
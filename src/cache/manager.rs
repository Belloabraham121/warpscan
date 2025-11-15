//! Cache manager implementation
//!
//! This module provides the main cache manager for storing and retrieving
//! blockchain data with TTL support.

use super::types::{AddressInfo, CacheEntry, CacheStats, ContractInfo, TokenInfo};
use crate::config::Config;
use crate::error::Result;
use ethers::types::{Block, Transaction, H256};
use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.block_ttl_seconds,
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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.transaction_ttl_seconds,
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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.address_ttl_seconds,
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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.contract_ttl_seconds,
        };

        let mut cache = self.contracts.lock().unwrap();
        cache.put(address, entry);
    }

    /// Check if cache entry is expired
    fn is_expired<T>(&self, entry: &CacheEntry<T>) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
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
        let blocks_count = self.blocks.lock().unwrap().len();
        let transactions_count = self.transactions.lock().unwrap().len();
        let addresses_count = self.addresses.lock().unwrap().len();
        let contracts_count = self.contracts.lock().unwrap().len();
        let tokens_count = self.tokens.lock().unwrap().len();

        CacheStats {
            blocks_count,
            transactions_count,
            addresses_count,
            contracts_count,
            tokens_count,
            total_entries: blocks_count
                + transactions_count
                + addresses_count
                + contracts_count
                + tokens_count,
        }
    }
}

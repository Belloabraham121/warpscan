//! Cache manager implementation
//!
//! This module provides the main cache manager for storing and retrieving
//! blockchain data with TTL support.

use super::types::{
    AddressInfo, CacheEntry, CacheStats, CachedAddressTransactions, CachedEnsName,
    CachedInternalTransactions, CachedTokenBalances, CachedTokenTransfers, ContractInfo, TokenInfo,
};
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
    // New caches for address lookup optimization
    address_transactions: Arc<Mutex<LruCache<String, CacheEntry<CachedAddressTransactions>>>>,
    token_transfers: Arc<Mutex<LruCache<String, CacheEntry<CachedTokenTransfers>>>>,
    token_balances: Arc<Mutex<LruCache<String, CacheEntry<CachedTokenBalances>>>>,
    internal_transactions: Arc<Mutex<LruCache<String, CacheEntry<CachedInternalTransactions>>>>,
    ens_names: Arc<Mutex<LruCache<String, CacheEntry<CachedEnsName>>>>,

    /// Configuration
    config: Config,
}

impl CacheManager {
    /// Create a new cache manager
    pub fn new(config: Config) -> Result<Self> {
        // Only create cache directory if caching is enabled
        if config.cache.enabled {
            let cache_dir = Config::cache_dir()?;
            // Use create_dir_all only if directory doesn't exist to avoid unnecessary work
            if !cache_dir.exists() {
                std::fs::create_dir_all(&cache_dir)?;
            }
        }

        // Use a single cache size constant to avoid repeated unwrap calls
        let cache_size = NonZeroUsize::new(1000).unwrap(); // Default cache size

        Ok(Self {
            blocks: Arc::new(Mutex::new(LruCache::new(cache_size))),
            transactions: Arc::new(Mutex::new(LruCache::new(cache_size))),
            addresses: Arc::new(Mutex::new(LruCache::new(cache_size))),
            contracts: Arc::new(Mutex::new(LruCache::new(cache_size))),
            tokens: Arc::new(Mutex::new(LruCache::new(cache_size))),
            address_transactions: Arc::new(Mutex::new(LruCache::new(cache_size))),
            token_transfers: Arc::new(Mutex::new(LruCache::new(cache_size))),
            token_balances: Arc::new(Mutex::new(LruCache::new(cache_size))),
            internal_transactions: Arc::new(Mutex::new(LruCache::new(cache_size))),
            ens_names: Arc::new(Mutex::new(LruCache::new(cache_size))),
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

    /// Get address transactions from cache
    pub fn get_address_transactions(
        &self,
        address: &str,
    ) -> Option<Vec<crate::blockchain::types::AddressTx>> {
        if !self.config.cache.enabled {
            return None;
        }

        let mut cache = self.address_transactions.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                // Convert from serializable format back to AddressTx
                return Some(
                    entry
                        .data
                        .transactions
                        .iter()
                        .map(|t| {
                            use crate::blockchain::types::TransactionStatus;
                            let status = match t.status.as_str() {
                                "Pending" => TransactionStatus::Pending,
                                "Success" => TransactionStatus::Success,
                                "Failed" => TransactionStatus::Failed,
                                _ => TransactionStatus::Unknown,
                            };
                            crate::blockchain::types::AddressTx {
                                tx_hash: t.tx_hash.clone(),
                                method: t.method.clone(),
                                block_number: t.block_number,
                                timestamp: t.timestamp,
                                from: t.from.clone(),
                                to: t.to.clone(),
                                value_eth: t.value_eth,
                                fee_eth: t.fee_eth,
                                status,
                            }
                        })
                        .collect(),
                );
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store address transactions in cache
    pub fn store_address_transactions(
        &self,
        address: String,
        transactions: Vec<crate::blockchain::types::AddressTx>,
    ) {
        if !self.config.cache.enabled {
            return;
        }

        // Convert to serializable format
        let serializable_txs: Vec<super::types::SerializableAddressTx> = transactions
            .iter()
            .map(|t| {
                let status = match t.status {
                    crate::blockchain::types::TransactionStatus::Pending => "Pending",
                    crate::blockchain::types::TransactionStatus::Success => "Success",
                    crate::blockchain::types::TransactionStatus::Failed => "Failed",
                    crate::blockchain::types::TransactionStatus::Unknown => "Unknown",
                };
                super::types::SerializableAddressTx {
                    tx_hash: t.tx_hash.clone(),
                    method: t.method.clone(),
                    block_number: t.block_number,
                    timestamp: t.timestamp,
                    from: t.from.clone(),
                    to: t.to.clone(),
                    value_eth: t.value_eth,
                    fee_eth: t.fee_eth,
                    status: status.to_string(),
                }
            })
            .collect();

        let entry = CacheEntry {
            data: CachedAddressTransactions {
                address: address.clone(),
                transactions: serializable_txs,
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.address_transactions_ttl_seconds,
        };

        let mut cache = self.address_transactions.lock().unwrap();
        cache.put(address, entry);
    }

    /// Get token transfers from cache
    pub fn get_token_transfers(
        &self,
        address: &str,
    ) -> Option<Vec<crate::blockchain::etherscan::TokenTransfer>> {
        if !self.config.cache.enabled {
            return None;
        }

        let mut cache = self.token_transfers.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                // Convert from serializable format back to TokenTransfer
                return Some(
                    entry
                        .data
                        .transfers
                        .iter()
                        .map(|t| crate::blockchain::etherscan::TokenTransfer {
                            token_id: t.token_id.clone(),
                            txn_hash: t.txn_hash.clone(),
                            from: t.from.clone(),
                            to: t.to.clone(),
                            token_name: t.token_name.clone(),
                            token_symbol: t.token_symbol.clone(),
                            amount: t.amount,
                            timestamp: t.timestamp,
                        })
                        .collect(),
                );
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store token transfers in cache
    pub fn store_token_transfers(
        &self,
        address: String,
        transfers: Vec<crate::blockchain::etherscan::TokenTransfer>,
    ) {
        if !self.config.cache.enabled {
            return;
        }

        // Convert to serializable format
        let serializable_transfers: Vec<super::types::SerializableTokenTransfer> = transfers
            .iter()
            .map(|t| super::types::SerializableTokenTransfer {
                token_id: t.token_id.clone(),
                txn_hash: t.txn_hash.clone(),
                from: t.from.clone(),
                to: t.to.clone(),
                token_name: t.token_name.clone(),
                token_symbol: t.token_symbol.clone(),
                amount: t.amount,
                timestamp: t.timestamp,
            })
            .collect();

        let entry = CacheEntry {
            data: CachedTokenTransfers {
                address: address.clone(),
                transfers: serializable_transfers,
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.token_transfers_ttl_seconds,
        };

        let mut cache = self.token_transfers.lock().unwrap();
        cache.put(address, entry);
    }

    /// Get token balances from cache
    pub fn get_token_balances(
        &self,
        address: &str,
    ) -> Option<Vec<crate::blockchain::etherscan::TokenBalance>> {
        if !self.config.cache.enabled {
            return None;
        }

        let mut cache = self.token_balances.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                // Convert from serializable format back to TokenBalance
                return Some(
                    entry
                        .data
                        .balances
                        .iter()
                        .map(|b| crate::blockchain::etherscan::TokenBalance {
                            contract_address: b.contract_address.clone(),
                            name: b.name.clone(),
                            symbol: b.symbol.clone(),
                            decimals: b.decimals,
                            balance: b.balance,
                        })
                        .collect(),
                );
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store token balances in cache
    pub fn store_token_balances(
        &self,
        address: String,
        balances: Vec<crate::blockchain::etherscan::TokenBalance>,
    ) {
        if !self.config.cache.enabled {
            return;
        }

        // Convert to serializable format
        let serializable_balances: Vec<super::types::SerializableTokenBalance> = balances
            .iter()
            .map(|b| super::types::SerializableTokenBalance {
                contract_address: b.contract_address.clone(),
                name: b.name.clone(),
                symbol: b.symbol.clone(),
                decimals: b.decimals,
                balance: b.balance,
            })
            .collect();

        let entry = CacheEntry {
            data: CachedTokenBalances {
                address: address.clone(),
                balances: serializable_balances,
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.token_balances_ttl_seconds,
        };

        let mut cache = self.token_balances.lock().unwrap();
        cache.put(address, entry);
    }

    /// Get internal transactions from cache
    pub fn get_internal_transactions(
        &self,
        address: &str,
    ) -> Option<Vec<crate::blockchain::etherscan::InternalTransaction>> {
        if !self.config.cache.enabled {
            return None;
        }

        let mut cache = self.internal_transactions.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                // Convert from serializable format back to InternalTransaction
                return Some(
                    entry
                        .data
                        .transactions
                        .iter()
                        .map(|t| crate::blockchain::etherscan::InternalTransaction {
                            parent_tx_hash: t.parent_tx_hash.clone(),
                            block: t.block,
                            from: t.from.clone(),
                            to: t.to.clone(),
                            value: t.value,
                            gas_limit: t.gas_limit,
                            gas_used: t.gas_used,
                            tx_type: t.tx_type.clone(),
                            timestamp: t.timestamp,
                        })
                        .collect(),
                );
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store internal transactions in cache
    pub fn store_internal_transactions(
        &self,
        address: String,
        transactions: Vec<crate::blockchain::etherscan::InternalTransaction>,
    ) {
        if !self.config.cache.enabled {
            return;
        }

        // Convert to serializable format
        let serializable_txns: Vec<super::types::SerializableInternalTransaction> = transactions
            .iter()
            .map(|t| super::types::SerializableInternalTransaction {
                parent_tx_hash: t.parent_tx_hash.clone(),
                block: t.block,
                from: t.from.clone(),
                to: t.to.clone(),
                value: t.value,
                gas_limit: t.gas_limit,
                gas_used: t.gas_used,
                tx_type: t.tx_type.clone(),
                timestamp: t.timestamp,
            })
            .collect();

        let entry = CacheEntry {
            data: CachedInternalTransactions {
                address: address.clone(),
                transactions: serializable_txns,
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.internal_transactions_ttl_seconds,
        };

        let mut cache = self.internal_transactions.lock().unwrap();
        cache.put(address, entry);
    }

    /// Get ENS name from cache
    pub fn get_ens_name(&self, address: &str) -> Option<Option<String>> {
        if !self.config.cache.enabled {
            return None;
        }

        let mut cache = self.ens_names.lock().unwrap();
        if let Some(entry) = cache.get(address) {
            if !self.is_expired(entry) {
                return Some(entry.data.ens_name.clone());
            } else {
                cache.pop(address);
            }
        }
        None
    }

    /// Store ENS name in cache
    pub fn store_ens_name(&self, address: String, ens_name: Option<String>) {
        if !self.config.cache.enabled {
            return;
        }

        let entry = CacheEntry {
            data: CachedEnsName {
                address: address.clone(),
                ens_name,
                last_updated: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            ttl_seconds: self.config.cache.ens_names_ttl_seconds,
        };

        let mut cache = self.ens_names.lock().unwrap();
        cache.put(address, entry);
    }

    /// Clear all caches
    pub fn clear_all(&self) {
        self.blocks.lock().unwrap().clear();
        self.transactions.lock().unwrap().clear();
        self.addresses.lock().unwrap().clear();
        self.contracts.lock().unwrap().clear();
        self.tokens.lock().unwrap().clear();
        self.address_transactions.lock().unwrap().clear();
        self.token_transfers.lock().unwrap().clear();
        self.token_balances.lock().unwrap().clear();
        self.internal_transactions.lock().unwrap().clear();
        self.ens_names.lock().unwrap().clear();
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStats {
        let blocks_count = self.blocks.lock().unwrap().len();
        let transactions_count = self.transactions.lock().unwrap().len();
        let addresses_count = self.addresses.lock().unwrap().len();
        let contracts_count = self.contracts.lock().unwrap().len();
        let tokens_count = self.tokens.lock().unwrap().len();
        let address_transactions_count = self.address_transactions.lock().unwrap().len();
        let token_transfers_count = self.token_transfers.lock().unwrap().len();
        let token_balances_count = self.token_balances.lock().unwrap().len();
        let internal_transactions_count = self.internal_transactions.lock().unwrap().len();
        let ens_names_count = self.ens_names.lock().unwrap().len();

        CacheStats {
            blocks_count,
            transactions_count,
            addresses_count,
            contracts_count,
            tokens_count,
            address_transactions_count,
            token_transfers_count,
            token_balances_count,
            internal_transactions_count,
            ens_names_count,
            total_entries: blocks_count
                + transactions_count
                + addresses_count
                + contracts_count
                + tokens_count
                + address_transactions_count
                + token_transfers_count
                + token_balances_count
                + internal_transactions_count
                + ens_names_count,
        }
    }
}

//! Blockchain service layer for WarpScan
//!
//! This module provides the interface for interacting with Ethereum blockchain
//! using ethers.rs library.

use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, Block, Transaction, TransactionReceipt, H256, U256},
};
use std::str::FromStr;
use std::sync::Arc;
use crate::cache::{CacheManager, AddressInfo};
use crate::config::Config;
use crate::error::{Error, Result};

/// Blockchain service for interacting with Ethereum
pub struct BlockchainService {
    provider: Arc<Provider<Http>>,
    cache: Arc<CacheManager>,
    config: Config,
}

/// Gas price information
#[derive(Debug, Clone)]
pub struct GasPrices {
    pub slow: U256,
    pub standard: U256,
    pub fast: U256,
    pub timestamp: u64,
}

/// Transaction status
#[derive(Debug, Clone)]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Unknown,
}

impl BlockchainService {
    /// Create a new blockchain service
    pub async fn new(config: Config, cache: Arc<CacheManager>) -> Result<Self> {
        let provider = Provider::<Http>::try_from(&config.network.rpc_url)
            .map_err(|e| Error::network(format!("Failed to create provider: {}", e)))?;
        
        let provider = Arc::new(provider);
        
        // Skip connection test during initialization to allow offline startup
        // Connection will be tested when first network call is made
        
        Ok(Self {
            provider,
            cache,
            config,
        })
    }
    
    /// Test network connection
    pub async fn test_connection(&self) -> Result<u64> {
        match tokio::time::timeout(
            std::time::Duration::from_secs(self.config.network.timeout_seconds),
            self.provider.get_chainid()
        ).await {
            Ok(Ok(chain_id)) => Ok(chain_id.as_u64()),
            Ok(Err(e)) => Err(Error::network(format!("Failed to connect to network: {}", e))),
            Err(_) => Err(Error::network("Connection timeout".to_string())),
        }
    }

    /// Get block by number
    pub async fn get_block_by_number(&self, block_number: u64) -> Result<Option<Block<H256>>> {
        // Check cache first
        if let Some(cached_block) = self.cache.get_block(block_number) {
            return Ok(Some(cached_block));
        }
        
        let block = self
            .provider
            .get_block(block_number)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        // Store in cache if found
        if let Some(ref block) = block {
            self.cache.store_block(block_number, block.clone());
        }
        
        Ok(block)
    }

    /// Get latest block
    pub async fn get_latest_block(&self) -> Result<Option<Block<H256>>> {
        let block = self
            .provider
            .get_block(ethers::types::BlockNumber::Latest)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        // Store in cache if found
        if let Some(ref block) = block {
            if let Some(number) = block.number {
                self.cache.store_block(number.as_u64(), block.clone());
            }
        }
        
        Ok(block)
    }

    /// Get transaction by hash
    pub async fn get_transaction_by_hash(&self, tx_hash: &str) -> Result<Option<Transaction>> {
        // Check cache first
        if let Some(cached_tx) = self.cache.get_transaction(tx_hash) {
            return Ok(Some(cached_tx));
        }
        
        let hash = H256::from_str(tx_hash)
            .map_err(|e| Error::parse(format!("Invalid transaction hash: {}", e)))?;
        
        let transaction = self
            .provider
            .get_transaction(hash)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        // Store in cache if found
        if let Some(ref tx) = transaction {
            self.cache.store_transaction(tx_hash.to_string(), tx.clone());
        }
        
        Ok(transaction)
    }

    /// Get transaction receipt
    pub async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Option<TransactionReceipt>> {
        let hash = H256::from_str(tx_hash)
            .map_err(|e| Error::parse(format!("Invalid transaction hash: {}", e)))?;
        
        let receipt = self
            .provider
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(receipt)
    }

    /// Get address balance
    pub async fn get_address_balance(&self, address: &str) -> Result<U256> {
        let addr = Address::from_str(address)
            .map_err(|e| Error::parse(format!("Invalid address: {}", e)))?;
        
        let balance = self
            .provider
            .get_balance(addr, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(balance)
    }

    /// Get address transaction count (nonce)
    pub async fn get_address_transaction_count(&self, address: &str) -> Result<U256> {
        let addr = Address::from_str(address)
            .map_err(|e| Error::parse(format!("Invalid address: {}", e)))?;
        
        let count = self
            .provider
            .get_transaction_count(addr, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(count)
    }

    /// Check if address is a contract
    pub async fn is_contract(&self, address: &str) -> Result<bool> {
        let addr = Address::from_str(address)
            .map_err(|e| Error::parse(format!("Invalid address: {}", e)))?;
        
        let code = self
            .provider
            .get_code(addr, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(!code.is_empty())
    }

    /// Get address information
    pub async fn get_address_info(&self, address: &str) -> Result<AddressInfo> {
        // Check cache first
        if let Some(cached_info) = self.cache.get_address_info(address) {
            return Ok(cached_info);
        }
        
        let balance = self.get_address_balance(address).await?;
        let tx_count = self.get_address_transaction_count(address).await?;
        let is_contract = self.is_contract(address).await?;
        
        let info = AddressInfo {
            address: address.to_string(),
            balance: balance.to_string(),
            transaction_count: tx_count.as_u64(),
            is_contract,
            last_updated: chrono::Utc::now().timestamp() as u64,
        };
        
        // Store in cache
        self.cache.store_address_info(address.to_string(), info.clone());
        
        Ok(info)
    }

    /// Get current gas prices
    pub async fn get_gas_prices(&self) -> Result<GasPrices> {
        let gas_price = self
            .provider
            .get_gas_price()
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        // Simple gas price estimation (in a real implementation, you might use more sophisticated methods)
        let slow = gas_price * 80 / 100;  // 80% of current price
        let standard = gas_price;         // Current price
        let fast = gas_price * 120 / 100; // 120% of current price
        
        Ok(GasPrices {
            slow,
            standard,
            fast,
            timestamp: chrono::Utc::now().timestamp() as u64,
        })
    }

    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(block_number.as_u64())
    }

    /// Get chain ID
    pub async fn get_chain_id(&self) -> Result<u64> {
        let chain_id = self
            .provider
            .get_chainid()
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(chain_id.as_u64())
    }

    /// Estimate gas for a transaction
    pub async fn estimate_gas(
        &self,
        from: &str,
        to: &str,
        data: Option<&str>,
        value: Option<U256>,
    ) -> Result<U256> {
        let from_addr = Address::from_str(from)
            .map_err(|e| Error::parse(format!("Invalid from address: {}", e)))?;
        let to_addr = Address::from_str(to)
            .map_err(|e| Error::parse(format!("Invalid to address: {}", e)))?;
        
        let mut tx = ethers::types::TransactionRequest::new()
            .from(from_addr)
            .to(to_addr);
        
        if let Some(val) = value {
            tx = tx.value(val);
        }
        
        if let Some(data_str) = data {
            let data_bytes = hex::decode(data_str.trim_start_matches("0x"))
                .map_err(|e| Error::parse(format!("Invalid hex data: {}", e)))?;
            tx = tx.data(data_bytes);
        }
        
        let typed_tx: ethers::types::transaction::eip2718::TypedTransaction = tx.into();
        let gas_estimate = self
            .provider
            .estimate_gas(&typed_tx, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(gas_estimate)
    }

    /// Get network name from chain ID
    pub fn get_network_name(&self) -> String {
        match self.config.network.chain_id {
            1 => "Mainnet".to_string(),
            5 => "Goerli".to_string(),
            11155111 => "Sepolia".to_string(),
            _ => format!("Chain {}", self.config.network.chain_id),
        }
    }
}
//! Blockchain service implementation

use ethers::{
    providers::{Http, Middleware, Provider},
    types::{Address, Block, Transaction, TransactionReceipt, H256, U256, TransactionRequest, transaction::eip2718::TypedTransaction},
};
use std::str::FromStr;
use std::sync::Arc;
use crate::cache::{CacheManager, AddressInfo};
use crate::config::Config;
use crate::error::{Error, Result};
use super::types::GasPrices;
use super::etherscan::{EtherscanClient, EtherscanChain};

/// Blockchain service for interacting with Ethereum
pub struct BlockchainService {
    provider: Arc<Provider<Http>>,
    cache: Arc<CacheManager>,
    config: Config,
    etherscan: Option<EtherscanClient>,
}

impl BlockchainService {
    /// Create a new blockchain service
    pub async fn new(config: Config, cache: Arc<CacheManager>) -> Result<Self> {
        let provider = Provider::<Http>::try_from(&config.network.rpc_url)
            .map_err(|e| Error::network(format!("Failed to create provider: {}", e)))?;
        
        let provider = Arc::new(provider);
        
        // Skip connection test during initialization to allow offline startup
        // Connection will be tested when first network call is made
        
        // Initialize Etherscan client if API key present
        let api_key = config.etherscan_api_key.clone().or_else(|| std::env::var("ETHERSCAN_API_KEY").ok());
        let etherscan = api_key.map(|key| {
            let chain = match config.network.chain_id {
                1 => EtherscanChain::Ethereum,
                5 => EtherscanChain::Goerli,
                11155111 => EtherscanChain::Sepolia,
                137 => EtherscanChain::Polygon,
                42161 => EtherscanChain::Arbitrum,
                10 => EtherscanChain::Optimism,
                8453 => EtherscanChain::Base,
                other => EtherscanChain::Custom(other),
            };
            EtherscanClient::new(key, chain)
        });
        
        Ok(Self {
            provider,
            cache,
            config,
            etherscan,
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
            .map_err(|e| Error::validation(format!("Invalid transaction hash: {}", e)))?;
        
        let tx = self
            .provider
            .get_transaction(hash)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        // Store in cache if found
        if let Some(ref tx) = tx {
            self.cache.store_transaction(tx_hash.to_string(), tx.clone());
        }
        
        Ok(tx)
    }

    /// Get transaction receipt
    pub async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Option<TransactionReceipt>> {
        let hash = H256::from_str(tx_hash)
            .map_err(|e| Error::validation(format!("Invalid transaction hash: {}", e)))?;
        
        self.provider
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))
    }

    /// Get address balance
    pub async fn get_address_balance(&self, address: &str) -> Result<U256> {
        // Prefer Etherscan V2 when configured
        if let Some(ref client) = self.etherscan {
            match client.get_address_balance(address).await {
                Ok(bal) => return Ok(bal),
                Err(err) => {
                    // Fallback to provider on error
                    tracing::warn!(target = "warpscan", "Etherscan failed for balance: {}. Falling back to provider.", err);
                }
            }
        }
        let addr = Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))?;
        
        self.provider
            .get_balance(addr, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))
    }

    /// Get address transaction count (nonce)
    pub async fn get_address_transaction_count(&self, address: &str) -> Result<U256> {
        let addr = Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))?;
        
        self.provider
            .get_transaction_count(addr, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))
    }

    /// Check if address is a contract
    pub async fn is_contract(&self, address: &str) -> Result<bool> {
        let addr = Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))?;
        
        let code = self.provider.get_code(addr, None).await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        Ok(!code.is_empty())
    }

    /// Get comprehensive address information
    pub async fn get_address_info(&self, address: &str) -> Result<AddressInfo> {
        // Check cache first
        if let Some(cached_info) = self.cache.get_address_info(address) {
            return Ok(cached_info);
        }
        
        let balance = self.get_address_balance(address).await?;
        let transaction_count = self.get_address_transaction_count(address).await?;
        let is_contract = self.is_contract(address).await?;
        
        let info = AddressInfo {
            address: address.to_string(),
            balance: balance.to_string(),
            transaction_count: transaction_count.as_u64(),
            is_contract,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        // Store in cache
        self.cache.store_address_info(address.to_string(), info.clone());
        
        Ok(info)
    }

    /// Get current gas prices
    pub async fn get_gas_prices(&self) -> Result<GasPrices> {
        let gas_price = self.provider
            .get_gas_price()
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;
        
        // Simple gas price estimation (in a real implementation, you might use a gas oracle)
        let slow = gas_price * 80 / 100;  // 80% of current
        let standard = gas_price;
        let fast = gas_price * 120 / 100; // 120% of current
        
        Ok(GasPrices {
            slow,
            standard,
            fast,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }

    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        self.provider
            .get_block_number()
            .await
            .map(|n| n.as_u64())
            .map_err(|e| Error::blockchain(format!("{}", e)))
    }

    /// Get chain ID
    pub async fn get_chain_id(&self) -> Result<u64> {
        self.provider
            .get_chainid()
            .await
            .map(|n| n.as_u64())
            .map_err(|e| Error::blockchain(format!("{}", e)))
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
            .map_err(|e| Error::validation(format!("Invalid from address: {}", e)))?;
        let to_addr = Address::from_str(to)
            .map_err(|e| Error::validation(format!("Invalid to address: {}", e)))?;
        
        let mut tx = TransactionRequest::new()
            .from(from_addr)
            .to(to_addr);
        
        if let Some(data) = data {
            let data_bytes = hex::decode(data.trim_start_matches("0x"))
                .map_err(|e| Error::validation(format!("Invalid data: {}", e)))?;
            tx = tx.data(data_bytes);
        }
        
        if let Some(value) = value {
            tx = tx.value(value);
        }
        
        let typed_tx = TypedTransaction::Legacy(tx.into());
        self.provider
            .estimate_gas(&typed_tx, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))
    }

    /// Get network name based on chain ID
    pub fn get_network_name(&self) -> String {
        self.config.network.name.clone()
    }
}
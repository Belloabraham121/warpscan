//! Blockchain service implementation

use super::etherscan::{
    EtherscanChain, EtherscanClient, InternalTransaction as EtherscanInternalTransaction,
    TokenBalance as EtherscanTokenBalance, TokenTransfer as EtherscanTokenTransfer,
};
use super::types::AddressTx;
use super::types::GasPrices;
use crate::cache::{AddressInfo, CacheManager};
use crate::config::Config;
use crate::error::{Error, Result};
use ethers::{
    providers::{Http, Middleware, Provider},
    types::{
        transaction::eip2718::TypedTransaction, Address, Block, Transaction, TransactionReceipt,
        TransactionRequest, H256, U256,
    },
};
use std::str::FromStr;
use std::sync::Arc;

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
        // Create provider - this is fast, no network call
        let provider = Provider::<Http>::try_from(&config.network.rpc_url)
            .map_err(|e| Error::network(format!("Failed to create provider: {}", e)))?;

        let provider = Arc::new(provider);

        // Skip connection test during initialization to allow offline startup
        // Connection will be tested when first network call is made

        // Initialize Etherscan client if API key present
        // Optimize: check env var first (faster) before cloning config
        let api_key = std::env::var("ETHERSCAN_API_KEY")
            .ok()
            .or_else(|| config.etherscan_api_key.clone());

        let etherscan = api_key.map(|key| {
            // Use a match expression directly instead of storing in variable
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
            self.provider.get_chainid(),
        )
        .await
        {
            Ok(Ok(chain_id)) => Ok(chain_id.as_u64()),
            Ok(Err(e)) => Err(Error::network(format!(
                "Failed to connect to network: {}",
                e
            ))),
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
            self.cache
                .store_transaction(tx_hash.to_string(), tx.clone());
        }

        Ok(tx)
    }

    /// Get transaction receipt
    pub async fn get_transaction_receipt(
        &self,
        tx_hash: &str,
    ) -> Result<Option<TransactionReceipt>> {
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
                    tracing::warn!(
                        target = "warpscan",
                        "Etherscan failed for balance: {}. Falling back to provider.",
                        err
                    );
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

        let code = self
            .provider
            .get_code(addr, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;

        Ok(!code.is_empty())
    }

    /// Get comprehensive address information
    pub async fn get_address_info(&self, address: &str) -> Result<AddressInfo> {
        // Check cache first
        if let Some(cached_info) = self.cache.get_address_info(address) {
            return Ok(cached_info);
        }

        // PARALLELIZE: Fetch balance, transaction count, and contract status concurrently
        // These calls are independent and can be done in parallel
        let (balance_result, transaction_count_result, is_contract_result) = tokio::join!(
            self.get_address_balance(address),
            self.get_address_transaction_count(address),
            self.is_contract(address),
        );

        let balance = balance_result?;
        let transaction_count = transaction_count_result?;
        let is_contract = is_contract_result?;

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
        self.cache
            .store_address_info(address.to_string(), info.clone());

        Ok(info)
    }

    /// Get address transactions (normal transactions)
    pub async fn get_address_transactions(&self, address: &str) -> Result<Vec<AddressTx>> {
        // Check cache first
        if let Some(cached_txs) = self.cache.get_address_transactions(address) {
            tracing::debug!(target: "warpscan", "Cache hit for address transactions: {}", address);
            return Ok(cached_txs);
        }

        // Prefer Etherscan V2 when configured
        if let Some(ref client) = self.etherscan {
            match client.get_address_transactions(address).await {
                Ok(txs) => {
                    // Store in cache for future use
                    self.cache
                        .store_address_transactions(address.to_string(), txs.clone());
                    tracing::debug!(target: "warpscan", "Cached address transactions for: {}", address);
                    return Ok(txs);
                }
                Err(err) => {
                    tracing::warn!(
                        target = "warpscan",
                        "Etherscan failed for txlist: {}. Returning empty list.",
                        err
                    );
                    return Ok(vec![]);
                }
            }
        }
        // Fallback: provider doesn't offer per-address tx listing easily; return empty for now
        Ok(vec![])
    }

    /// Get token transfers for an address
    pub async fn get_token_transfers(&self, address: &str) -> Result<Vec<EtherscanTokenTransfer>> {
        // Check cache first
        if let Some(cached_transfers) = self.cache.get_token_transfers(address) {
            tracing::debug!(target: "warpscan", "Cache hit for token transfers: {}", address);
            return Ok(cached_transfers);
        }

        if let Some(ref client) = self.etherscan {
            match client.get_token_transfers(address).await {
                Ok(transfers) => {
                    // Store in cache for future use
                    self.cache
                        .store_token_transfers(address.to_string(), transfers.clone());
                    tracing::debug!(target: "warpscan", "Cached token transfers for: {}", address);
                    return Ok(transfers);
                }
                Err(err) => {
                    tracing::warn!(
                        target = "warpscan",
                        "Etherscan failed for tokentx: {}. Returning empty list.",
                        err
                    );
                    return Ok(vec![]);
                }
            }
        }
        Ok(vec![])
    }

    /// Get internal transactions for an address
    pub async fn get_internal_transactions(
        &self,
        address: &str,
    ) -> Result<Vec<EtherscanInternalTransaction>> {
        // Check cache first
        if let Some(cached_txns) = self.cache.get_internal_transactions(address) {
            tracing::debug!(target: "warpscan", "Cache hit for internal transactions: {}", address);
            return Ok(cached_txns);
        }

        if let Some(ref client) = self.etherscan {
            match client.get_internal_transactions(address).await {
                Ok(txns) => {
                    // Store in cache for future use
                    self.cache
                        .store_internal_transactions(address.to_string(), txns.clone());
                    tracing::debug!(target: "warpscan", "Cached internal transactions for: {}", address);
                    return Ok(txns);
                }
                Err(err) => {
                    tracing::warn!(
                        target = "warpscan",
                        "Etherscan failed for txlistinternal: {}. Returning empty list.",
                        err
                    );
                    return Ok(vec![]);
                }
            }
        }
        Ok(vec![])
    }

    /// Get token balances for an address
    pub async fn get_token_balances(&self, address: &str) -> Result<Vec<EtherscanTokenBalance>> {
        // Check cache first
        if let Some(cached_balances) = self.cache.get_token_balances(address) {
            tracing::debug!(target: "warpscan", "Cache hit for token balances: {}", address);
            return Ok(cached_balances);
        }

        if let Some(ref client) = self.etherscan {
            match client.get_token_balances(address).await {
                Ok(tokens) => {
                    // Store in cache for future use
                    self.cache
                        .store_token_balances(address.to_string(), tokens.clone());
                    tracing::debug!(target: "warpscan", "Cached token balances for: {}", address);
                    return Ok(tokens);
                }
                Err(err) => {
                    tracing::warn!(
                        target = "warpscan",
                        "Etherscan failed for tokenlist: {}. Returning empty list.",
                        err
                    );
                    return Ok(vec![]);
                }
            }
        }
        Ok(vec![])
    }

    /// Get current gas prices
    pub async fn get_gas_prices(&self) -> Result<GasPrices> {
        let gas_price = self
            .provider
            .get_gas_price()
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))?;

        // Simple gas price estimation (in a real implementation, you might use a gas oracle)
        let slow = gas_price * 80 / 100; // 80% of current
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

        let mut tx = TransactionRequest::new().from(from_addr).to(to_addr);

        if let Some(data) = data {
            let data_bytes = hex::decode(data.trim_start_matches("0x"))
                .map_err(|e| Error::validation(format!("Invalid data: {}", e)))?;
            tx = tx.data(data_bytes);
        }

        if let Some(value) = value {
            tx = tx.value(value);
        }

        let typed_tx = TypedTransaction::Legacy(tx);
        self.provider
            .estimate_gas(&typed_tx, None)
            .await
            .map_err(|e| Error::blockchain(format!("{}", e)))
    }

    /// Get network name based on chain ID
    pub fn get_network_name(&self) -> String {
        self.config.network.name.clone()
    }

    /// Resolve ENS name for an address (only works on mainnet)
    pub async fn resolve_ens_name(&self, address: &str) -> Result<Option<String>> {
        // Only resolve ENS on Ethereum mainnet (chain ID 1)
        if self.config.network.chain_id != 1 {
            return Ok(None);
        }

        // Check cache first
        if let Some(cached_ens) = self.cache.get_ens_name(address) {
            tracing::debug!(target: "warpscan", "Cache hit for ENS name: {}", address);
            return Ok(cached_ens);
        }

        let addr = Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))?;

        // Use ethers-rs ENS resolver
        let ens_result = match self.provider.lookup_address(addr).await {
            Ok(name) => Ok(Some(name)),
            Err(_) => {
                // ENS resolution failed - address might not have an ENS name
                // This is not an error, just return None
                Ok(None)
            }
        };

        // Store in cache (even if None, to avoid repeated lookups)
        if let Ok(ref ens_name) = ens_result {
            self.cache
                .store_ens_name(address.to_string(), ens_name.clone());
            tracing::debug!(target: "warpscan", "Cached ENS name for: {}", address);
        }

        ens_result
    }

    /// Get transaction details - tries Etherscan first, falls back to RPC (for local nodes)
    pub async fn get_transaction_details(
        &self,
        tx_hash: &str,
    ) -> Result<crate::ui::models::TransactionDetails> {
        use crate::ui::models::{TransactionDetails, TransactionStatus};

        // Check if this is a local node (Anvil/Hardhat) - skip Etherscan for local nodes
        let is_local_node = self
            .config
            .network
            .node_type
            .as_ref()
            .map(|t| t == "anvil" || t == "hardhat")
            .unwrap_or(false);

        // Try Etherscan first if available and not a local node
        if !is_local_node {
            if let Some(ref client) = self.etherscan {
                match client.get_transaction_details(tx_hash).await {
                    Ok(etherscan_tx) => {
                        // Convert Etherscan format to UI model
                        let current_block = self.get_block_number().await.unwrap_or(0);
                        let confirmations = if etherscan_tx.block_number > 0
                            && current_block > etherscan_tx.block_number
                        {
                            current_block - etherscan_tx.block_number
                        } else {
                            0
                        };

                        // Try to decode method from input data (first 4 bytes = method selector)
                        let method = if etherscan_tx.input_data.len() >= 10 {
                            // Could decode method selector here, but for now return None
                            None
                        } else {
                            None
                        };

                        // Calculate transaction fee (gas_used * gas_price in wei, convert to ETH)
                        // gas_price is in gwei, so convert to wei first
                        let gas_price_wei = etherscan_tx.gas_price as f64 * 1_000_000_000.0;
                        let transaction_fee = (etherscan_tx.gas_used as f64 * gas_price_wei)
                            / 1_000_000_000_000_000_000.0;

                        // Fetch transfers for this transaction
                        let mut transfers = self
                            .get_transaction_transfers(
                                tx_hash,
                                &etherscan_tx.from,
                                etherscan_tx.to.as_deref(),
                            )
                            .await
                            .unwrap_or_default();

                        // Add main ETH transfer if value > 0
                        if etherscan_tx.value > 0.0 {
                            if let Some(ref to_address) = etherscan_tx.to {
                                use crate::ui::models::transaction::{
                                    TransactionTransfer, TransferType,
                                };
                                transfers.insert(
                                    0,
                                    TransactionTransfer {
                                        transfer_type: TransferType::ETH,
                                        from: etherscan_tx.from.clone(),
                                        to: to_address.clone(),
                                        value: etherscan_tx.value,
                                        token_symbol: None,
                                        token_name: None,
                                        token_address: None,
                                    },
                                );
                            }
                        }

                        return Ok(TransactionDetails {
                            hash: etherscan_tx.hash,
                            status: if etherscan_tx.is_error {
                                TransactionStatus::Failed
                            } else {
                                TransactionStatus::Success
                            },
                            block_number: etherscan_tx.block_number,
                            timestamp: etherscan_tx.timestamp,
                            from: etherscan_tx.from,
                            to: etherscan_tx.to,
                            value: etherscan_tx.value,
                            gas_limit: etherscan_tx.gas_limit,
                            gas_used: etherscan_tx.gas_used,
                            gas_price: etherscan_tx.gas_price,
                            transaction_fee,
                            nonce: etherscan_tx.nonce,
                            transaction_index: etherscan_tx.transaction_index,
                            input_data: etherscan_tx.input_data,
                            method,
                            contract_address: etherscan_tx.contract_address,
                            confirmations,
                            transfers,
                        });
                    }
                    Err(err) => {
                        // Fallback to RPC if Etherscan fails
                        tracing::warn!(
                            target = "warpscan",
                            "Etherscan failed for transaction: {}. Falling back to RPC.",
                            err
                        );
                    }
                }
            }
        }

        // Fallback to RPC provider (works with local nodes like Anvil/Hardhat)
        let hash = H256::from_str(tx_hash)
            .map_err(|e| Error::validation(format!("Invalid transaction hash: {}", e)))?;

        let tx = self
            .provider
            .get_transaction(hash)
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get transaction: {}", e)))?;

        let tx = tx.ok_or_else(|| Error::blockchain("Transaction not found".to_string()))?;

        let receipt = self
            .provider
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| Error::blockchain(format!("Failed to get receipt: {}", e)))?;

        let block_number = tx.block_number.map(|n| n.as_u64()).unwrap_or(0);
        let current_block = self.get_block_number().await.unwrap_or(0);
        let confirmations = if block_number > 0 && current_block > block_number {
            current_block - block_number
        } else {
            0
        };

        // Get block timestamp
        let timestamp = if block_number > 0 {
            self.get_block_by_number(block_number)
                .await?
                .map(|b| b.timestamp.as_u64())
                .unwrap_or(0)
        } else {
            0
        };

        let value_eth =
            tx.value.to_string().parse::<f64>().unwrap_or(0.0) / 1_000_000_000_000_000_000.0;

        let gas_price_gwei = if let Some(gp) = &tx.gas_price {
            (gp.to_string().parse::<f64>().unwrap_or(0.0) / 1_000_000_000.0) as u64
        } else {
            0
        };

        let gas_used = receipt
            .as_ref()
            .and_then(|r| r.gas_used.map(|g| g.as_u64()))
            .unwrap_or(0);

        // Calculate transaction fee (gas_used * gas_price in wei, convert to ETH)
        let gas_price_wei = gas_price_gwei as f64 * 1_000_000_000.0;
        let transaction_fee = (gas_used as f64 * gas_price_wei) / 1_000_000_000_000_000_000.0;

        let status = if let Some(ref r) = receipt {
            if r.status == Some(ethers::types::U64::from(1)) {
                TransactionStatus::Success
            } else {
                TransactionStatus::Failed
            }
        } else {
            TransactionStatus::Pending
        };

        // Try to decode method from input data
        let method = if tx.input.len() >= 4 {
            // Could decode method selector here
            None
        } else {
            None
        };

        // Fetch transfers for this transaction
        let from_addr = format!("{:?}", tx.from);
        let to_addr = tx.to.as_ref().map(|a| format!("{:?}", a));
        let mut transfers = self
            .get_transaction_transfers(tx_hash, &from_addr, to_addr.as_deref())
            .await
            .unwrap_or_default();

        // Add main ETH transfer if value > 0
        if value_eth > 0.0 {
            if let Some(ref to_address) = to_addr {
                use crate::ui::models::transaction::{TransactionTransfer, TransferType};
                transfers.insert(
                    0,
                    TransactionTransfer {
                        transfer_type: TransferType::ETH,
                        from: from_addr.clone(),
                        to: to_address.clone(),
                        value: value_eth,
                        token_symbol: None,
                        token_name: None,
                        token_address: None,
                    },
                );
            }
        }

        Ok(TransactionDetails {
            hash: tx_hash.to_string(),
            status,
            block_number,
            timestamp,
            from: from_addr,
            to: to_addr,
            value: value_eth,
            gas_limit: tx.gas.as_u64(),
            gas_used,
            gas_price: gas_price_gwei,
            transaction_fee,
            nonce: tx.nonce.as_u64(),
            transaction_index: tx.transaction_index.map(|i| i.as_u64()),
            input_data: format!("0x{}", hex::encode(&tx.input)),
            method,
            contract_address: receipt
                .as_ref()
                .and_then(|r| r.contract_address.map(|a| format!("{:?}", a))),
            confirmations,
            transfers,
        })
    }

    /// Get all transfers for a transaction (ETH, tokens, internal)
    async fn get_transaction_transfers(
        &self,
        tx_hash: &str,
        from: &str,
        to: Option<&str>,
    ) -> Result<Vec<crate::ui::models::transaction::TransactionTransfer>> {
        use crate::ui::models::transaction::{TransactionTransfer, TransferType};

        let mut transfers = Vec::new();

        // Get token transfers for this transaction
        if let Some(ref client) = self.etherscan {
            // Get token transfers from both from and to addresses, filter by tx_hash
            let from_transfers = client.get_token_transfers(from).await.unwrap_or_default();
            let to_transfers = if let Some(to_addr) = to {
                client
                    .get_token_transfers(to_addr)
                    .await
                    .unwrap_or_default()
            } else {
                Vec::new()
            };

            // Filter by transaction hash and convert to TransactionTransfer
            for transfer in from_transfers.iter().chain(to_transfers.iter()) {
                if transfer.txn_hash.to_lowercase() == tx_hash.to_lowercase() {
                    transfers.push(TransactionTransfer {
                        transfer_type: TransferType::Token,
                        from: transfer.from.clone(),
                        to: transfer.to.clone(),
                        value: transfer.amount,
                        token_symbol: Some(transfer.token_symbol.clone()),
                        token_name: Some(transfer.token_name.clone()),
                        token_address: None, // Could be added if needed
                    });
                }
            }

            // Get internal transactions
            let from_internal = client
                .get_internal_transactions(from)
                .await
                .unwrap_or_default();
            let to_internal = if let Some(to_addr) = to {
                client
                    .get_internal_transactions(to_addr)
                    .await
                    .unwrap_or_default()
            } else {
                Vec::new()
            };

            // Filter by parent transaction hash
            for internal in from_internal.iter().chain(to_internal.iter()) {
                if internal.parent_tx_hash.to_lowercase() == tx_hash.to_lowercase() {
                    transfers.push(TransactionTransfer {
                        transfer_type: TransferType::Internal,
                        from: internal.from.clone(),
                        to: internal.to.clone(),
                        value: internal.value,
                        token_symbol: None,
                        token_name: None,
                        token_address: None,
                    });
                }
            }
        }

        Ok(transfers)
    }
}

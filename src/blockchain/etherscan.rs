//! Etherscan V2 multichain client

use crate::blockchain::types::{AddressTx, TransactionStatus};
use crate::error::{Error, Result};
use ethers::types::U256;
use reqwest::Client;

/// Supported chains for Etherscan V2
#[derive(Debug, Clone, Copy)]
pub enum EtherscanChain {
    Ethereum,    // 1
    Goerli,      // 5
    Sepolia,     // 11155111
    Polygon,     // 137
    Arbitrum,    // 42161
    Optimism,    // 10
    Base,        // 8453
    Custom(u64), // Any other chain supported by Etherscan-like explorers
}

/// Token transfer data structure for Etherscan API
#[derive(Debug, Clone)]
pub struct TokenTransfer {
    pub token_id: Option<String>,
    pub txn_hash: String,
    pub from: String,
    pub to: String,
    pub token_name: String,
    pub token_symbol: String,
    pub amount: f64,
    pub timestamp: u64,
}

/// Internal transaction data structure for Etherscan API
#[derive(Debug, Clone)]
pub struct InternalTransaction {
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

/// Token balance information from Etherscan API
#[derive(Debug, Clone)]
pub struct TokenBalance {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub balance: f64,
}

impl EtherscanChain {
    fn chain_id(&self) -> u64 {
        match self {
            EtherscanChain::Ethereum => 1,
            EtherscanChain::Goerli => 5,
            EtherscanChain::Sepolia => 11155111,
            EtherscanChain::Polygon => 137,
            EtherscanChain::Arbitrum => 42161,
            EtherscanChain::Optimism => 10,
            EtherscanChain::Base => 8453,
            EtherscanChain::Custom(id) => *id,
        }
    }
}

/// Simple Etherscan V2 client
#[derive(Clone)]
pub struct EtherscanClient {
    api_key: String,
    client: Client,
    chain: EtherscanChain,
}

impl EtherscanClient {
    /// Create a new client with the provided API key and chain
    pub fn new(api_key: String, chain: EtherscanChain) -> Self {
        // Optimize HTTP client with connection pooling and timeouts
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10)) // 10 second timeout
            .tcp_keepalive(std::time::Duration::from_secs(60)) // Keep connections alive
            .pool_max_idle_per_host(10) // Reuse connections
            .build()
            .expect("Failed to build HTTP client");
        Self {
            api_key,
            client,
            chain,
        }
    }

    /// Base URL for Etherscan V2
    fn base_url(&self) -> &'static str {
        // Etherscan V2 unified endpoint
        "https://api.etherscan.io/v2/api"
    }

    /// Get address balance via Etherscan V2
    pub async fn get_address_balance(&self, address: &str) -> Result<U256> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "account".to_string()),
                ("action", "balance".to_string()),
                ("address", address.to_string()),
                ("tag", "latest".to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(Error::serialization)?;

        // Etherscan V2 returns data under `result` for many endpoints
        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;

        // Expect result to be a string representing a balance in wei
        let balance_str = result
            .as_str()
            .ok_or_else(|| Error::parse("Unexpected result type for balance"))?;
        let balance = U256::from_dec_str(balance_str)
            .map_err(|e| Error::parse(format!("Failed to parse balance: {}", e)))?;

        Ok(balance)
    }

    /// Get normal transactions for an address via Etherscan V2
    pub async fn get_address_transactions(&self, address: &str) -> Result<Vec<AddressTx>> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "account".to_string()),
                ("action", "txlist".to_string()),
                ("address", address.to_string()),
                ("startblock", "0".to_string()),
                ("endblock", "99999999".to_string()),
                ("sort", "desc".to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
            Error::parse(format!(
                "Failed to parse Etherscan JSON: {} | Response: {}",
                e, text
            ))
        })?;

        // Check for API-level errors first
        if let Some(status) = json.get("status") {
            if let Some(status_str) = status.as_str() {
                if status_str != "1" {
                    let message = json
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Unknown error");
                    let result_msg = json.get("result").and_then(|r| r.as_str()).unwrap_or("");
                    return Err(Error::network(format!(
                        "Etherscan API error (status={}): {} | result: {}",
                        status_str, message, result_msg
                    )));
                }
            }
        }

        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;

        // Handle case where result is an error string instead of array
        if let Some(error_msg) = result.as_str() {
            if error_msg.is_empty() || error_msg == "No transactions found" {
                // This is fine, just return empty list
                tracing::info!(
                    target: "warpscan",
                    "Etherscan returned no transactions for address: {}",
                    address
                );
                return Ok(vec![]);
            }
            // Otherwise it's an actual error message
            return Err(Error::network(format!(
                "Etherscan returned error message: {}",
                error_msg
            )));
        }

        let arr = result.as_array().ok_or_else(|| {
            Error::parse(format!(
                "Unexpected result type for txlist: expected array, got {:?} | Full response: {}",
                result, text
            ))
        })?;

        let txs: Vec<AddressTx> = arr
            .iter()
            .filter_map(|item| {
                // Safely map fields, skip if mandatory fields missing
                let tx_hash = item.get("hash")?.as_str()?.to_string();
                let block_number = item.get("blockNumber")?.as_str()?.parse::<u64>().ok()?;
                let time_stamp = item.get("timeStamp")?.as_str()?.parse::<u64>().ok()?;
                let from = item.get("from")?.as_str()?.to_string();
                let to = item.get("to")?.as_str()?.to_string();
                let value_wei = item.get("value")?.as_str()?.to_string();
                let gas_price_wei = item.get("gasPrice")?.as_str()?.to_string();
                let gas_used = item.get("gasUsed")?.as_str()?.parse::<u64>().ok()?;
                let is_error = item.get("isError")?.as_str()?.to_string();

                // Method/function name: Etherscan V2 may provide different fields
                // Prefer `functionName`, then `methodId`, then derive from `input`.
                let method = item
                    .get("functionName")
                    .and_then(|m| m.as_str())
                    .map(|s| s.to_string())
                    .or_else(|| {
                        item.get("methodId")
                            .and_then(|m| m.as_str())
                            .map(|s| s.to_string())
                    })
                    .or_else(|| {
                        item.get("method")
                            .and_then(|m| m.as_str())
                            .map(|s| s.to_string())
                    })
                    .or_else(|| {
                        item.get("input").and_then(|m| m.as_str()).map(|input| {
                            // Use first 10 chars (method selector) if present
                            if input.len() >= 10 {
                                input[..10].to_string()
                            } else {
                                input.to_string()
                            }
                        })
                    })
                    .unwrap_or_default();

                // OPTIMIZE: Convert values using constant for division
                const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000.0;
                let value_eth = U256::from_dec_str(&value_wei)
                    .map(|wei| wei.as_u128() as f64 / WEI_TO_ETH)
                    .unwrap_or(0.0);

                let gas_price_eth_per_gas = U256::from_dec_str(&gas_price_wei)
                    .map(|wei| wei.as_u128() as f64 / WEI_TO_ETH)
                    .unwrap_or(0.0);
                let fee_eth = gas_price_eth_per_gas * (gas_used as f64);

                let status = match is_error.as_str() {
                    "0" => TransactionStatus::Success,
                    "1" => TransactionStatus::Failed,
                    _ => TransactionStatus::Unknown,
                };

                Some(AddressTx {
                    tx_hash,
                    method,
                    block_number,
                    timestamp: time_stamp,
                    from,
                    to,
                    value_eth,
                    fee_eth,
                    status,
                })
            })
            .collect();

        Ok(txs)
    }

    /// Get token transfers for an address via Etherscan V2
    pub async fn get_token_transfers(&self, address: &str) -> Result<Vec<TokenTransfer>> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "account".to_string()),
                ("action", "tokentx".to_string()),
                ("address", address.to_string()),
                ("startblock", "0".to_string()),
                ("endblock", "99999999".to_string()),
                ("sort", "desc".to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
            Error::parse(format!(
                "Failed to parse Etherscan JSON: {} | Response: {}",
                e, text
            ))
        })?;

        // Check for API-level errors first
        if let Some(status) = json.get("status") {
            if let Some(status_str) = status.as_str() {
                if status_str != "1" {
                    let message = json
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Unknown error");
                    let result_msg = json.get("result").and_then(|r| r.as_str()).unwrap_or("");
                    return Err(Error::network(format!(
                        "Etherscan API error (status={}): {} | result: {}",
                        status_str, message, result_msg
                    )));
                }
            }
        }

        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;

        // Handle case where result is an error string instead of array
        if let Some(error_msg) = result.as_str() {
            if error_msg.is_empty() || error_msg == "No transfers found" {
                // This is fine, just return empty list
                tracing::info!(
                    target: "warpscan",
                    "Etherscan returned no token transfers for address: {}",
                    address
                );
                return Ok(vec![]);
            }
            // Otherwise it's an actual error message
            return Err(Error::network(format!(
                "Etherscan returned error message: {}",
                error_msg
            )));
        }

        let arr = result.as_array().ok_or_else(|| {
            Error::parse(format!(
                "Unexpected result type for tokentx: expected array, got {:?} | Full response: {}",
                result, text
            ))
        })?;

        let transfers: Vec<TokenTransfer> = arr
            .iter()
            .filter_map(|item| {
                let txn_hash = item.get("hash")?.as_str()?.to_string();
                let from = item.get("from")?.as_str()?.to_string();
                let to = item.get("to")?.as_str()?.to_string();
                let token_name = item
                    .get("tokenName")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let token_symbol = item
                    .get("tokenSymbol")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let value_str = item.get("value")?.as_str()?.to_string();
                let decimals_str = item
                    .get("tokenDecimal")
                    .and_then(|v| v.as_str())
                    .unwrap_or("18");
                let decimals = decimals_str.parse::<u8>().unwrap_or(18);
                let timestamp = item.get("timeStamp")?.as_str()?.parse::<u64>().ok()?;

                // Token ID for ERC-721/ERC-1155
                let token_id = item
                    .get("tokenID")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                // Convert value with proper decimals
                let value_wei = U256::from_dec_str(&value_str).ok()?;
                // Use U256 for precise division instead of lossy float/string conversions
                let divisor = U256::exp10(decimals as usize);
                let amount = if divisor > U256::zero() {
                    // Divide in U256 then convert to f64 (beware of losing large mantissa)
                    let quotient = value_wei.checked_div(divisor).unwrap_or(U256::zero());
                    let remainder = value_wei.checked_rem(divisor).unwrap_or(U256::zero());
                    let quotient_f64 = quotient.as_u128() as f64;
                    let remainder_f64 = remainder.as_u128() as f64 / divisor.as_u128() as f64;
                    quotient_f64 + remainder_f64
                } else {
                    0.0
                };

                Some(TokenTransfer {
                    token_id,
                    txn_hash,
                    from,
                    to,
                    token_name,
                    token_symbol,
                    amount,
                    timestamp,
                })
            })
            .collect();

        Ok(transfers)
    }

    /// Get internal transactions for an address via Etherscan V2
    pub async fn get_internal_transactions(
        &self,
        address: &str,
    ) -> Result<Vec<InternalTransaction>> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "account".to_string()),
                ("action", "txlistinternal".to_string()),
                ("address", address.to_string()),
                ("startblock", "0".to_string()),
                ("endblock", "99999999".to_string()),
                ("sort", "desc".to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(|e| {
            Error::parse(format!(
                "Failed to parse Etherscan JSON: {} | Response: {}",
                e, text
            ))
        })?;

        // Check for API-level errors first
        if let Some(status) = json.get("status") {
            if let Some(status_str) = status.as_str() {
                if status_str != "1" {
                    let message = json
                        .get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("Unknown error");
                    let result_msg = json.get("result").and_then(|r| r.as_str()).unwrap_or("");
                    return Err(Error::network(format!(
                        "Etherscan API error (status={}): {} | result: {}",
                        status_str, message, result_msg
                    )));
                }
            }
        }

        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;

        // Handle case where result is an error string instead of array
        if let Some(error_msg) = result.as_str() {
            if error_msg.is_empty() || error_msg == "No internal transactions found" {
                // This is fine, just return empty list
                tracing::info!(
                    target: "warpscan",
                    "Etherscan returned no internal transactions for address: {}",
                    address
                );
                return Ok(vec![]);
            }
            // Otherwise it's an actual error message
            return Err(Error::network(format!(
                "Etherscan returned error message: {}",
                error_msg
            )));
        }

        let arr = result
            .as_array()
            .ok_or_else(|| {
                Error::parse(format!(
                    "Unexpected result type for txlistinternal: expected array, got {:?} | Full response: {}",
                    result, text
                ))
            })?;

        let internal_txns: Vec<InternalTransaction> = arr
            .iter()
            .filter_map(|item| {
                let parent_tx_hash = item.get("hash")?.as_str()?.to_string();
                let block = item.get("blockNumber")?.as_str()?.parse::<u64>().ok()?;
                let from = item.get("from")?.as_str()?.to_string();
                let to = item.get("to")?.as_str()?.to_string();
                let value_str = item.get("value")?.as_str()?.to_string();
                let gas_limit = item
                    .get("gas")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
                let gas_used = item
                    .get("gasUsed")
                    .and_then(|v| v.as_str())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(0);
                let tx_type = item
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("call")
                    .to_string();
                let timestamp = item.get("timeStamp")?.as_str()?.parse::<u64>().ok()?;

                // Convert value from wei to ETH
                let value_wei = U256::from_dec_str(&value_str).ok()?;
                // OPTIMIZE: Use U256 directly, avoid string conversion
                const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000.0;
                let value_eth = value_wei.as_u128() as f64 / WEI_TO_ETH;

                Some(InternalTransaction {
                    parent_tx_hash,
                    block,
                    from,
                    to,
                    value: value_eth,
                    gas_limit,
                    gas_used,
                    tx_type,
                    timestamp,
                })
            })
            .collect();

        Ok(internal_txns)
    }

    /// Get token transfers for a specific transaction hash
    /// This gets transfers from both from and to addresses and filters by tx_hash
    pub async fn get_transaction_token_transfers(
        &self,
        _tx_hash: &str,
    ) -> Result<Vec<TokenTransfer>> {
        // Get transfers from the transaction's from address and filter by tx_hash
        // We'll need to get the transaction first to know the addresses
        // For now, return empty - will be populated in service layer
        Ok(vec![])
    }

    /// Get internal transactions for a specific transaction hash
    pub async fn get_transaction_internal_transactions(
        &self,
        _tx_hash: &str,
    ) -> Result<Vec<InternalTransaction>> {
        // Get internal transactions from the transaction's from address and filter by parent_tx_hash
        // For now, return empty - will be populated in service layer
        Ok(vec![])
    }

    /// Get token balances for an address via Etherscan V2
    pub async fn get_token_balances(&self, address: &str) -> Result<Vec<TokenBalance>> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "account".to_string()),
                ("action", "tokenlist".to_string()),
                ("address", address.to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(Error::serialization)?;

        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;
        let arr = result
            .as_array()
            .ok_or_else(|| Error::parse("Unexpected result type for tokenlist"))?;

        let tokens: Vec<TokenBalance> = arr
            .iter()
            .filter_map(|item| {
                let contract_address = item.get("contractAddress")?.as_str()?.to_string();
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let symbol = item
                    .get("symbol")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let decimals_str = item
                    .get("decimals")
                    .and_then(|v| v.as_str())
                    .unwrap_or("18");
                let decimals = decimals_str.parse::<u8>().unwrap_or(18);
                let balance_str = item.get("balance")?.as_str()?.to_string();

                // Convert balance with proper decimals
                let balance_wei = U256::from_dec_str(&balance_str).ok()?;
                let divisor = 10_u64.pow(decimals as u32) as f64;
                // OPTIMIZE: Use U256 directly, avoid string conversion
                let balance = balance_wei.as_u128() as f64 / divisor;

                Some(TokenBalance {
                    contract_address,
                    name,
                    symbol,
                    decimals,
                    balance,
                })
            })
            .collect();

        Ok(tokens)
    }
}

/// Transaction details from Etherscan API
#[derive(Debug, Clone)]
pub struct EtherscanTransactionDetails {
    pub hash: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub from: String,
    pub to: Option<String>,
    pub value: f64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub gas_price: u64, // In gwei
    pub nonce: u64,
    pub transaction_index: Option<u64>,
    pub input_data: String,
    pub is_error: bool,
    pub contract_address: Option<String>,
}

impl EtherscanClient {
    /// Get transaction details via Etherscan V2
    pub async fn get_transaction_details(
        &self,
        tx_hash: &str,
    ) -> Result<EtherscanTransactionDetails> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "proxy".to_string()),
                ("action", "eth_getTransactionByHash".to_string()),
                ("txhash", tx_hash.to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(Error::serialization)?;

        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;

        // Parse transaction data
        let hash = result
            .get("hash")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::parse("Missing hash field"))?
            .to_string();

        let block_number_str = result
            .get("blockNumber")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0");
        let block_number = u64::from_str_radix(block_number_str.trim_start_matches("0x"), 16)
            .map_err(|e| Error::parse(format!("Failed to parse block number: {}", e)))?;

        let _timestamp = 0; // Will be fetched separately if needed

        let from = result
            .get("from")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::parse("Missing from field"))?
            .to_string();

        let to = result
            .get("to")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let value_str = result
            .get("value")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0");
        let value_wei = U256::from_str_radix(value_str.trim_start_matches("0x"), 16)
            .map_err(|e| Error::parse(format!("Failed to parse value: {}", e)))?;
        // OPTIMIZE: Use U256 directly, avoid string conversion
        const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000.0;
        let value = value_wei.as_u128() as f64 / WEI_TO_ETH;

        let gas_str = result.get("gas").and_then(|v| v.as_str()).unwrap_or("0x0");
        let gas_limit = u64::from_str_radix(gas_str.trim_start_matches("0x"), 16)
            .map_err(|e| Error::parse(format!("Failed to parse gas: {}", e)))?;

        let _gas_used = 0; // Will be fetched from receipt if needed

        let gas_price_str = result
            .get("gasPrice")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0");
        let gas_price_wei = U256::from_str_radix(gas_price_str.trim_start_matches("0x"), 16)
            .map_err(|e| Error::parse(format!("Failed to parse gas price: {}", e)))?;
        // Convert to gwei (1 gwei = 10^9 wei)
        // OPTIMIZE: Use U256 directly, avoid string conversion
        const GWEI_TO_ETH: f64 = 1_000_000_000.0;
        let gas_price = (gas_price_wei.as_u128() as f64 / GWEI_TO_ETH) as u64;

        let nonce_str = result
            .get("nonce")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0");
        let nonce = u64::from_str_radix(nonce_str.trim_start_matches("0x"), 16)
            .map_err(|e| Error::parse(format!("Failed to parse nonce: {}", e)))?;

        let transaction_index_str = result
            .get("transactionIndex")
            .and_then(|v| v.as_str())
            .map(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap_or(0));

        let input_data = result
            .get("input")
            .and_then(|v| v.as_str())
            .unwrap_or("0x")
            .to_string();

        // Check if this is a contract creation (to is None and input data is not empty)
        let contract_address = if to.is_none() && !input_data.is_empty() {
            // For contract creation, we'd need to get the receipt, but for now return None
            None
        } else {
            None
        };

        // Get transaction receipt to check status and get gas used
        let receipt = self.get_transaction_receipt(tx_hash).await?;
        let (gas_used, is_error) = if let Some(receipt) = receipt {
            let gas_used_str = receipt
                .get("gasUsed")
                .and_then(|v| v.as_str())
                .unwrap_or("0x0");
            let gas_used =
                u64::from_str_radix(gas_used_str.trim_start_matches("0x"), 16).unwrap_or(0);
            let status_str = receipt
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("0x1");
            let is_error = status_str == "0x0";
            (gas_used, is_error)
        } else {
            (0, false)
        };

        // Get block timestamp
        let timestamp = if block_number > 0 {
            self.get_block_timestamp(block_number).await.unwrap_or(0)
        } else {
            0
        };

        Ok(EtherscanTransactionDetails {
            hash,
            block_number,
            timestamp,
            from,
            to,
            value,
            gas_limit,
            gas_used,
            gas_price,
            nonce,
            transaction_index: transaction_index_str,
            input_data,
            is_error,
            contract_address,
        })
    }

    /// Get transaction receipt
    async fn get_transaction_receipt(&self, tx_hash: &str) -> Result<Option<serde_json::Value>> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "proxy".to_string()),
                ("action", "eth_getTransactionReceipt".to_string()),
                ("txhash", tx_hash.to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Ok(None);
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(Error::serialization)?;

        let result = json.get("result").cloned();
        Ok(result)
    }

    /// Get block timestamp
    async fn get_block_timestamp(&self, block_number: u64) -> Result<u64> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self
            .client
            .get(url)
            .query(&[
                ("chainid", chain_id.to_string()),
                ("module", "proxy".to_string()),
                ("action", "eth_getBlockByNumber".to_string()),
                ("tag", format!("0x{:x}", block_number)),
                ("boolean", "false".to_string()),
                ("apikey", self.api_key.clone()),
            ])
            .send()
            .await
            .map_err(|e| Error::network(format!("Etherscan request failed: {}", e)))?;

        if !resp.status().is_success() {
            return Err(Error::network(format!(
                "Etherscan HTTP error: {}",
                resp.status()
            )));
        }

        let text = resp
            .text()
            .await
            .map_err(|e| Error::network(format!("Etherscan response read failed: {}", e)))?;
        let json: serde_json::Value = serde_json::from_str(&text).map_err(Error::serialization)?;

        let result = json
            .get("result")
            .ok_or_else(|| Error::parse("Missing result field"))?;

        let timestamp_str = result
            .get("timestamp")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0");
        let timestamp = u64::from_str_radix(timestamp_str.trim_start_matches("0x"), 16)
            .map_err(|e| Error::parse(format!("Failed to parse timestamp: {}", e)))?;

        Ok(timestamp)
    }
}

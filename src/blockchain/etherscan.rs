//! Etherscan V2 multichain client

use crate::error::{Error, Result};
use ethers::types::U256;
use reqwest::Client;
use crate::blockchain::types::{AddressTx, TransactionStatus};

/// Supported chains for Etherscan V2
#[derive(Debug, Clone, Copy)]
pub enum EtherscanChain {
    Ethereum,     // 1
    Goerli,       // 5
    Sepolia,      // 11155111
    Polygon,      // 137
    Arbitrum,     // 42161
    Optimism,     // 10
    Base,         // 8453
    Custom(u64),  // Any other chain supported by Etherscan-like explorers
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
        let client = Client::builder()
            .build()
            .expect("Failed to build HTTP client");
        Self { api_key, client, chain }
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
        let resp = self.client
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
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(Error::serialization)?;

        // Etherscan V2 returns data under `result` for many endpoints
        let result = json.get("result").ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;

        // Expect result to be a string representing a balance in wei
        let balance_str = result.as_str().ok_or_else(|| Error::parse("Unexpected result type for balance"))?;
        let balance = U256::from_dec_str(balance_str)
            .map_err(|e| Error::parse(format!("Failed to parse balance: {}", e)))?;

        Ok(balance)
    }

    /// Get normal transactions for an address via Etherscan V2
    pub async fn get_address_transactions(&self, address: &str) -> Result<Vec<AddressTx>> {
        let url = self.base_url();
        let chain_id = self.chain.chain_id();
        let resp = self.client
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
        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(Error::serialization)?;

        let result = json.get("result").ok_or_else(|| Error::parse("Missing result field from Etherscan response"))?;
        let arr = result.as_array().ok_or_else(|| Error::parse("Unexpected result type for txlist"))?;

        let txs: Vec<AddressTx> = arr.iter().filter_map(|item| {
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
                    item.get("input")
                        .and_then(|m| m.as_str())
                        .map(|input| {
                            // Use first 10 chars (method selector) if present
                            if input.len() >= 10 { input[..10].to_string() } else { input.to_string() }
                        })
                })
                .unwrap_or_default();

            // Convert values
            let value_eth = U256::from_dec_str(&value_wei)
                .map(|wei| {
                    // Convert wei to f64 ETH
                    let wei_f = wei.to_string().parse::<f64>().unwrap_or(0.0);
                    wei_f / 1_000_000_000_000_000_000.0
                })
                .unwrap_or(0.0);

            let gas_price_eth_per_gas = U256::from_dec_str(&gas_price_wei)
                .map(|wei| {
                    let wei_f = wei.to_string().parse::<f64>().unwrap_or(0.0);
                    wei_f / 1_000_000_000_000_000_000.0
                })
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
        }).collect();

        Ok(txs)
    }
}
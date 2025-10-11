//! Etherscan V2 multichain client

use crate::error::{Error, Result};
use ethers::types::U256;
use reqwest::Client;

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
}
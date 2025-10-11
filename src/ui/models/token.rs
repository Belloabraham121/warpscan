use serde::{Deserialize, Serialize};

/// Token transfer entry
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for TokenTransfer {
    fn default() -> Self {
        Self {
            token_id: None,
            txn_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab".to_string(),
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x2222222222222222222222222222222222222222".to_string(),
            token_name: "Tether USD".to_string(),
            token_symbol: "USDT".to_string(),
            amount: 100.0,
            timestamp: 1640995200,
        }
    }
}

/// Token information for tokens tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub token_type: TokenType,
    pub balance: f64,
    pub value_usd: f64,
    pub decimals: u8,
}

/// Token type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    ERC20,
    ERC721,  // NFT
    ERC1155, // Multi-token
    Other(String),
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self {
            contract_address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
            name: "Tether USD".to_string(),
            symbol: "USDT".to_string(),
            token_type: TokenType::ERC20,
            balance: 1000.0,
            value_usd: 1000.0,
            decimals: 6,
        }
    }
}
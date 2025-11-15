use serde::{Deserialize, Serialize};

/// Transaction information for the latest transactions section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: f64,
    pub gas_price: u64,
    pub gas_used: u64,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub block_number: u64,
    pub transaction_fee: f64,
}

/// Transfer entry for a transaction (ETH or token)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionTransfer {
    pub transfer_type: TransferType,
    pub from: String,
    pub to: String,
    pub value: f64,
    pub token_symbol: Option<String>,  // None for ETH transfers
    pub token_name: Option<String>,    // None for ETH transfers
    pub token_address: Option<String>, // None for ETH transfers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferType {
    ETH,
    Token,
    Internal, // Internal ETH transfer
}

/// Comprehensive transaction details for the transaction viewer screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionDetails {
    pub hash: String,
    pub status: TransactionStatus,
    pub block_number: u64,
    pub timestamp: u64,
    pub from: String,
    pub to: Option<String>, // Can be None for contract creation
    pub value: f64,         // In ETH
    pub gas_limit: u64,
    pub gas_used: u64,
    pub gas_price: u64,       // In gwei
    pub transaction_fee: f64, // In ETH
    pub nonce: u64,
    pub transaction_index: Option<u64>,
    pub input_data: String,               // Hex string
    pub method: Option<String>,           // Decoded method name if available
    pub contract_address: Option<String>, // If this is a contract creation
    pub confirmations: u64,
    pub transfers: Vec<TransactionTransfer>, // All transfers in this transaction
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

impl Default for TransactionInfo {
    fn default() -> Self {
        Self {
            hash: "0xabcdef1234567890...".to_string(),
            from: "0x1234567890abcdef...".to_string(),
            to: "0xfedcba0987654321...".to_string(),
            value: 1.5,
            gas_price: 25,
            gas_used: 21_000,
            status: TransactionStatus::Success,
            timestamp: 1640995200,
            block_number: 21_234_567,
            transaction_fee: 0.000525,
        }
    }
}

impl Default for TransactionDetails {
    fn default() -> Self {
        Self {
            hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
                .to_string(),
            status: TransactionStatus::Success,
            block_number: 21_234_567,
            timestamp: 1640995200,
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: Some("0x2222222222222222222222222222222222222222".to_string()),
            value: 1.5,
            gas_limit: 21_000,
            gas_used: 21_000,
            gas_price: 25, // gwei
            transaction_fee: 0.000525,
            nonce: 42,
            transaction_index: Some(5),
            input_data: "0x".to_string(),
            method: Some("transfer".to_string()),
            contract_address: None,
            confirmations: 1234,
            transfers: Vec::new(),
        }
    }
}

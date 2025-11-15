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

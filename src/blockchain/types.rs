//! Blockchain types and data structures

use ethers::types::U256;

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

/// Simplified address transaction model used by services
#[derive(Debug, Clone)]
pub struct AddressTx {
    pub tx_hash: String,
    pub method: String,
    pub block_number: u64,
    pub timestamp: u64,
    pub from: String,
    pub to: String,
    pub value_eth: f64,
    pub fee_eth: f64,
    pub status: TransactionStatus,
}

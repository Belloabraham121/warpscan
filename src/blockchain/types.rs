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
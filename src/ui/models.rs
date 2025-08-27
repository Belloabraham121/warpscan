//! Data models for UI components
//!
//! This module contains data structures used by the UI to display blockchain information.

use serde::{Deserialize, Serialize};


/// Network statistics displayed on the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub ethereum_price: f64,
    pub market_cap: f64,
    pub latest_block: u64,
    pub transactions_count: u64,
    pub gas_price: u64,
    pub network_utilization: f64,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            ethereum_price: 2048.75,
            market_cap: 246_000_000_000.0,
            latest_block: 21_234_567,
            transactions_count: 1_234_567_890,
            gas_price: 25,
            network_utilization: 0.75,
        }
    }
}

/// Block information for the latest blocks section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
    pub miner: String,
    pub transaction_count: u32,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub size: u64,
    pub reward: f64,
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            number: 21_234_567,
            hash: "0x1234567890abcdef...".to_string(),
            timestamp: 1640995200, // Example timestamp
            miner: "0xabcdef1234567890...".to_string(),
            transaction_count: 150,
            gas_used: 15_000_000,
            gas_limit: 30_000_000,
            size: 50_000,
            reward: 2.5,
        }
    }
}

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

/// Dashboard data containing all information for the main screen
#[derive(Debug, Clone, Default)]
pub struct DashboardData {
    pub network_stats: NetworkStats,
    pub latest_blocks: Vec<BlockInfo>,
    pub latest_transactions: Vec<TransactionInfo>,
    pub search_results: Option<SearchResult>,
}

/// Search result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResult {
    Block(BlockInfo),
    Transaction(TransactionInfo),
    Address(AddressInfo),
    NotFound,
}

/// Address information for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: String,
    pub balance: f64,
    pub transaction_count: u64,
    pub is_contract: bool,
    pub contract_name: Option<String>,
}

impl Default for AddressInfo {
    fn default() -> Self {
        Self {
            address: "0x1234567890abcdef...".to_string(),
            balance: 10.5,
            transaction_count: 1_234,
            is_contract: false,
            contract_name: None,
        }
    }
}

/// Generate mock data for development
impl DashboardData {
    pub fn mock() -> Self {
        let mut latest_blocks = Vec::new();
        for i in 0..5 {
            let mut block = BlockInfo::default();
            block.number = 21_234_567 - i;
            block.hash = format!("0x{:016x}...", 0x1234567890abcdef - i * 0x1111);
            block.transaction_count = 150 - (i as u32 * 10);
            latest_blocks.push(block);
        }

        let mut latest_transactions = Vec::new();
        for i in 0..5 {
            let mut tx = TransactionInfo::default();
            tx.hash = format!("0x{:016x}...", 0xabcdef1234567890u64 - i as u64 * 0x2222);
            tx.value = 1.5 - (i as f64 * 0.1);
            tx.block_number = 19000000 + i as u64;
            tx.from = format!("0x{:016x}...", 0x1111111111111111u64 + i as u64 * 0x1000);
            tx.to = format!("0x{:016x}...", 0x2222222222222222u64 + i as u64 * 0x1000);
            latest_transactions.push(tx);
        }

        Self {
            network_stats: NetworkStats::default(),
            latest_blocks,
            latest_transactions,
            search_results: None,
        }
    }
}
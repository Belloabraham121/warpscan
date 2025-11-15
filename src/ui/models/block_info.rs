use serde::{Deserialize, Serialize};

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

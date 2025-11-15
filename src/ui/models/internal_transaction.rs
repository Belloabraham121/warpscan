use serde::{Deserialize, Serialize};

/// Internal transaction entry
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for InternalTransaction {
    fn default() -> Self {
        Self {
            parent_tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
                .to_string(),
            block: 21_234_567,
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x2222222222222222222222222222222222222222".to_string(),
            value: 0.5,
            gas_limit: 21_000,
            gas_used: 21_000,
            tx_type: "call".to_string(),
            timestamp: 1640995200,
        }
    }
}

use serde::{Deserialize, Serialize};

/// Daily transaction data point for the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyTransactionData {
    pub date: String,
    pub transaction_count: u64,
    pub timestamp: u64,
}

impl Default for DailyTransactionData {
    fn default() -> Self {
        Self {
            date: "2024-01-01".to_string(),
            transaction_count: 1_500_000,
            timestamp: 1704067200, // 2024-01-01
        }
    }
}
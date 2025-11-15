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

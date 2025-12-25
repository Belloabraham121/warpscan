use super::super::models::BlockInfo;
use super::core::App;

impl App {
    /// Set loading state for an operation
    pub fn set_loading(&mut self, operation: &str, loading: bool) {
        self.loading_states.insert(operation.to_string(), loading);
    }

    /// Check if an operation is loading
    pub fn is_loading(&self, operation: &str) -> bool {
        self.loading_states.get(operation).copied().unwrap_or(false)
    }

    /// Set error message
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }

    /// Set success message
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }

    /// Cache data for a key
    pub fn cache_data(&mut self, key: String, data: serde_json::Value) {
        self.data_cache.insert(key, data);
    }

    /// Get cached data for a key
    pub fn get_cached_data(&self, key: &str) -> Option<&serde_json::Value> {
        self.data_cache.get(key)
    }

    /// Clear cached data
    pub fn clear_cache(&mut self) {
        self.data_cache.clear();
    }

    /// Refresh dashboard data with real blockchain data
    pub async fn refresh_dashboard(&mut self) {
        self.set_loading("dashboard_refresh", true);

        // Fetch latest block and network stats in parallel
        let (latest_block_result, block_number_result) = tokio::join!(
            self.blockchain_client.get_latest_block(),
            self.blockchain_client.get_block_number(),
        );

        // Update network stats
        if let Ok(Some(block)) = latest_block_result {
            if let Some(block_number) = block.number {
                let block_num = block_number.as_u64();
                self.dashboard_data.network_stats.latest_block = block_num;

                // Update block timestamp
                let timestamp = block.timestamp.as_u64();
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let seconds_ago = now.saturating_sub(timestamp);
                self.dashboard_data.network_stats.block_time = format!("{} secs ago", seconds_ago);
            }
        }

        if let Ok(block_number) = block_number_result {
            self.dashboard_data.network_stats.latest_block = block_number;
        }

        // Fetch latest blocks
        if let Ok(Some(latest_block)) = self.blockchain_client.get_latest_block().await {
            if let Some(block_number) = latest_block.number {
                let mut blocks = Vec::new();
                let start_block = block_number.as_u64();

                // Fetch last 5 blocks
                for i in 0..5 {
                    if let Ok(Some(block)) = self
                        .blockchain_client
                        .get_block_by_number(start_block.saturating_sub(i))
                        .await
                    {
                        if let Some(num) = block.number {
                            let block_info = BlockInfo {
                                number: num.as_u64(),
                                hash: format!("{:?}", block.hash),
                                transaction_count: block.transactions.len() as u32,
                                timestamp: block.timestamp.as_u64(),
                                gas_limit: block.gas_limit.as_u64(),
                                gas_used: block.gas_used.as_u64(),
                                miner: format!("{:?}", block.author.unwrap_or_default()),
                                size: 0,     // Size not available from RPC
                                reward: 0.0, // Reward not available from RPC
                            };
                            blocks.push(block_info);
                        }
                    }
                }
                self.dashboard_data.latest_blocks = blocks;
            }
        }

        self.set_loading("dashboard_refresh", false);
    }
}

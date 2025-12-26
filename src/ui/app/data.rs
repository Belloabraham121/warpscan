use super::super::models::{BlockInfo, TransactionInfo};
use super::core::App;
use crate::ui::models::TransactionStatus;

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

        // Fetch latest blocks and collect transaction hashes
        let mut all_tx_hashes = Vec::new();
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
                            let block_num = num.as_u64();
                            let block_info = BlockInfo {
                                number: block_num,
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

                            // Collect transaction hashes from this block
                            for tx_hash in &block.transactions {
                                all_tx_hashes.push((
                                    format!("{:?}", tx_hash),
                                    block_num,
                                    block.timestamp.as_u64(),
                                ));
                            }
                        }
                    }
                }
                self.dashboard_data.latest_blocks = blocks;
            }
        }

        // Fetch latest transactions from the collected hashes
        // Sort by block number (descending) and take latest 5
        all_tx_hashes.sort_by(|a, b| b.1.cmp(&a.1));
        all_tx_hashes.truncate(5);

        let mut transactions = Vec::new();
        for (tx_hash_str, block_num, block_timestamp) in all_tx_hashes {
            // Fetch transaction details
            if let Ok(Some(tx)) = self
                .blockchain_client
                .get_transaction_by_hash(&tx_hash_str)
                .await
            {
                // Fetch transaction receipt for gas_used and status
                let receipt_result = self
                    .blockchain_client
                    .get_transaction_receipt(&tx_hash_str)
                    .await;

                let (gas_used, status, gas_price_gwei) = if let Ok(Some(receipt)) = receipt_result {
                    let gas_used_val = receipt.gas_used.map(|g| g.as_u128() as u64).unwrap_or(0);
                    let status_val = if receipt.status == Some(1.into()) {
                        TransactionStatus::Success
                    } else if receipt.status == Some(0.into()) {
                        TransactionStatus::Failed
                    } else {
                        TransactionStatus::Pending
                    };
                    let gas_price_val = receipt
                        .effective_gas_price
                        .map(|p| p.as_u64() / 1_000_000_000) // Convert to gwei
                        .unwrap_or_else(|| {
                            // Fallback to transaction gas_price if effective_gas_price not available
                            tx.gas_price
                                .map(|p| p.as_u64() / 1_000_000_000)
                                .unwrap_or(0)
                        });
                    (gas_used_val, status_val, gas_price_val)
                } else {
                    // Fallback if receipt not available (pending transaction)
                    let gas_price_val = tx
                        .gas_price
                        .map(|p| p.as_u64() / 1_000_000_000)
                        .unwrap_or(0);
                    (0, TransactionStatus::Pending, gas_price_val)
                };

                // Convert value from wei to ETH
                const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000.0;
                let value_eth = tx.value.as_u128() as f64 / WEI_TO_ETH;

                // Calculate transaction fee (gas_used * gas_price in ETH)
                let tx_fee_eth = (gas_used as f64 * gas_price_gwei as f64) / 1_000_000_000.0;

                let tx_info = TransactionInfo {
                    hash: tx_hash_str.clone(),
                    from: format!("{:?}", tx.from),
                    to: tx.to.map(|a| format!("{:?}", a)).unwrap_or_default(),
                    value: value_eth,
                    gas_price: gas_price_gwei,
                    gas_used,
                    status,
                    timestamp: block_timestamp,
                    block_number: block_num,
                    transaction_fee: tx_fee_eth,
                };

                transactions.push(tx_info);
            }
        }

        // Sort transactions by block number (descending) to show most recent first
        transactions.sort_by(|a, b| b.block_number.cmp(&a.block_number));
        self.dashboard_data.latest_transactions = transactions;

        self.set_loading("dashboard_refresh", false);
    }
}

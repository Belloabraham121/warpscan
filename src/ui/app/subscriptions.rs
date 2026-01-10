//! Subscription management for real-time updates

use super::core::App;
use crate::blockchain::SubscriptionEvent;
use crate::error::Result;
use hex;

impl App {
    /// Start subscriptions based on current state
    pub async fn start_subscriptions(&mut self) -> Result<()> {
        // Yield to allow UI to update before starting subscriptions
        tokio::task::yield_now().await;

        let subscription_manager = match self.blockchain_client.subscription_manager() {
            Some(manager) => manager,
            None => {
                tracing::warn!(target: "warpscan", "Subscription manager not available");
                return Ok(());
            }
        };

        let mut manager = subscription_manager.lock().await;

        match self.state {
            crate::ui::app::state::AppState::Home => {
                // Subscribe to new blocks for homepage
                manager
                    .subscribe_to_blocks("homepage_blocks".to_string())
                    .await?;
                tracing::info!(target: "warpscan", "Started block subscription for homepage");
            }
            crate::ui::app::state::AppState::AddressLookup => {
                // Subscribe to address transactions if address data exists
                if let Some(ref address_data) = self.address_data {
                    let subscription_id = format!("address_{}", address_data.details.address);
                    manager
                        .subscribe_to_address(
                            subscription_id.clone(),
                            &address_data.details.address,
                        )
                        .await?;
                    tracing::info!(
                        target: "warpscan",
                        "Started address subscription for {}",
                        address_data.details.address
                    );
                }
            }
            _ => {
                // No subscriptions needed for other screens
            }
        }

        Ok(())
    }

    /// Stop all subscriptions
    pub async fn stop_all_subscriptions(&mut self) {
        let subscription_manager = match self.blockchain_client.subscription_manager() {
            Some(manager) => manager,
            None => return,
        };

        let mut manager = subscription_manager.lock().await;
        manager.unsubscribe_all();
        tracing::info!(target: "warpscan", "Stopped all subscriptions");
    }

    /// Stop subscriptions for current state
    pub async fn stop_current_subscriptions(&mut self) {
        let subscription_manager = match self.blockchain_client.subscription_manager() {
            Some(manager) => manager,
            None => return,
        };

        let mut manager = subscription_manager.lock().await;

        match self.state {
            crate::ui::app::state::AppState::Home => {
                manager.unsubscribe("homepage_blocks");
            }
            crate::ui::app::state::AppState::AddressLookup => {
                if let Some(ref address_data) = self.address_data {
                    let subscription_id = format!("address_{}", address_data.details.address);
                    manager.unsubscribe(&subscription_id);
                }
            }
            _ => {}
        }
    }

    /// Handle real-time subscription event
    pub async fn handle_subscription_event(&mut self, event: SubscriptionEvent) {
        tracing::info!(
            target: "warpscan",
            "🎯 handle_subscription_event called: {:?}, current_state={:?}",
            event,
            self.state
        );
        match event {
            SubscriptionEvent::NewBlock {
                block_number,
                block_hash,
            } => {
                // Always update dashboard data for new blocks, regardless of current screen.
                // This keeps the Home dashboard in sync even when the user is viewing other screens.
                self.handle_new_block(block_number, block_hash).await;
            }
            SubscriptionEvent::NewAddressTransaction {
                address,
                transaction,
                block_number,
            } => {
                // Update address lookup with new transaction
                if self.state == crate::ui::app::state::AppState::AddressLookup {
                    if let Some(ref address_data) = self.address_data {
                        if address_data.details.address.to_lowercase() == address.to_lowercase() {
                            self.handle_new_address_transaction(transaction, block_number)
                                .await;
                        }
                    }
                }
            }
            SubscriptionEvent::PendingTransaction { transaction } => {
                // Handle pending transaction (could show notification)
                tracing::debug!(
                    target: "warpscan",
                    "Pending transaction: {:?}",
                    transaction.hash
                );
            }
            SubscriptionEvent::NewLog { log } => {
                // Handle new log/event
                tracing::debug!(target: "warpscan", "New log: {:?}", log);
            }
            SubscriptionEvent::Error {
                subscription_id,
                error,
            } => {
                tracing::error!(
                    target: "warpscan",
                    "Subscription error for {}: {}",
                    subscription_id,
                    error
                );
                self.set_error(format!("Subscription error: {}", error));
            }
        }
    }

    /// Handle new block event
    ///
    /// This updates:
    /// - Latest blocks list
    /// - Network stats (latest block + age)
    /// - Latest transactions list (incrementally, from this block only)
    async fn handle_new_block(&mut self, block_number: u64, _block_hash: ethers::types::H256) {
        tracing::info!(
            target: "warpscan",
            "🔄 handle_new_block called: block_number={}",
            block_number
        );
        // Fetch the new block and add to dashboard
        if let Ok(Some(block)) = self
            .blockchain_client
            .get_block_by_number(block_number)
            .await
        {
            use crate::ui::models::{BlockInfo, TransactionInfo, TransactionStatus};

            if let Some(num) = block.number {
                let block_num = num.as_u64();
                let block_timestamp = block.timestamp.as_u64();

                let block_info = BlockInfo {
                    number: block_num,
                    hash: block
                        .hash
                        .map(|h| format!("{:#x}", h))
                        .unwrap_or_else(|| "0x0".to_string()),
                    transaction_count: block.transactions.len() as u32,
                    timestamp: block_timestamp,
                    gas_limit: block.gas_limit.as_u64(),
                    gas_used: block.gas_used.as_u64(),
                    miner: block
                        .author
                        .map(|a| format!("{:#x}", a))
                        .unwrap_or_else(|| "0x0".to_string()),
                    size: 0,
                    reward: 0.0,
                };

                // Prepend new block to the list
                self.dashboard_data.latest_blocks.insert(0, block_info);
                // Keep only last 5 blocks
                if self.dashboard_data.latest_blocks.len() > 5 {
                    self.dashboard_data.latest_blocks.truncate(5);
                }

                // Update network stats
                self.dashboard_data.network_stats.latest_block = block_num;
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let seconds_ago = now.saturating_sub(block_timestamp);
                self.dashboard_data.network_stats.block_time = format!("{} secs ago", seconds_ago);

                // Incrementally update latest transactions from this block
                // Avoid duplicates by checking existing hashes
                let mut new_txs: Vec<TransactionInfo> = Vec::new();

                // Helper to check if a hash already exists in the latest transactions list
                let existing_hashes: std::collections::HashSet<String> = self
                    .dashboard_data
                    .latest_transactions
                    .iter()
                    .map(|t| t.hash.clone())
                    .collect();

                for tx_hash in &block.transactions {
                    let tx_hash_str = format!("{:#x}", tx_hash);

                    // Skip if we already know about this transaction
                    if existing_hashes.contains(&tx_hash_str) {
                        continue;
                    }

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

                        let (gas_used, status, gas_price_gwei) =
                            if let Ok(Some(receipt)) = receipt_result {
                                let gas_used_val =
                                    receipt.gas_used.map(|g| g.as_u128() as u64).unwrap_or(0);
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
                        let tx_fee_eth =
                            (gas_used as f64 * gas_price_gwei as f64) / 1_000_000_000.0;

                        let tx_info = TransactionInfo {
                            hash: tx_hash_str.clone(),
                            from: format!("{:#x}", tx.from),
                            to: tx.to.map(|a| format!("{:#x}", a)).unwrap_or_default(),
                            value: value_eth,
                            gas_price: gas_price_gwei,
                            gas_used,
                            status,
                            timestamp: block_timestamp,
                            block_number: block_num,
                            transaction_fee: tx_fee_eth,
                        };

                        new_txs.push(tx_info);
                    }
                }

                if !new_txs.is_empty() {
                    // Prepend new transactions and keep a small, recent window
                    // (we keep them sorted by block_number desc)
                    self.dashboard_data
                        .latest_transactions
                        .extend(new_txs.into_iter());

                    self.dashboard_data
                        .latest_transactions
                        .sort_by(|a, b| b.block_number.cmp(&a.block_number));

                    // Keep only the latest N transactions (e.g. 20 for a richer view)
                    const MAX_LATEST_TXS: usize = 20;
                    if self.dashboard_data.latest_transactions.len() > MAX_LATEST_TXS {
                        self.dashboard_data
                            .latest_transactions
                            .truncate(MAX_LATEST_TXS);
                    }
                }

                tracing::debug!(
                    target: "warpscan",
                    "New block {} received, updated latest blocks and transactions",
                    block_num
                );
            }
        }
    }

    /// Handle new address transaction event
    ///
    /// This incrementally updates:
    /// - Transactions tab
    /// - Account history tab
    /// - Basic address details (total tx count, last activity)
    async fn handle_new_address_transaction(
        &mut self,
        transaction: ethers::types::Transaction,
        block_number: u64,
    ) {
        tracing::info!(
            target: "warpscan",
            "🔄 handle_new_address_transaction called: tx_hash={:#x}, block_number={}",
            transaction.hash(),
            block_number
        );
        use crate::ui::models::{AccountHistoryEntry, AddressTransaction, TransactionStatus};

        // We only update if we have address data loaded
        let address_data = match self.address_data.as_mut() {
            Some(data) => data,
            None => {
                tracing::debug!(
                    target: "warpscan",
                    "Received address transaction event but address_data is None"
                );
                return;
            }
        };

        let address_lower = address_data.details.address.to_lowercase();

        // Convert ethers transaction to AddressTransaction
        let tx_hash = format!("{:#x}", transaction.hash());
        let value_eth = transaction.value.as_u128() as f64 / 1_000_000_000_000_000_000.0;

        // Fetch receipt for status
        let receipt_result = self
            .blockchain_client
            .get_transaction_receipt(&tx_hash)
            .await;

        let (status, fee) = if let Ok(Some(receipt)) = receipt_result {
            let status_val = if receipt.status == Some(1.into()) {
                TransactionStatus::Success
            } else if receipt.status == Some(0.into()) {
                TransactionStatus::Failed
            } else {
                TransactionStatus::Pending
            };
            let fee = receipt
                .gas_used
                .and_then(|g| {
                    receipt
                        .effective_gas_price
                        .map(|p| (g.as_u128() as f64 * p.as_u64() as f64) / 1_000_000_000.0)
                })
                .unwrap_or(0.0);
            (status_val, fee)
        } else {
            (TransactionStatus::Pending, 0.0)
        };

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let address_tx = AddressTransaction {
            tx_hash: tx_hash.clone(),
            tx_type: if transaction.to.is_none() {
                "Contract Creation".to_string()
            } else {
                "Transfer".to_string()
            },
            method: if transaction.input.len() >= 4 {
                format!(
                    "0x{}",
                    hex::encode(&transaction.input[..4.min(transaction.input.len())])
                )
            } else {
                String::new()
            },
            block: block_number,
            from: format!("{:#x}", transaction.from),
            to: transaction
                .to
                .map(|a| format!("{:#x}", a))
                .unwrap_or_default(),
            value: value_eth,
            fee,
            timestamp,
            status,
        };

        // 1) Update transactions tab (prepend, with reasonable cap)
        address_data.transactions.insert(0, address_tx.clone());
        if address_data.transactions.len() > 200 {
            address_data.transactions.truncate(200);
        }

        // 2) Update account history tab
        let from_lower = address_tx.from.to_lowercase();
        let to_lower = address_tx.to.to_lowercase();
        let action = if from_lower == address_lower {
            "Sent"
        } else if to_lower == address_lower {
            "Received"
        } else {
            "Activity"
        };

        let age = "0s ago".to_string();

        let history_entry = AccountHistoryEntry {
            age,
            action: action.to_string(),
            from: address_tx.from.clone(),
            to: address_tx.to.clone(),
            timestamp,
            tx_hash: tx_hash.clone(),
        };

        address_data.account_history.insert(0, history_entry);
        if address_data.account_history.len() > 500 {
            address_data.account_history.truncate(500);
        }

        // 3) Update basic address details (cheap incremental stats)
        address_data.details.total_transactions =
            address_data.details.total_transactions.saturating_add(1);
        address_data.details.last_activity = timestamp;

        tracing::debug!(
            target: "warpscan",
            "New transaction {} received for address {} (block {})",
            tx_hash,
            address_data.details.address,
            block_number
        );
    }
}

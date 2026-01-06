//! Subscription management for real-time updates

use super::core::App;
use crate::blockchain::SubscriptionEvent;
use crate::error::Result;
use hex;

impl App {
    /// Start subscriptions based on current state
    pub async fn start_subscriptions(&mut self) -> Result<()> {
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
        match event {
            SubscriptionEvent::NewBlock {
                block_number,
                block_hash,
            } => {
                // Update homepage with new block
                if self.state == crate::ui::app::state::AppState::Home {
                    self.handle_new_block(block_number, block_hash).await;
                }
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
                            self.handle_new_address_transaction(transaction, block_number).await;
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
    async fn handle_new_block(&mut self, block_number: u64, _block_hash: ethers::types::H256) {
        // Fetch the new block and add to dashboard
        if let Ok(Some(block)) = self
            .blockchain_client
            .get_block_by_number(block_number)
            .await
        {
            use crate::ui::models::BlockInfo;

            if let Some(num) = block.number {
                let block_num = num.as_u64();
                let block_info = BlockInfo {
                    number: block_num,
                    hash: block.hash.map(|h| format!("{:#x}", h)).unwrap_or_else(|| "0x0".to_string()),
                    transaction_count: block.transactions.len() as u32,
                    timestamp: block.timestamp.as_u64(),
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
                let timestamp = block.timestamp.as_u64();
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let seconds_ago = now.saturating_sub(timestamp);
                self.dashboard_data.network_stats.block_time =
                    format!("{} secs ago", seconds_ago);

                // Fetch transactions from this block and update latest transactions
                // This is a simplified version - in production you'd want to be more selective
                tracing::debug!(
                    target: "warpscan",
                    "New block {} received, updating dashboard",
                    block_num
                );
            }
        }
    }

    /// Handle new address transaction event
    async fn handle_new_address_transaction(
        &mut self,
        transaction: ethers::types::Transaction,
        block_number: u64,
    ) {
        use crate::ui::models::{AddressTransaction, TransactionStatus};

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
                    receipt.effective_gas_price.map(|p| {
                        (g.as_u128() as f64 * p.as_u64() as f64) / 1_000_000_000.0
                    })
                })
                .unwrap_or(0.0);
            (status_val, fee)
        } else {
            (TransactionStatus::Pending, 0.0)
        };

        let address_tx = AddressTransaction {
            tx_hash: tx_hash.clone(),
            tx_type: if transaction.to.is_none() {
                "Contract Creation".to_string()
            } else {
                "Transfer".to_string()
            },
            method: if transaction.input.len() >= 4 {
                format!("0x{}", hex::encode(&transaction.input[..4.min(transaction.input.len())]))
            } else {
                String::new()
            },
            block: block_number,
            from: format!("{:#x}", transaction.from),
            to: transaction.to.map(|a| format!("{:#x}", a)).unwrap_or_default(),
            value: value_eth,
            fee,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status,
        };

        // Prepend to transactions list
        if let Some(ref mut address_data) = self.address_data {
            address_data.transactions.insert(0, address_tx);
            // Keep reasonable limit
            if address_data.transactions.len() > 100 {
                address_data.transactions.truncate(100);
            }

            // Update balance if needed (simplified - would need to fetch actual balance)
            tracing::debug!(
                target: "warpscan",
                "New transaction {} received for address",
                tx_hash
            );
        }
    }
}


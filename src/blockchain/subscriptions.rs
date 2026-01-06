//! Real-time subscription manager for blockchain events
//!
//! This module handles WebSocket subscriptions for real-time blockchain updates

use crate::error::{Error, Result};
use ethers::{
    providers::{Middleware, Provider, StreamExt, Ws},
    types::{Address, Log, Transaction, H256},
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/// Subscription handle for managing active subscriptions
pub struct SubscriptionHandle {
    handle: JoinHandle<()>,
    _sender: mpsc::UnboundedSender<SubscriptionEvent>,
}

impl SubscriptionHandle {
    pub fn new(handle: JoinHandle<()>, sender: mpsc::UnboundedSender<SubscriptionEvent>) -> Self {
        Self {
            handle,
            _sender: sender,
        }
    }

    pub fn abort(self) {
        self.handle.abort();
    }
}

/// Events emitted by subscriptions
#[derive(Debug, Clone)]
pub enum SubscriptionEvent {
    /// New block mined
    NewBlock { block_number: u64, block_hash: H256 },
    /// New transaction involving an address
    NewAddressTransaction {
        address: String,
        transaction: Transaction,
        block_number: u64,
    },
    /// New pending transaction
    PendingTransaction { transaction: Transaction },
    /// New log/event
    NewLog { log: Log },
    /// Subscription error
    Error {
        subscription_id: String,
        error: String,
    },
}

/// Subscription manager for real-time blockchain events
pub struct SubscriptionManager {
    /// WebSocket provider (if available)
    ws_provider: Option<Arc<Provider<Ws>>>,
    /// HTTP provider for fallback queries
    http_provider: Arc<Provider<ethers::providers::Http>>,
    /// Active subscriptions
    subscriptions: HashMap<String, SubscriptionHandle>,
    /// Event sender for broadcasting subscription events
    event_sender: mpsc::UnboundedSender<SubscriptionEvent>,
    /// Event receiver (dummy, actual receiver is returned separately)
    _event_receiver: mpsc::UnboundedReceiver<SubscriptionEvent>,
}

impl SubscriptionManager {
    /// Create a new subscription manager
    pub fn new(
        ws_provider: Option<Arc<Provider<Ws>>>,
        http_provider: Arc<Provider<ethers::providers::Http>>,
    ) -> (Self, mpsc::UnboundedReceiver<SubscriptionEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();

        // Create a dummy receiver for the struct (we return the real one separately)
        let (_dummy_sender, dummy_receiver) = mpsc::unbounded_channel();

        let manager = Self {
            ws_provider,
            http_provider,
            subscriptions: HashMap::new(),
            event_sender: sender.clone(),
            _event_receiver: dummy_receiver,
        };

        (manager, receiver)
    }

    /// Get the event sender for broadcasting events
    pub fn event_sender(&self) -> mpsc::UnboundedSender<SubscriptionEvent> {
        self.event_sender.clone()
    }

    /// Subscribe to new blocks
    pub async fn subscribe_to_blocks(&mut self, subscription_id: String) -> Result<()> {
        // Unsubscribe if already exists
        if let Some(handle) = self.subscriptions.remove(&subscription_id) {
            handle.abort();
        }

        let provider = if let Some(ref ws) = self.ws_provider {
            ws.clone()
        } else {
            // Fallback: use HTTP provider with polling (not ideal but works)
            tracing::warn!(
                target: "warpscan",
                "WebSocket not available, using HTTP polling for blocks"
            );
            return self.subscribe_to_blocks_polling(subscription_id).await;
        };

        let sender = self.event_sender.clone();
        let _id = subscription_id.clone();

        let handle = tokio::spawn(async move {
            match provider.watch_blocks().await {
                Ok(mut stream) => {
                    tracing::info!(target: "warpscan", "Subscribed to new blocks");
                    while let Some(block_hash) = stream.next().await {
                        // Fetch full block details
                        if let Ok(Some(block)) = provider.get_block(block_hash).await {
                            if let Some(block_number) = block.number {
                                let _ = sender.send(SubscriptionEvent::NewBlock {
                                    block_number: block_number.as_u64(),
                                    block_hash,
                                });
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::error!(target: "warpscan", "Failed to subscribe to blocks: {}", e);
                    let _ = sender.send(SubscriptionEvent::Error {
                        subscription_id: _id.clone(),
                        error: format!("Failed to subscribe to blocks: {}", e),
                    });
                }
            }
        });

        self.subscriptions.insert(
            subscription_id,
            SubscriptionHandle::new(handle, self.event_sender.clone()),
        );

        Ok(())
    }

    /// Fallback: Subscribe to blocks using HTTP polling
    async fn subscribe_to_blocks_polling(&mut self, subscription_id: String) -> Result<()> {
        let http_provider = self.http_provider.clone();
        let sender = self.event_sender.clone();
        let _id = subscription_id.clone();

        let handle = tokio::spawn(async move {
            let mut last_block = http_provider.get_block_number().await.ok();
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                if let Ok(current_block) = http_provider.get_block_number().await {
                    let current = current_block.as_u64();
                    if let Some(last) = last_block {
                        if current > last.as_u64() {
                            // New block detected
                            if let Ok(Some(block)) = http_provider
                                .get_block(ethers::types::BlockId::Number(
                                    ethers::types::BlockNumber::Number(current.into()),
                                ))
                                .await
                            {
                                if let Some(hash) = block.hash {
                                    let _ = sender.send(SubscriptionEvent::NewBlock {
                                        block_number: current,
                                        block_hash: hash,
                                    });
                                }
                            }
                        }
                    }
                    last_block = Some(current_block);
                }
            }
        });

        self.subscriptions.insert(
            subscription_id,
            SubscriptionHandle::new(handle, self.event_sender.clone()),
        );

        Ok(())
    }

    /// Subscribe to transactions involving a specific address
    pub async fn subscribe_to_address(
        &mut self,
        subscription_id: String,
        address: &str,
    ) -> Result<()> {
        // Unsubscribe if already exists
        if let Some(handle) = self.subscriptions.remove(&subscription_id) {
            handle.abort();
        }

        let addr = Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))?;

        let provider = if let Some(ref ws) = self.ws_provider {
            ws.clone()
        } else {
            // Fallback: use HTTP provider with polling
            tracing::warn!(
                target: "warpscan",
                "WebSocket not available, using HTTP polling for address {}",
                address
            );
            return self
                .subscribe_to_address_polling(subscription_id, address)
                .await;
        };

        let sender = self.event_sender.clone();
        let _id = subscription_id.clone();
        let address_str = address.to_string();

        // Subscribe to new blocks and filter for address transactions
        let block_handle = {
            let provider = provider.clone();
            let sender = sender.clone();
            let address_str = address_str.clone();
            // Capture addr for use in the closure
            let target_addr = addr;

            tokio::spawn(async move {
                match provider.watch_blocks().await {
                    Ok(mut stream) => {
                        tracing::info!(
                            target: "warpscan",
                            "Subscribed to blocks for address {}",
                            address_str
                        );
                        while let Some(block_hash) = stream.next().await {
                            if let Ok(Some(block)) = provider
                                .get_block_with_txs(ethers::types::BlockId::Hash(block_hash))
                                .await
                            {
                                if let Some(block_number) = block.number {
                                    let block_num = block_number.as_u64();
                                    for tx in block.transactions {
                                        if tx.from == target_addr || tx.to == Some(target_addr) {
                                            let _ = sender.send(
                                                SubscriptionEvent::NewAddressTransaction {
                                                    address: address_str.clone(),
                                                    transaction: tx,
                                                    block_number: block_num,
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            target: "warpscan",
                            "Failed to subscribe to blocks for address: {}",
                            e
                        );
                        let _ = sender.send(SubscriptionEvent::Error {
                            subscription_id: _id.clone(),
                            error: format!("Failed to subscribe to blocks: {}", e),
                        });
                    }
                }
            })
        };

        // Store both handles (we'll need to manage multiple handles per subscription)
        // For now, just store the block handle
        self.subscriptions.insert(
            subscription_id,
            SubscriptionHandle::new(block_handle, self.event_sender.clone()),
        );

        // Note: pending_handle is not stored, but will run in background
        // In a production system, you'd want to track both

        Ok(())
    }

    /// Fallback: Subscribe to address using HTTP polling
    async fn subscribe_to_address_polling(
        &mut self,
        subscription_id: String,
        address: &str,
    ) -> Result<()> {
        let http_provider = self.http_provider.clone();
        let sender = self.event_sender.clone();
        let _id = subscription_id.clone();
        let address_str = address.to_string();
        let addr = Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))?;

        let handle = tokio::spawn(async move {
            let mut last_block = http_provider.get_block_number().await.ok();
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                if let Ok(current_block) = http_provider.get_block_number().await {
                    let current = current_block.as_u64();
                    if let Some(last) = last_block {
                        if current > last.as_u64() {
                            // Scan new blocks for address transactions
                            for block_num in (last.as_u64() + 1)..=current {
                                if let Ok(Some(block)) = http_provider
                                    .get_block_with_txs(ethers::types::BlockId::Number(
                                        ethers::types::BlockNumber::Number(block_num.into()),
                                    ))
                                    .await
                                {
                                    for tx in block.transactions {
                                        if tx.from == addr || tx.to == Some(addr) {
                                            let _ = sender.send(
                                                SubscriptionEvent::NewAddressTransaction {
                                                    address: address_str.clone(),
                                                    transaction: tx,
                                                    block_number: block_num,
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                    last_block = Some(current_block);
                }
            }
        });

        self.subscriptions.insert(
            subscription_id,
            SubscriptionHandle::new(handle, self.event_sender.clone()),
        );

        Ok(())
    }

    /// Unsubscribe from a subscription
    pub fn unsubscribe(&mut self, subscription_id: &str) {
        if let Some(handle) = self.subscriptions.remove(subscription_id) {
            handle.abort();
            tracing::info!(
                target: "warpscan",
                "Unsubscribed from {}",
                subscription_id
            );
        }
    }

    /// Unsubscribe from all subscriptions
    pub fn unsubscribe_all(&mut self) {
        for (id, handle) in self.subscriptions.drain() {
            handle.abort();
            tracing::info!(target: "warpscan", "Unsubscribed from {}", id);
        }
    }

    /// Check if WebSocket is available
    pub fn has_websocket(&self) -> bool {
        self.ws_provider.is_some()
    }
}

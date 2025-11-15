use super::super::models::{AddressDetails, AddressTab, AddressType, CompleteAddressData};
use super::core::App;
use crate::blockchain::types::AddressTx as ServiceAddressTx;
use crate::blockchain::types::TransactionStatus as ChainTransactionStatus;

impl App {
    /// Lookup address information and populate address_data
    pub async fn lookup_address(&mut self, address: &str) -> crate::error::Result<()> {
        // Set loading state
        self.set_loading("address_search", true);
        self.clear_messages();

        // Get basic address info from blockchain
        match self.blockchain_client.get_address_info(address).await {
            Ok(address_info) => {
                // Determine address type based on contract status
                let address_type = if address_info.is_contract {
                    AddressType::Contract
                } else {
                    AddressType::EOA
                };

                // Convert balance from string to f64 (assuming it's in wei, convert to ETH)
                let balance_wei: Result<ethers::types::U256, _> = address_info.balance.parse();
                let balance_eth = match balance_wei {
                    Ok(wei) => {
                        // Convert wei to ETH (1 ETH = 10^18 wei)
                        let eth_value = wei.as_u128() as f64 / 1_000_000_000_000_000_000.0;
                        eth_value
                    }
                    Err(_) => 0.0,
                };

                // Create comprehensive address details
                let details = AddressDetails {
                    address: address.to_string(),
                    address_type,
                    balance: balance_eth,
                    token_count: 0,                   // TODO: Implement token counting
                    estimated_net_worth: balance_eth, // For now, just use ETH balance
                    total_transactions: address_info.transaction_count,
                    outgoing_transfers: 0,  // TODO: Implement transfer counting
                    total_gas_used: 0,      // TODO: Implement gas usage calculation
                    contract_name: None,    // TODO: Implement contract name resolution
                    contract_creator: None, // TODO: Implement contract creator lookup
                    creation_tx_hash: None, // TODO: Implement creation tx lookup
                    last_activity: chrono::Utc::now().timestamp() as u64, // TODO: Get actual last activity
                };

                // Create complete address data with empty collections for now
                // Fetch transactions via service
                let txs: Vec<ServiceAddressTx> = match self
                    .blockchain_client
                    .get_address_transactions(address)
                    .await
                {
                    Ok(txs) => txs,
                    Err(_) => Vec::new(),
                };

                // Map to UI model
                let ui_txs: Vec<super::super::models::AddressTransaction> = txs
                    .into_iter()
                    .map(|t| super::super::models::AddressTransaction {
                        tx_hash: t.tx_hash,
                        tx_type: if t.method.is_empty() {
                            "Transfer".to_string()
                        } else {
                            "Contract Call".to_string()
                        },
                        method: t.method,
                        block: t.block_number,
                        from: t.from,
                        to: t.to,
                        value: t.value_eth,
                        fee: t.fee_eth,
                        timestamp: t.timestamp,
                        status: match t.status {
                            ChainTransactionStatus::Pending => {
                                super::super::models::TransactionStatus::Pending
                            }
                            ChainTransactionStatus::Success => {
                                super::super::models::TransactionStatus::Success
                            }
                            ChainTransactionStatus::Failed => {
                                super::super::models::TransactionStatus::Failed
                            }
                            ChainTransactionStatus::Unknown => {
                                super::super::models::TransactionStatus::Pending
                            }
                        },
                    })
                    .collect();

                let complete_data = CompleteAddressData {
                    details,
                    transactions: ui_txs,
                    account_history: Vec::new(),
                    token_transfers: Vec::new(),
                    tokens: Vec::new(),
                    internal_transactions: Vec::new(),
                    current_tab: AddressTab::Transactions,
                    selected_transaction_index: 0,
                    selected_history_index: 0,
                    selected_token_transfer_index: 0,
                    selected_token_index: 0,
                    selected_internal_txn_index: 0,
                };

                self.address_data = Some(complete_data);
                self.set_success(format!("Address {} loaded successfully", address));
            }
            Err(e) => {
                self.set_error(format!("Failed to lookup address: {}", e));
            }
        }

        // Clear loading state
        self.set_loading("address_search", false);
        Ok(())
    }

    /// Switch to a different address tab
    pub fn switch_address_tab(&mut self, tab: AddressTab) {
        if let Some(ref mut address_data) = self.address_data {
            address_data.current_tab = tab;
            // Reset selection index when switching tabs
            address_data.selected_transaction_index = 0;
            address_data.selected_history_index = 0;
            address_data.selected_token_transfer_index = 0;
            address_data.selected_token_index = 0;
            address_data.selected_internal_txn_index = 0;
        }
    }

    /// Get the current address tab
    pub fn get_current_address_tab(&self) -> Option<AddressTab> {
        self.address_data
            .as_ref()
            .map(|data| data.current_tab.clone())
    }

    /// Move selection to previous transaction in the Transactions tab
    pub fn address_select_previous_transaction(&mut self) {
        if let Some(ref mut data) = self.address_data {
            if !data.transactions.is_empty() && data.selected_transaction_index > 0 {
                data.selected_transaction_index -= 1;
            }
        }
    }

    /// Move selection to next transaction in the Transactions tab
    pub fn address_select_next_transaction(&mut self) {
        if let Some(ref mut data) = self.address_data {
            if !data.transactions.is_empty() {
                let max_index = data.transactions.len().saturating_sub(1);
                if data.selected_transaction_index < max_index {
                    data.selected_transaction_index += 1;
                }
            }
        }
    }

    /// Navigate to an address (used for clicking on addresses)
    pub async fn navigate_to_address(&mut self, address: &str) {
        self.navigate_to(crate::ui::app::state::AppState::AddressLookup);
        self.set_input(address.to_string());
        if let Err(e) = self.lookup_address(address).await {
            self.set_error(format!("Failed to lookup address: {}", e));
        }
    }

    /// Navigate to a transaction (used for clicking on transaction hashes)
    pub fn navigate_to_transaction(&mut self, tx_hash: &str) {
        self.navigate_to(crate::ui::app::state::AppState::TransactionViewer);
        self.set_input(tx_hash.to_string());
        // TODO: Implement transaction lookup
        self.set_error("Transaction lookup not yet implemented".to_string());
    }

    /// Move selection to previous item in current tab
    pub fn address_select_previous_item(&mut self) {
        if let Some(ref mut data) = self.address_data {
            match data.current_tab {
                AddressTab::Transactions => {
                    if !data.transactions.is_empty() && data.selected_transaction_index > 0 {
                        data.selected_transaction_index -= 1;
                    }
                }
                AddressTab::AccountHistory => {
                    if !data.account_history.is_empty() && data.selected_history_index > 0 {
                        data.selected_history_index -= 1;
                    }
                }
                AddressTab::TokenTransfers => {
                    if !data.token_transfers.is_empty() && data.selected_token_transfer_index > 0 {
                        data.selected_token_transfer_index -= 1;
                    }
                }
                AddressTab::Tokens => {
                    if !data.tokens.is_empty() && data.selected_token_index > 0 {
                        data.selected_token_index -= 1;
                    }
                }
                AddressTab::InternalTxns => {
                    if !data.internal_transactions.is_empty()
                        && data.selected_internal_txn_index > 0
                    {
                        data.selected_internal_txn_index -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    /// Move selection to next item in current tab
    pub fn address_select_next_item(&mut self) {
        if let Some(ref mut data) = self.address_data {
            match data.current_tab {
                AddressTab::Transactions => {
                    if !data.transactions.is_empty() {
                        let max_index = data.transactions.len().saturating_sub(1);
                        if data.selected_transaction_index < max_index {
                            data.selected_transaction_index += 1;
                        }
                    }
                }
                AddressTab::AccountHistory => {
                    if !data.account_history.is_empty() {
                        let max_index = data.account_history.len().saturating_sub(1);
                        if data.selected_history_index < max_index {
                            data.selected_history_index += 1;
                        }
                    }
                }
                AddressTab::TokenTransfers => {
                    if !data.token_transfers.is_empty() {
                        let max_index = data.token_transfers.len().saturating_sub(1);
                        if data.selected_token_transfer_index < max_index {
                            data.selected_token_transfer_index += 1;
                        }
                    }
                }
                AddressTab::Tokens => {
                    if !data.tokens.is_empty() {
                        let max_index = data.tokens.len().saturating_sub(1);
                        if data.selected_token_index < max_index {
                            data.selected_token_index += 1;
                        }
                    }
                }
                AddressTab::InternalTxns => {
                    if !data.internal_transactions.is_empty() {
                        let max_index = data.internal_transactions.len().saturating_sub(1);
                        if data.selected_internal_txn_index < max_index {
                            data.selected_internal_txn_index += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

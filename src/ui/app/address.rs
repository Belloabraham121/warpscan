use super::super::models::{
    AccountHistoryEntry, AddressDetails, AddressTab, AddressType, CompleteAddressData,
    InternalTransaction, TokenInfo, TokenTransfer, TokenType,
};
use super::core::App;
use crate::blockchain::types::AddressTx as ServiceAddressTx;
use crate::blockchain::types::TransactionStatus as ChainTransactionStatus;

impl App {
    /// Lookup address information and populate address_data
    pub async fn lookup_address(&mut self, address: &str) -> crate::error::Result<()> {
        // Set loading state
        self.set_loading("address_search", true);
        self.clear_messages();

        // PARALLELIZE: Fetch ALL data concurrently
        // get_address_info, transactions, token transfers, token balances, internal transactions,
        // and ENS resolution can all run in parallel since they're independent
        let (
            address_info_result,
            ens_result,
            txs_result,
            token_transfers_result,
            token_balances_result,
            internal_transactions_result,
        ) = tokio::join!(
            // Fetch address info (balance, transaction count, contract status)
            self.blockchain_client.get_address_info(address),
            // Resolve ENS name (safe to call for all addresses, returns None for contracts/non-ENS)
            self.blockchain_client.resolve_ens_name(address),
            // Fetch transactions
            self.blockchain_client.get_address_transactions(address),
            // Fetch token transfers
            self.blockchain_client.get_token_transfers(address),
            // Fetch token balances
            self.blockchain_client.get_token_balances(address),
            // Fetch internal transactions
            self.blockchain_client.get_internal_transactions(address),
        );

        // Process address info result
        match address_info_result {
            Ok(address_info) => {
                // Determine address type based on contract status
                let address_type = if address_info.is_contract {
                    AddressType::Contract
                } else {
                    AddressType::EOA
                };

                // OPTIMIZE: Convert balance from string to f64 (assuming it's in wei, convert to ETH)
                // Use constant for division to avoid repeated calculation
                const WEI_TO_ETH: f64 = 1_000_000_000_000_000_000.0;
                let balance_eth = address_info
                    .balance
                    .parse::<ethers::types::U256>()
                    .map(|wei| wei.as_u128() as f64 / WEI_TO_ETH)
                    .unwrap_or(0.0);

                // Use ENS name only for EOA addresses
                let ens_name = match address_type {
                    AddressType::EOA => ens_result.unwrap_or(None),
                    _ => None,
                };

                // Create comprehensive address details
                let details = AddressDetails {
                    address: address.to_string(),
                    address_type,
                    balance: balance_eth,
                    token_count: 0, // Will be updated after fetching tokens
                    estimated_net_worth: balance_eth, // For now, just use ETH balance
                    total_transactions: address_info.transaction_count,
                    outgoing_transfers: 0,  // TODO: Implement transfer counting
                    total_gas_used: 0,      // TODO: Implement gas usage calculation
                    contract_name: None,    // TODO: Implement contract name resolution
                    contract_creator: None, // TODO: Implement contract creator lookup
                    creation_tx_hash: None, // TODO: Implement creation tx lookup
                    last_activity: chrono::Utc::now().timestamp() as u64, // TODO: Get actual last activity
                    ens_name,
                };

                // Process transactions
                let txs: Vec<ServiceAddressTx> = match txs_result {
                    Ok(txs) => txs,
                    Err(_) => Vec::new(),
                };

                // OPTIMIZE: Pre-compute address lowercase once to avoid repeated conversions
                let address_lower = address.to_lowercase();
                let now = chrono::Utc::now().timestamp() as u64;

                // OPTIMIZE: Pre-allocate vectors with known capacity
                let txs_len = txs.len();
                let mut ui_txs = Vec::with_capacity(txs_len);
                let mut account_history = Vec::with_capacity(txs_len);

                // OPTIMIZE: Process in single pass to avoid multiple iterations
                for t in &txs {
                    // Map to UI transaction model
                    let tx_type = if t.method.is_empty() {
                        "Transfer"
                    } else {
                        "Contract Call"
                    };
                    let status = match t.status {
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
                    };

                    ui_txs.push(super::super::models::AddressTransaction {
                        tx_hash: t.tx_hash.clone(),
                        tx_type: tx_type.to_string(),
                        method: t.method.clone(),
                        block: t.block_number,
                        from: t.from.clone(),
                        to: t.to.clone(),
                        value: t.value_eth,
                        fee: t.fee_eth,
                        timestamp: t.timestamp,
                        status,
                    });

                    // OPTIMIZE: Determine action based on address (use pre-computed lowercase)
                    // Pre-compute from/to lowercase once per transaction
                    let from_lower = t.from.to_lowercase();
                    let to_lower = t.to.to_lowercase();
                    let action = if from_lower == address_lower {
                        "Sent"
                    } else if to_lower == address_lower {
                        "Received"
                    } else {
                        "Unknown"
                    };

                    // Calculate age from timestamp (use pre-computed now)
                    let age_seconds = now.saturating_sub(t.timestamp);
                    let age = if age_seconds < 60 {
                        format!("{}s ago", age_seconds)
                    } else if age_seconds < 3600 {
                        format!("{}m ago", age_seconds / 60)
                    } else if age_seconds < 86400 {
                        format!("{}h ago", age_seconds / 3600)
                    } else {
                        format!("{}d ago", age_seconds / 86400)
                    };

                    account_history.push(AccountHistoryEntry {
                        age,
                        action: action.to_string(),
                        from: t.from.clone(),
                        to: t.to.clone(),
                        timestamp: t.timestamp,
                        tx_hash: t.tx_hash.clone(),
                    });
                }

                // OPTIMIZE: Process token transfers with pre-allocated capacity
                let token_transfers: Vec<TokenTransfer> = match token_transfers_result {
                    Ok(transfers) => {
                        let mut result = Vec::with_capacity(transfers.len());
                        for t in transfers {
                            result.push(TokenTransfer {
                                token_id: t.token_id,
                                txn_hash: t.txn_hash,
                                from: t.from,
                                to: t.to,
                                token_name: t.token_name,
                                token_symbol: t.token_symbol,
                                amount: t.amount,
                                timestamp: t.timestamp,
                            });
                        }
                        result
                    }
                    Err(_) => Vec::new(),
                };

                // OPTIMIZE: Process token balances with pre-allocated capacity
                let tokens: Vec<TokenInfo> = match token_balances_result {
                    Ok(balances) => {
                        let mut result = Vec::with_capacity(balances.len());
                        for b in balances {
                            result.push(TokenInfo {
                                contract_address: b.contract_address,
                                name: b.name,
                                symbol: b.symbol,
                                token_type: TokenType::ERC20, // Default to ERC20, could be enhanced
                                balance: b.balance,
                                value_usd: 0.0, // TODO: Fetch USD value from price API
                                decimals: b.decimals,
                            });
                        }
                        result
                    }
                    Err(_) => Vec::new(),
                };

                // Update token count in details
                let token_count = tokens.len() as u32;

                // OPTIMIZE: Process internal transactions with pre-allocated capacity
                let internal_transactions: Vec<InternalTransaction> =
                    match internal_transactions_result {
                        Ok(txns) => {
                            let mut result = Vec::with_capacity(txns.len());
                            for t in txns {
                                result.push(InternalTransaction {
                                    parent_tx_hash: t.parent_tx_hash,
                                    block: t.block,
                                    from: t.from,
                                    to: t.to,
                                    value: t.value,
                                    gas_limit: t.gas_limit,
                                    gas_used: t.gas_used,
                                    tx_type: t.tx_type,
                                    timestamp: t.timestamp,
                                });
                            }
                            result
                        }
                        Err(_) => Vec::new(),
                    };

                // Update details with token count
                let mut details = details;
                details.token_count = token_count;

                let complete_data = CompleteAddressData {
                    details,
                    transactions: ui_txs,
                    account_history,
                    token_transfers,
                    tokens,
                    internal_transactions,
                    current_tab: AddressTab::Details, // Default to Details tab
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
    pub async fn navigate_to_transaction(&mut self, tx_hash: &str) {
        self.navigate_to(crate::ui::app::state::AppState::TransactionViewer);
        self.set_input(tx_hash.to_string());
        self.input_data_expanded = false; // Reset expansion state

        // Clear previous transaction data
        self.transaction_data = None;
        self.set_loading("transaction_search", true);
        self.clear_messages();

        // Lookup transaction details
        match self
            .blockchain_client
            .get_transaction_details(tx_hash)
            .await
        {
            Ok(tx_details) => {
                self.transaction_data = Some(tx_details);
                self.set_success(format!("Transaction {} loaded successfully", tx_hash));
            }
            Err(e) => {
                self.set_error(format!("Failed to lookup transaction: {}", e));
            }
        }

        self.set_loading("transaction_search", false);
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

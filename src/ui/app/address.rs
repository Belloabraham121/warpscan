use super::core::App;
use super::super::models::{AddressDetails, AddressType, AddressTab, CompleteAddressData};

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
                    token_count: 0, // TODO: Implement token counting
                    estimated_net_worth: balance_eth, // For now, just use ETH balance
                    total_transactions: address_info.transaction_count,
                    outgoing_transfers: 0, // TODO: Implement transfer counting
                    total_gas_used: 0, // TODO: Implement gas usage calculation
                    contract_name: None, // TODO: Implement contract name resolution
                    contract_creator: None, // TODO: Implement contract creator lookup
                    creation_tx_hash: None, // TODO: Implement creation tx lookup
                    last_activity: chrono::Utc::now().timestamp() as u64, // TODO: Get actual last activity
                };

                // Create complete address data with empty collections for now
                let complete_data = CompleteAddressData {
                    details,
                    transactions: Vec::new(), // TODO: Fetch transaction history
                    account_history: Vec::new(), // TODO: Fetch account history
                    token_transfers: Vec::new(), // TODO: Fetch token transfers
                    tokens: Vec::new(), // TODO: Fetch token holdings
                    internal_transactions: Vec::new(), // TODO: Fetch internal transactions
                    current_tab: AddressTab::Details,
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
        }
    }

    /// Get the current address tab
    pub fn get_current_address_tab(&self) -> Option<AddressTab> {
        self.address_data.as_ref().map(|data| data.current_tab.clone())
    }
}
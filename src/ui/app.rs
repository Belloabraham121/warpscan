//! Main application state and logic

use crate::{
    blockchain::BlockchainService,
    cache::CacheManager,
    config::Config,
    wallet::WalletManager,
};
use super::models::{DashboardData, CompleteAddressData, AddressDetails, AddressType, AddressTab};
use ratatui::layout::Rect;
use std::collections::HashMap;
use tokio::sync::mpsc;

/// Application state
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Home,
    BlockExplorer,
    TransactionViewer,
    AddressLookup,
    ContractSearch,
    TokenInfo,
    GasTracker,
    ContractInteraction,
    ContractVerification,
    WalletManager,
    MultisigWallet,
    EventMonitor,
    Help,
    Quit,
}

/// Input mode for text fields
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

/// Main application struct
pub struct App {
    /// Current application state
    pub state: AppState,
    /// Previous state for navigation
    pub previous_state: Option<AppState>,
    /// Whether the application should quit
    pub should_quit: bool,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input text
    pub input: String,
    /// Cursor position in input
    pub cursor_position: usize,
    /// Current screen size
    pub size: Rect,
    /// Navigation history
    pub navigation_history: Vec<AppState>,
    /// Current tab index for screens with tabs
    pub current_tab: usize,
    /// Current list index for scrollable lists
    pub current_list_index: usize,
    /// Scroll offset for long content
    pub scroll_offset: usize,
    /// Search results or data cache
    pub data_cache: HashMap<String, serde_json::Value>,
    /// Loading states for different operations
    pub loading_states: HashMap<String, bool>,
    /// Error messages
    pub error_message: Option<String>,
    /// Success messages
    pub success_message: Option<String>,
    /// Configuration
    pub config: Config,
    /// Blockchain client
    pub blockchain_client: BlockchainService,
    /// Cache manager
    pub cache_manager: CacheManager,
    /// Wallet manager
    pub wallet_manager: WalletManager,
    /// Event sender for async operations
    pub event_sender: Option<mpsc::UnboundedSender<crate::ui::events::Event>>,
    /// Dashboard data for the main screen
    pub dashboard_data: DashboardData,
    /// Address data for address lookup screen
    pub address_data: Option<CompleteAddressData>,
}

impl App {
    /// Create a new application instance
    pub fn new(
        config: Config,
        blockchain_client: BlockchainService,
        cache_manager: CacheManager,
    ) -> Self {
        Self {
            state: AppState::Home,
            previous_state: None,
            should_quit: false,
            input_mode: InputMode::Normal,
            input: String::new(),
            cursor_position: 0,
            size: Rect::default(),
            navigation_history: Vec::new(),
            current_tab: 0,
            current_list_index: 0,
            scroll_offset: 0,
            data_cache: HashMap::new(),
            loading_states: HashMap::new(),
            error_message: None,
            success_message: None,
            config,
            blockchain_client,
            cache_manager,
            wallet_manager: WalletManager::new(),
            event_sender: None,
            dashboard_data: DashboardData::mock(),
            address_data: None,
        }
    }

    /// Set the event sender for async operations
    pub fn set_event_sender(&mut self, sender: mpsc::UnboundedSender<crate::ui::events::Event>) {
        self.event_sender = Some(sender);
    }

    /// Navigate to a new state
    pub fn navigate_to(&mut self, new_state: AppState) {
        if self.state != new_state {
            self.navigation_history.push(self.state.clone());
            self.previous_state = Some(self.state.clone());
            self.state = new_state;
            self.reset_navigation_state();
        }
    }

    /// Go back to the previous state
    pub fn go_back(&mut self) {
        if let Some(previous) = self.navigation_history.pop() {
            self.previous_state = Some(self.state.clone());
            self.state = previous;
            self.reset_navigation_state();
        }
    }

    /// Reset navigation-related state
    fn reset_navigation_state(&mut self) {
        self.current_tab = 0;
        self.current_list_index = 0;
        self.scroll_offset = 0;
        self.input.clear();
        self.cursor_position = 0;
        self.input_mode = InputMode::Normal;
        self.clear_messages();
    }

    /// Set the terminal size
    pub fn set_size(&mut self, size: Rect) {
        self.size = size;
    }

    /// Enter input mode
    pub fn enter_input_mode(&mut self) {
        self.input_mode = InputMode::Editing;
    }

    /// Exit input mode
    pub fn exit_input_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    /// Add character to input
    pub fn add_char(&mut self, c: char) {
        if self.input_mode == InputMode::Editing {
            self.input.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    /// Remove character from input
    pub fn remove_char(&mut self) {
        if self.input_mode == InputMode::Editing && self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    /// Move cursor left
    pub fn move_cursor_left(&mut self) {
        if self.input_mode == InputMode::Editing && self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// Move cursor right
    pub fn move_cursor_right(&mut self) {
        if self.input_mode == InputMode::Editing && self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    /// Clear input
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    /// Get current input
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Process current input and clear it
    pub fn process_input(&mut self) -> String {
        let input = self.input.clone();
        self.clear_input();
        input
    }

    /// Set input text
    pub fn set_input(&mut self, text: String) {
        self.input = text;
        self.cursor_position = self.input.len();
    }

    /// Move to next tab
    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.saturating_add(1);
    }

    /// Move to previous tab
    pub fn previous_tab(&mut self) {
        self.current_tab = self.current_tab.saturating_sub(1);
    }

    /// Move to next item in list
    pub fn next_item(&mut self) {
        self.current_list_index = self.current_list_index.saturating_add(1);
    }

    /// Move to previous item in list
    pub fn previous_item(&mut self) {
        self.current_list_index = self.current_list_index.saturating_sub(1);
    }

    /// Scroll down
    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    /// Scroll up
    pub fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

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

    /// Clear all messages
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
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

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get the current state as a string for display
    pub fn state_title(&self) -> &'static str {
        match self.state {
            AppState::Home => "Home",
            AppState::BlockExplorer => "Block Explorer",
            AppState::TransactionViewer => "Transaction Viewer",
            AppState::AddressLookup => "Address Lookup",
            AppState::ContractSearch => "Contract Search",
            AppState::TokenInfo => "Token Information",
            AppState::GasTracker => "Gas Tracker",
            AppState::ContractInteraction => "Contract Interaction",
            AppState::ContractVerification => "Contract Verification",
            AppState::WalletManager => "Wallet Manager",
            AppState::MultisigWallet => "Multi-Signature Wallet",
            AppState::EventMonitor => "Event Monitor",
            AppState::Help => "Help",
            AppState::Quit => "Quit",
        }
    }

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
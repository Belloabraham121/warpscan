use super::super::models::{CompleteAddressData, DashboardData};
use super::state::{AppState, DataMode, InputMode, ModeSelectionState};
use crate::{
    blockchain::BlockchainService, cache::CacheManager, config::Config, wallet::WalletManager,
};
use ratatui::layout::Rect;
use std::collections::HashMap;
use tokio::sync::mpsc;

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
    /// Transaction details for transaction viewer screen
    pub transaction_data: Option<super::super::models::TransactionDetails>,
    /// Whether input data section is expanded in transaction viewer
    pub input_data_expanded: bool,
    /// Data source mode (Local Node or Etherscan)
    pub data_mode: Option<DataMode>,
    /// Mode selection state
    pub mode_selection_state: ModeSelectionState,
    /// Pending address lookup (address being looked up in background)
    pub pending_address_lookup: Option<String>,
    /// Flag to trigger dashboard refresh in background
    pub pending_dashboard_refresh: bool,
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
            input: String::new(), // String::new() is already optimized
            cursor_position: 0,
            size: Rect::default(),
            navigation_history: Vec::new(), // Empty Vec is already optimized
            current_tab: 0,
            current_list_index: 0,
            scroll_offset: 0,
            data_cache: HashMap::new(), // Empty HashMap is already optimized
            loading_states: HashMap::new(), // Empty HashMap is already optimized
            error_message: None,
            success_message: None,
            config,
            blockchain_client,
            cache_manager,
            wallet_manager: WalletManager::new(),
            event_sender: None,
            dashboard_data: DashboardData::mock(), // This is the main cost, but needed for UI
            address_data: None,
            transaction_data: None,
            input_data_expanded: false,
            data_mode: None,
            mode_selection_state: ModeSelectionState::Selecting,
            pending_address_lookup: None,
            pending_dashboard_refresh: false,
        }
    }

    /// Set the event sender for async operations
    pub fn set_event_sender(&mut self, sender: mpsc::UnboundedSender<crate::ui::events::Event>) {
        self.event_sender = Some(sender);
    }

    /// Set the terminal size
    pub fn set_size(&mut self, size: Rect) {
        self.size = size;
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
        self.state.title()
    }
}

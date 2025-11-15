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
    Settings,
    Help,
    Quit,
}

/// Input mode for text fields
#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

impl AppState {
    /// Get the current state as a string for display
    pub fn title(&self) -> &'static str {
        match self {
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
            AppState::Settings => "Settings",
            AppState::Help => "Help",
            AppState::Quit => "Quit",
        }
    }
}

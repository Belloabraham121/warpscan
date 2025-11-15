// Screen modules
pub mod home;
pub mod block_explorer;
pub mod transaction_viewer;
pub mod address_lookup;
pub mod gas_tracker;
pub mod wallet_manager;
pub mod settings;

// Re-export render functions for convenience
pub use home::render_home;
pub use block_explorer::render_block_explorer;
pub use transaction_viewer::render_transaction_viewer;
pub use address_lookup::render_address_lookup;
pub use gas_tracker::render_gas_tracker;
pub use wallet_manager::render_wallet_manager;
pub use settings::render_settings;

// Screen enum definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Home,
    BlockExplorer,
    TransactionViewer,
    AddressLookup,
    ContractSearch,
    TokenInformation,
    GasTracker,
    ContractInteraction,
    ContractVerification,
    WalletManager,
    MultiSigWallet,
    EventMonitor,
    Help,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}
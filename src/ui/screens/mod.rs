// Screen modules
pub mod address_lookup;
pub mod block_explorer;
pub mod contract_interaction;
pub mod contract_search;
pub mod contract_verification;
pub mod event_monitor;
pub mod gas_tracker;
pub mod help;
pub mod home;
pub mod multisig_wallet;
pub mod settings;
pub mod token_info;
pub mod transaction_viewer;
pub mod wallet_manager;

// Re-export render functions for convenience
pub use address_lookup::render_address_lookup;
pub use block_explorer::render_block_explorer;
pub use contract_interaction::render_contract_interaction;
pub use contract_search::render_contract_search;
pub use contract_verification::render_contract_verification;
pub use event_monitor::render_event_monitor;
pub use gas_tracker::render_gas_tracker;
pub use help::render_help;
pub use home::render_home;
pub use multisig_wallet::render_multisig_wallet;
pub use settings::render_settings;
pub use token_info::render_token_info;
pub use transaction_viewer::render_transaction_viewer;
pub use wallet_manager::render_wallet_manager;

// Screen enum definition
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
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

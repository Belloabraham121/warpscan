// Network and blockchain data models
pub mod network_stats;
pub mod block_info;
pub mod transaction;
pub mod daily_transaction_data;
pub mod dashboard_data;
pub mod search_result;
pub mod address;
pub mod token;
pub mod internal_transaction;

// Re-export all public types for convenience
pub use network_stats::NetworkStats;
pub use block_info::BlockInfo;
pub use transaction::{TransactionInfo, TransactionStatus};
pub use daily_transaction_data::DailyTransactionData;
pub use dashboard_data::DashboardData;
pub use search_result::SearchResult;
pub use address::{
    AddressInfo, AddressDetails, AddressType, AddressTransaction, 
    AccountHistoryEntry, CompleteAddressData, AddressTab
};
pub use token::{TokenTransfer, TokenInfo, TokenType};
pub use internal_transaction::InternalTransaction;
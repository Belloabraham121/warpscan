use super::{AddressInfo, BlockInfo, TransactionInfo};
use serde::{Deserialize, Serialize};

/// Search result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResult {
    Block(BlockInfo),
    Transaction(TransactionInfo),
    Address(AddressInfo),
    NotFound,
}

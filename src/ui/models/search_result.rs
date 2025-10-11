use serde::{Deserialize, Serialize};
use super::{BlockInfo, TransactionInfo, AddressInfo};

/// Search result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResult {
    Block(BlockInfo),
    Transaction(TransactionInfo),
    Address(AddressInfo),
    NotFound,
}
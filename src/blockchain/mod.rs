//! Blockchain service layer for WarpScan
//!
//! This module provides the interface for interacting with Ethereum blockchain
//! using ethers.rs library.

pub mod etherscan;
pub mod service;
pub mod types;

// Re-export commonly used types and structs
pub use etherscan::{EtherscanChain, EtherscanClient};
pub use service::BlockchainService;
pub use types::{GasPrices, TransactionStatus};

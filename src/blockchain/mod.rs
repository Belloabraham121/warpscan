//! Blockchain service layer for WarpScan
//!
//! This module provides the interface for interacting with Ethereum blockchain
//! using ethers.rs library.

pub mod types;
pub mod service;

// Re-export commonly used types and structs
pub use types::{GasPrices, TransactionStatus};
pub use service::BlockchainService;
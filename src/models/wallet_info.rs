//! Wallet information models
//!
//! This module contains data structures for wallet information and balances.

use ethers::types::U256;
use serde::{Deserialize, Serialize};

/// Wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub address: String,
    pub mnemonic: Option<String>,
    pub created_at: u64,
    pub name: Option<String>,
}

/// Wallet balance information
#[derive(Debug, Clone)]
pub struct WalletBalance {
    pub eth_balance: U256,
    pub token_balances: Vec<TokenBalance>,
}

/// Token balance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenBalance {
    pub contract_address: String,
    pub symbol: String,
    pub name: String,
    pub balance: U256,
    pub decimals: u8,
}
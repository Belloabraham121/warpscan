//! Multi-signature wallet models
//!
//! This module contains data structures for multi-signature wallets and transaction proposals.

use ethers::types::U256;
use serde::{Deserialize, Serialize};

/// Multi-signature wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultisigWallet {
    pub address: String,
    pub owners: Vec<String>,
    pub threshold: u32,
    pub created_at: u64,
    pub name: Option<String>,
}

/// Transaction proposal for multi-signature wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionProposal {
    pub id: String,
    pub to: String,
    pub value: U256,
    pub data: Vec<u8>,
    pub signatures: Vec<String>,
    pub executed: bool,
    pub created_at: u64,
}

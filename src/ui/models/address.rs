use super::TransactionStatus;
use serde::{Deserialize, Serialize};

/// Address information for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressInfo {
    pub address: String,
    pub balance: f64,
    pub transaction_count: u64,
    pub is_contract: bool,
    pub contract_name: Option<String>,
}

impl Default for AddressInfo {
    fn default() -> Self {
        Self {
            address: "0x1234567890abcdef...".to_string(),
            balance: 10.5,
            transaction_count: 1_234,
            is_contract: false,
            contract_name: None,
        }
    }
}

/// Comprehensive address details for the address lookup screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDetails {
    pub address: String,
    pub address_type: AddressType,
    pub balance: f64,
    pub token_count: u32,
    pub estimated_net_worth: f64,
    pub total_transactions: u64,
    pub outgoing_transfers: u64,
    pub total_gas_used: u64,
    pub contract_name: Option<String>,
    pub contract_creator: Option<String>,
    pub creation_tx_hash: Option<String>,
    pub last_activity: u64,
}

/// Address type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AddressType {
    EOA,      // Externally Owned Account (Wallet)
    Contract, // Smart Contract
    Token,    // Token Contract
    MultiSig, // Multi-signature Wallet
    Exchange, // Exchange Address
    Unknown,  // Unknown type
}

impl Default for AddressDetails {
    fn default() -> Self {
        Self {
            address: "0x1234567890abcdef1234567890abcdef12345678".to_string(),
            address_type: AddressType::EOA,
            balance: 15.75,
            token_count: 25,
            estimated_net_worth: 18_450.50,
            total_transactions: 1_234,
            outgoing_transfers: 567,
            total_gas_used: 2_500_000,
            contract_name: None,
            contract_creator: None,
            creation_tx_hash: None,
            last_activity: 1640995200,
        }
    }
}

/// Transaction details for address transaction history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressTransaction {
    pub tx_hash: String,
    pub tx_type: String,
    pub method: String,
    pub block: u64,
    pub from: String,
    pub to: String,
    pub value: f64,
    pub fee: f64,
    pub timestamp: u64,
    pub status: TransactionStatus,
}

impl Default for AddressTransaction {
    fn default() -> Self {
        Self {
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
                .to_string(),
            tx_type: "Transfer".to_string(),
            method: "transfer".to_string(),
            block: 21_234_567,
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x2222222222222222222222222222222222222222".to_string(),
            value: 1.5,
            fee: 0.002,
            timestamp: 1640995200,
            status: TransactionStatus::Success,
        }
    }
}

/// Account history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountHistoryEntry {
    pub age: String,
    pub action: String,
    pub from: String,
    pub to: String,
    pub timestamp: u64,
    pub tx_hash: String,
}

impl Default for AccountHistoryEntry {
    fn default() -> Self {
        Self {
            age: "2 days ago".to_string(),
            action: "Received".to_string(),
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x2222222222222222222222222222222222222222".to_string(),
            timestamp: 1640995200,
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab"
                .to_string(),
        }
    }
}

/// Complete address data containing all tabs information
#[derive(Debug, Clone, Default)]
pub struct CompleteAddressData {
    pub details: AddressDetails,
    pub transactions: Vec<AddressTransaction>,
    pub account_history: Vec<AccountHistoryEntry>,
    pub token_transfers: Vec<super::TokenTransfer>,
    pub tokens: Vec<super::TokenInfo>,
    pub internal_transactions: Vec<super::InternalTransaction>,
    pub current_tab: AddressTab,
    pub selected_transaction_index: usize,
    pub selected_history_index: usize,
    pub selected_token_transfer_index: usize,
    pub selected_token_index: usize,
    pub selected_internal_txn_index: usize,
}

/// Address detail tabs
#[derive(Debug, Clone, PartialEq)]
pub enum AddressTab {
    Details,
    Transactions,
    AccountHistory,
    TokenTransfers,
    Tokens,
    InternalTxns,
}

impl Default for AddressTab {
    fn default() -> Self {
        AddressTab::Details
    }
}

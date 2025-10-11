//! Data models for UI components
//!
//! This module contains data structures used by the UI to display blockchain information.

use serde::{Deserialize, Serialize};


/// Network statistics displayed on the dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub ethereum_price: f64,
    pub market_cap: f64,
    pub latest_block: u64,
    pub transactions_count: u64,
    pub gas_price: u64,
    pub network_utilization: f64,
}

impl Default for NetworkStats {
    fn default() -> Self {
        Self {
            ethereum_price: 2048.75,
            market_cap: 246_000_000_000.0,
            latest_block: 21_234_567,
            transactions_count: 1_234_567_890,
            gas_price: 25,
            network_utilization: 0.75,
        }
    }
}

/// Block information for the latest blocks section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockInfo {
    pub number: u64,
    pub hash: String,
    pub timestamp: u64,
    pub miner: String,
    pub transaction_count: u32,
    pub gas_used: u64,
    pub gas_limit: u64,
    pub size: u64,
    pub reward: f64,
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            number: 21_234_567,
            hash: "0x1234567890abcdef...".to_string(),
            timestamp: 1640995200, // Example timestamp
            miner: "0xabcdef1234567890...".to_string(),
            transaction_count: 150,
            gas_used: 15_000_000,
            gas_limit: 30_000_000,
            size: 50_000,
            reward: 2.5,
        }
    }
}

/// Transaction information for the latest transactions section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: f64,
    pub gas_price: u64,
    pub gas_used: u64,
    pub status: TransactionStatus,
    pub timestamp: u64,
    pub block_number: u64,
    pub transaction_fee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
}

impl Default for TransactionInfo {
    fn default() -> Self {
        Self {
            hash: "0xabcdef1234567890...".to_string(),
            from: "0x1234567890abcdef...".to_string(),
            to: "0xfedcba0987654321...".to_string(),
            value: 1.5,
            gas_price: 25,
            gas_used: 21_000,
            status: TransactionStatus::Success,
            timestamp: 1640995200,
            block_number: 21_234_567,
            transaction_fee: 0.000525,
        }
    }
}

/// Daily transaction data point for the graph
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyTransactionData {
    pub date: String,
    pub transaction_count: u64,
    pub timestamp: u64,
}

impl Default for DailyTransactionData {
    fn default() -> Self {
        Self {
            date: "2024-01-01".to_string(),
            transaction_count: 1_500_000,
            timestamp: 1704067200, // 2024-01-01
        }
    }
}

/// Dashboard data containing all information for the main screen
#[derive(Debug, Clone, Default)]
pub struct DashboardData {
    pub network_stats: NetworkStats,
    pub latest_blocks: Vec<BlockInfo>,
    pub latest_transactions: Vec<TransactionInfo>,
    pub daily_transactions: Vec<DailyTransactionData>,
    pub search_results: Option<SearchResult>,
}

/// Search result types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchResult {
    Block(BlockInfo),
    Transaction(TransactionInfo),
    Address(AddressInfo),
    NotFound,
}

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
    EOA,           // Externally Owned Account (Wallet)
    Contract,      // Smart Contract
    Token,         // Token Contract
    MultiSig,      // Multi-signature Wallet
    Exchange,      // Exchange Address
    Unknown,       // Unknown type
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
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab".to_string(),
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
            tx_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab".to_string(),
        }
    }
}

/// Token transfer entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenTransfer {
    pub token_id: Option<String>,
    pub txn_hash: String,
    pub from: String,
    pub to: String,
    pub token_name: String,
    pub token_symbol: String,
    pub amount: f64,
    pub timestamp: u64,
}

impl Default for TokenTransfer {
    fn default() -> Self {
        Self {
            token_id: None,
            txn_hash: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab".to_string(),
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x2222222222222222222222222222222222222222".to_string(),
            token_name: "Tether USD".to_string(),
            token_symbol: "USDT".to_string(),
            amount: 100.0,
            timestamp: 1640995200,
        }
    }
}

/// Token information for tokens tab
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub token_type: TokenType,
    pub balance: f64,
    pub value_usd: f64,
    pub decimals: u8,
}

/// Token type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenType {
    ERC20,
    ERC721,  // NFT
    ERC1155, // Multi-token
    Other(String),
}

impl Default for TokenInfo {
    fn default() -> Self {
        Self {
            contract_address: "0xdac17f958d2ee523a2206206994597c13d831ec7".to_string(),
            name: "Tether USD".to_string(),
            symbol: "USDT".to_string(),
            token_type: TokenType::ERC20,
            balance: 1000.0,
            value_usd: 1000.0,
            decimals: 6,
        }
    }
}

/// Internal transaction entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternalTransaction {
    pub parent_tx: String,
    pub hash: String,
    pub tx_type: String,
    pub block: u64,
    pub from: String,
    pub to: String,
    pub value_in: f64,
    pub value_out: f64,
    pub timestamp: u64,
}

impl Default for InternalTransaction {
    fn default() -> Self {
        Self {
            parent_tx: "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890ab".to_string(),
            hash: "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12".to_string(),
            tx_type: "call".to_string(),
            block: 21_234_567,
            from: "0x1111111111111111111111111111111111111111".to_string(),
            to: "0x2222222222222222222222222222222222222222".to_string(),
            value_in: 0.0,
            value_out: 1.5,
            timestamp: 1640995200,
        }
    }
}

/// Complete address data containing all tabs information
#[derive(Debug, Clone, Default)]
pub struct CompleteAddressData {
    pub details: AddressDetails,
    pub transactions: Vec<AddressTransaction>,
    pub account_history: Vec<AccountHistoryEntry>,
    pub token_transfers: Vec<TokenTransfer>,
    pub tokens: Vec<TokenInfo>,
    pub internal_transactions: Vec<InternalTransaction>,
    pub current_tab: AddressTab,
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

/// Generate mock data for development
impl DashboardData {
    pub fn mock() -> Self {
        let mut latest_blocks = Vec::new();
        for i in 0..5 {
            let mut block = BlockInfo::default();
            block.number = 21_234_567 - i;
            block.hash = format!("0x{:016x}...", 0x1234567890abcdef - i * 0x1111);
            block.transaction_count = 150 - (i as u32 * 10);
            latest_blocks.push(block);
        }

        let mut latest_transactions = Vec::new();
        for i in 0..5 {
            let mut tx = TransactionInfo::default();
            tx.hash = format!("0x{:016x}...", 0xabcdef1234567890u64 - i as u64 * 0x2222);
            tx.value = 1.5 - (i as f64 * 0.1);
            tx.block_number = 19000000 + i as u64;
            tx.from = format!("0x{:016x}...", 0x1111111111111111u64 + i as u64 * 0x1000);
            tx.to = format!("0x{:016x}...", 0x2222222222222222u64 + i as u64 * 0x1000);
            latest_transactions.push(tx);
        }

        // Real daily transaction data
        let daily_transactions = vec![
            DailyTransactionData { date: "2025-09-07".to_string(), transaction_count: 136896, timestamp: 1725667200 },
            DailyTransactionData { date: "2025-09-08".to_string(), transaction_count: 179543, timestamp: 1725753600 },
            DailyTransactionData { date: "2025-09-09".to_string(), transaction_count: 150851, timestamp: 1725840000 },
            DailyTransactionData { date: "2025-09-10".to_string(), transaction_count: 132986, timestamp: 1725926400 },
            DailyTransactionData { date: "2025-09-11".to_string(), transaction_count: 141964, timestamp: 1726012800 },
            DailyTransactionData { date: "2025-09-12".to_string(), transaction_count: 131455, timestamp: 1726099200 },
            DailyTransactionData { date: "2025-09-13".to_string(), transaction_count: 116979, timestamp: 1726185600 },
            DailyTransactionData { date: "2025-09-14".to_string(), transaction_count: 115221, timestamp: 1726272000 },
            DailyTransactionData { date: "2025-09-15".to_string(), transaction_count: 141927, timestamp: 1726358400 },
            DailyTransactionData { date: "2025-09-16".to_string(), transaction_count: 139257, timestamp: 1726444800 },
            DailyTransactionData { date: "2025-09-17".to_string(), transaction_count: 141411, timestamp: 1726531200 },
            DailyTransactionData { date: "2025-09-18".to_string(), transaction_count: 150013, timestamp: 1726617600 },
            DailyTransactionData { date: "2025-09-19".to_string(), transaction_count: 150720, timestamp: 1726704000 },
            DailyTransactionData { date: "2025-09-20".to_string(), transaction_count: 146662, timestamp: 1726790400 },
            DailyTransactionData { date: "2025-09-21".to_string(), transaction_count: 132367, timestamp: 1726876800 },
            DailyTransactionData { date: "2025-09-22".to_string(), transaction_count: 148264, timestamp: 1726963200 },
            DailyTransactionData { date: "2025-09-23".to_string(), transaction_count: 147550, timestamp: 1727049600 },
            DailyTransactionData { date: "2025-09-24".to_string(), transaction_count: 145978, timestamp: 1727136000 },
            DailyTransactionData { date: "2025-09-25".to_string(), transaction_count: 151187, timestamp: 1727222400 },
            DailyTransactionData { date: "2025-09-26".to_string(), transaction_count: 149293, timestamp: 1727308800 },
            DailyTransactionData { date: "2025-09-27".to_string(), transaction_count: 136378, timestamp: 1727395200 },
            DailyTransactionData { date: "2025-09-28".to_string(), transaction_count: 126226, timestamp: 1727481600 },
            DailyTransactionData { date: "2025-09-29".to_string(), transaction_count: 121159, timestamp: 1727568000 },
            DailyTransactionData { date: "2025-09-30".to_string(), transaction_count: 129088, timestamp: 1727654400 },
            DailyTransactionData { date: "2025-10-01".to_string(), transaction_count: 120763, timestamp: 1727740800 },
            DailyTransactionData { date: "2025-10-02".to_string(), transaction_count: 121521, timestamp: 1727827200 },
            DailyTransactionData { date: "2025-10-03".to_string(), transaction_count: 116588, timestamp: 1727913600 },
            DailyTransactionData { date: "2025-10-04".to_string(), transaction_count: 104221, timestamp: 1728000000 },
            DailyTransactionData { date: "2025-10-05".to_string(), transaction_count: 142514, timestamp: 1728086400 },
            DailyTransactionData { date: "2025-10-06".to_string(), transaction_count: 114528, timestamp: 1728172800 },
            DailyTransactionData { date: "2025-10-07".to_string(), transaction_count: 113100, timestamp: 1728259200 },
        ];

        Self {
            network_stats: NetworkStats::default(),
            latest_blocks,
            latest_transactions,
            daily_transactions,
            search_results: None,
        }
    }
}
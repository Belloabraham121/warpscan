use super::{NetworkStats, BlockInfo, TransactionInfo, DailyTransactionData, SearchResult};

/// Dashboard data containing all information for the main screen
#[derive(Debug, Clone, Default)]
pub struct DashboardData {
    pub network_stats: NetworkStats,
    pub latest_blocks: Vec<BlockInfo>,
    pub latest_transactions: Vec<TransactionInfo>,
    pub daily_transactions: Vec<DailyTransactionData>,
    pub search_results: Option<SearchResult>,
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
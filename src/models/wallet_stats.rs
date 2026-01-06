//! Wallet statistics models
//!
//! This module contains data structures for wallet statistics and metrics.

/// Wallet statistics
#[derive(Debug, Clone)]
pub struct WalletStats {
    pub total_wallets: usize,
    pub total_multisig_wallets: usize,
    pub wallets_with_mnemonic: usize,
}

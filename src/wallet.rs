//! Wallet management for WarpScan
//!
//! This module provides functionality for generating and managing test wallets
//! for contract interaction and testing purposes.

use bip39::{Mnemonic, Language};
use ethers::{
    signers::{LocalWallet, Signer},
    types::{Address, Signature},
};
use rand::Rng;
use std::str::FromStr;
use crate::error::{Error, Result};
use crate::models::{WalletInfo, MultisigWallet, WalletStats};

/// Wallet manager for handling wallet operations
pub struct WalletManager {
    wallets: Vec<WalletInfo>,
    multisig_wallets: Vec<MultisigWallet>,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new() -> Self {
        Self {
            wallets: Vec::new(),
            multisig_wallets: Vec::new(),
        }
    }

    /// Generate a new random wallet
    pub fn generate_wallet(&mut self, name: Option<String>) -> Result<(WalletInfo, LocalWallet)> {
        // Generate random entropy
        let mut rng = rand::thread_rng();
        let entropy: [u8; 32] = rng.gen();
        
        // Create mnemonic from entropy
        let mnemonic = Mnemonic::from_entropy(&entropy)
            .map_err(|e| Error::wallet(format!("Failed to generate mnemonic: {}", e)))?;
        
        // Create wallet from mnemonic
        let wallet = mnemonic.to_string().parse::<LocalWallet>()
            .map_err(|e| Error::wallet(format!("Failed to create wallet from mnemonic: {}", e)))?;
        
        let wallet_info = WalletInfo {
            address: format!("{:?}", wallet.address()),
            mnemonic: Some(mnemonic.to_string()),
            created_at: chrono::Utc::now().timestamp() as u64,
            name,
        };
        
        self.wallets.push(wallet_info.clone());
        
        Ok((wallet_info, wallet))
    }

    /// Import wallet from private key
    pub fn import_wallet_from_private_key(
        &mut self,
        private_key: &str,
        name: Option<String>,
    ) -> Result<(WalletInfo, LocalWallet)> {
        let wallet = LocalWallet::from_str(private_key)
            .map_err(|e| Error::wallet(format!("Invalid private key: {}", e)))?;
        
        let wallet_info = WalletInfo {
            address: format!("{:?}", wallet.address()),
            mnemonic: None,
            created_at: chrono::Utc::now().timestamp() as u64,
            name,
        };
        
        self.wallets.push(wallet_info.clone());
        
        Ok((wallet_info, wallet))
    }

    /// Import wallet from mnemonic
    pub fn import_wallet_from_mnemonic(
        &mut self,
        mnemonic: &str,
        name: Option<String>,
    ) -> Result<(WalletInfo, LocalWallet)> {
        // Validate mnemonic
        let _mnemonic_obj = Mnemonic::parse_in_normalized(Language::English, mnemonic)
            .map_err(|e| Error::wallet(format!("Invalid mnemonic: {}", e)))?;
        
        let wallet = mnemonic.parse::<LocalWallet>()
            .map_err(|e| Error::wallet(format!("Failed to create wallet from mnemonic: {}", e)))?;
        
        let wallet_info = WalletInfo {
            address: format!("{:?}", wallet.address()),
            mnemonic: Some(mnemonic.to_string()),
            created_at: chrono::Utc::now().timestamp() as u64,
            name,
        };
        
        self.wallets.push(wallet_info.clone());
        
        Ok((wallet_info, wallet))
    }

    /// Get all wallets
    pub fn get_wallets(&self) -> &[WalletInfo] {
        &self.wallets
    }

    /// Remove wallet by address
    pub fn remove_wallet(&mut self, address: &str) -> Result<()> {
        let index = self
            .wallets
            .iter()
            .position(|w| w.address == address)
            .ok_or_else(|| Error::wallet("Wallet not found"))?;
        
        self.wallets.remove(index);
        Ok(())
    }

    /// Create a multi-signature wallet
    pub fn create_multisig_wallet(
        &mut self,
        owners: Vec<String>,
        threshold: u32,
        name: Option<String>,
    ) -> Result<MultisigWallet> {
        // Validate inputs
        if owners.is_empty() {
            return Err(Error::validation("At least one owner is required"));
        }
        
        if threshold == 0 || threshold > owners.len() as u32 {
            return Err(Error::validation("Invalid threshold value"));
        }
        
        // Validate owner addresses
        for owner in &owners {
            Address::from_str(owner)
                .map_err(|e| Error::validation(format!("Invalid owner address {}: {}", owner, e)))?;
        }
        
        // Generate a deterministic address based on owners and threshold
        // In a real implementation, this would be the actual deployed contract address
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        owners.hash(&mut hasher);
        threshold.hash(&mut hasher);
        let hash = hasher.finish();
        
        let multisig_wallet = MultisigWallet {
            address: format!("0x{:040x}", hash), // Placeholder address
            owners,
            threshold,
            created_at: chrono::Utc::now().timestamp() as u64,
            name,
        };
        
        self.multisig_wallets.push(multisig_wallet.clone());
        
        Ok(multisig_wallet)
    }

    /// Get all multi-signature wallets
    pub fn get_multisig_wallets(&self) -> &[MultisigWallet] {
        &self.multisig_wallets
    }

    /// Sign a transaction with a wallet
    pub async fn sign_transaction(
        wallet: &LocalWallet,
        transaction: &ethers::types::transaction::eip2718::TypedTransaction,
    ) -> Result<Signature> {
        let signature = wallet
            .sign_transaction(transaction)
            .await
            .map_err(|e| Error::wallet(format!("Failed to sign transaction: {}", e)))?;
        
        Ok(signature)
    }

    /// Validate an Ethereum address
    pub fn validate_address(address: &str) -> Result<Address> {
        Address::from_str(address)
            .map_err(|e| Error::validation(format!("Invalid address: {}", e)))
    }

    /// Generate a random mnemonic phrase
    pub fn generate_mnemonic() -> Result<String> {
        let mut rng = rand::thread_rng();
        let entropy: [u8; 32] = rng.gen();
        
        let mnemonic = Mnemonic::from_entropy(&entropy)
            .map_err(|e| Error::wallet(format!("Failed to generate mnemonic: {}", e)))?;
        
        Ok(mnemonic.to_string())
    }

    /// Validate a mnemonic phrase
    pub fn validate_mnemonic(mnemonic: &str) -> Result<()> {
        Mnemonic::parse_in_normalized(Language::English, mnemonic)
            .map_err(|e| Error::validation(format!("Invalid mnemonic: {}", e)))?;
        
        Ok(())
    }

    /// Get wallet statistics
    pub fn get_stats(&self) -> WalletStats {
        WalletStats {
            total_wallets: self.wallets.len(),
            total_multisig_wallets: self.multisig_wallets.len(),
            wallets_with_mnemonic: self.wallets.iter().filter(|w| w.mnemonic.is_some()).count(),
        }
    }
}



impl Default for WalletManager {
    fn default() -> Self {
        Self::new()
    }
}
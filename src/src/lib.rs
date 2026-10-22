pub mod fees;

use crate::fees::{calculate_and_split_fee, FeeConfig, FeeSplit};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum YodError {
    #[error("Wallet not found: {0}")]
    WalletNotFound(String),
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Offline limit exceeded")]
    OfflineLimitExceeded,
    #[error("Nonce already used – possible double-spend")]
    NonceAlreadyUsed,
    #[error("Invalid state transition")]
    InvalidState,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TxState {
    PendingOffline,
    ConfirmedLocal,
    Settled,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NetworkMode {
    Normal,      // 3f+1
    Partitioned, // 2f+1
    Extreme,     // 1f+1
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WalletTier {
    Standard,
    Business,
    Enterprise,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub balance: u64,
    pub offline_spent: u64,
    pub offline_limit: u64,
    pub nonce: u64,
    pub tier: WalletTier,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee_split: FeeSplit,
    pub nonce: u64,
    pub state: TxState,
    pub confirmations: HashSet<String>,
    pub created_at: DateTime<Utc>,
}

pub struct Yod4CoinsNode {
    pub node_id: String,
    pub mode: NetworkMode,
    pub wallets: HashMap<String, Wallet>,
    pub txs: HashMap<String, Transaction>,
    pub spent_nonces: HashSet<(String, u64)>,
    pub fee_config: FeeConfig,
}

impl Yod4CoinsNode {
    pub fn new(node_id: &str) -> Self {
        Self {
            node_id: node_id.to_string(),
            mode: NetworkMode::Normal,
            wallets: HashMap::new(),
            txs: HashMap::new(),
            spent_nonces: HashSet::new(),
            fee_config: FeeConfig::default(),
        }
    }

    pub fn required_confirmations(&self) -> usize {
        match self.mode {
            NetworkMode::Normal => 4,
            NetworkMode::Partitioned => 3,
            NetworkMode::Extreme => 1,
        }
    }

    pub fn create_wallet(&mut self, id: &str, balance: u64, tier: WalletTier) {
        let limit = match tier {
            WalletTier::Standard => 20_000,
            WalletTier::Business => 200_000,
            WalletTier::Enterprise => 1_000_000,
        };

        self.wallets.insert(
            id.to_string(),
            Wallet {
                id: id.to_string(),
                balance,
                offline_spent: 0,
                offline_limit: limit,
                nonce: 0,
                tier,
            },
        );
    }

    pub fn send_offline(
        &mut self,
        from: &str,
        to: &str,
        amount: u64,
    ) -> Result<String, YodError> {
        let wallet = self
            .wallets
            .get_mut(from)
            .ok_or(YodError::WalletNotFound(from.to_string()))?;

        if wallet.balance < amount {
            return Err(YodError::InsufficientBalance);
        }
        if wallet.offline_spent + amount > wallet.offline_limit {
            return Err(YodError::OfflineLimitExceeded);
        }
        if self.spent_nonces.contains(&(from.to_string(), wallet.nonce)) {
            return Err(YodError::NonceAlreadyUsed);
        }

        let fee_split = calculate_and_split_fee(amount, &self.fee_config);

        let tx_id = Uuid::new_v4().to_string();
        let tx = Transaction {
            id: tx_id.clone(),
            from: from.to_string(),
            to: to.to_string(),
            amount: fee_split.net_amount,
            fee_split,
            nonce: wallet.nonce,
            state: TxState::PendingOffline,
            confirmations: HashSet::new(),
            created_at: Utc::now(),
        };

        wallet.offline_spent += amount;
        wallet.nonce += 1;
        self.spent_nonces.insert((from.to_string(), tx.nonce));
        self.txs.insert(tx_id.clone(), tx);

        Ok(tx_id)
    }

    pub fn confirm_local(&mut self, tx_id: &str, confirmer: &str) -> Result<(), YodError> {
        let tx = self.txs.get_mut(tx_id).ok_or(YodError::InvalidState)?;

        if tx.state != TxState::PendingOffline && tx.state != TxState::ConfirmedLocal {
            return Err(YodError::InvalidState);
        }

        tx.confirmations.insert(confirmer.to_string());

        if tx.confirmations.len() >= self.required_confirmations() {
            tx.state = TxState::ConfirmedLocal;
        }

        Ok(())
    }

    pub fn settle(&mut self, tx_id: &str) -> Result<(), YodError> {
        let tx = self.txs.get_mut(tx_id).ok_or(YodError::InvalidState)?;

        if tx.state != TxState::ConfirmedLocal {
            return Err(YodError::InvalidState);
        }

        // In production: apply net_amount to recipient and
        // route founder_fee + host_fee to their wallets.
        tx.state = TxState::Settled;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: NetworkMode) {
        self.mode = mode;
    }
  }

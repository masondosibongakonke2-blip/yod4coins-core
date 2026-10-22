use serde::{Deserialize, Serialize};

/// Transparent protocol-level fee configuration with automatic split
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeConfig {
    /// Total utility fee in basis points (10 = 0.10%)
    pub utility_fee_bps: u16,
    /// Percentage of the fee that goes to the Founder (0–100)
    pub founder_share_percent: u8,
    /// Founder Treasury Wallet address
    pub founder_wallet: String,
    /// Sovereign Host Wallet address
    pub host_wallet: String,
}

impl Default for FeeConfig {
    fn default() -> Self {
        Self {
            utility_fee_bps: 10, // 0.10%
            founder_share_percent: 60,
            founder_wallet: "FOUNDER_TREASURY_WALLET_ADDRESS".to_string(),
            host_wallet: "SOVEREIGN_HOST_WALLET_ADDRESS".to_string(),
        }
    }
}

/// Result of fee calculation and split
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeeSplit {
    pub net_amount: u64,
    pub total_fee: u64,
    pub founder_fee: u64,
    pub host_fee: u64,
    pub founder_wallet: String,
    pub host_wallet: String,
}

/// Deterministic fee calculation + automatic split
pub fn calculate_and_split_fee(amount: u64, config: &FeeConfig) -> FeeSplit {
    let total_fee = (amount as u128 * config.utility_fee_bps as u128 / 10_000) as u64;
    let founder_fee = (total_fee as u128 * config.founder_share_percent as u128 / 100) as u64;
    let host_fee = total_fee.saturating_sub(founder_fee);
    let net_amount = amount.saturating_sub(total_fee);

    FeeSplit {
        net_amount,
        total_fee,
        founder_fee,
        host_fee,
        founder_wallet: config.founder_wallet.clone(),
        host_wallet: config.host_wallet.clone(),
    }
}

yod4coins.hpp
  #pragma once
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <cstdint>

enum class NetworkMode { Normal, Partitioned, Extreme };
enum class TxState { PendingOffline, ConfirmedLocal, Settled };
enum class WalletTier { Standard, Business, Enterprise };

struct FeeSplit {
    uint64_t net_amount;
    uint64_t total_fee;
    uint64_t founder_fee;
    uint64_t host_fee;
    std::string founder_wallet;
    std::string host_wallet;
};

struct Wallet {
    std::string id;
    uint64_t balance = 0;
    uint64_t offline_spent = 0;
    uint64_t offline_limit = 0;
    uint64_t nonce = 0;
    WalletTier tier = WalletTier::Standard;
};

struct Transaction {
    std::string id;
    std::string from;
    std::string to;
    uint64_t amount = 0;
    FeeSplit fee_split;
    uint64_t nonce = 0;
    TxState state = TxState::PendingOffline;
    std::unordered_set<std::string> confirmations;
};

class Yod4CoinsNode {
public:
    std::string node_id;
    NetworkMode mode = NetworkMode::Normal;
    std::unordered_map<std::string, Wallet> wallets;
    std::unordered_map<std::string, Transaction> txs;
    std::unordered_set<std::string> spent_nonces;

    // Fee configuration
    uint16_t utility_fee_bps = 10;          // 0.10%
    uint8_t  founder_share_percent = 60;
    std::string founder_wallet = "FOUNDER_TREASURY_WALLET_ADDRESS";
    std::string host_wallet    = "SOVEREIGN_HOST_WALLET_ADDRESS";

    explicit Yod4CoinsNode(const std::string& id) : node_id(id) {}

    size_t required_confirmations() const {
        switch (mode) {
            case NetworkMode::Normal:      return 4;
            case NetworkMode::Partitioned: return 3;
            case NetworkMode::Extreme:     return 1;
        }
        return 4;
    }

    FeeSplit calculate_and_split_fee(uint64_t amount) const {
        uint64_t total_fee   = (amount * utility_fee_bps) / 10000;
        uint64_t founder_fee = (total_fee * founder_share_percent) / 100;
        uint64_t host_fee    = total_fee - founder_fee;

        return {
            amount - total_fee,
            total_fee,
            founder_fee,
            host_fee,
            founder_wallet,
            host_wallet
        };
    }

    // Note: Full send_offline / confirm_local / settle methods
    // follow the same logic as the Rust implementation.
};

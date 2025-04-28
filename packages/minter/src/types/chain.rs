use candid::CandidType;
use crypto_bigint::U256;
use serde::{Deserialize, Serialize};
use super::block_hash::{BlockHash, BlockId};

pub const BLOCK_TIME: u64 = 60 * 15; // block time in secs (same as bitcoin)
pub const INIT_BLOCK_REWARD: u64 = 50_00000000; // initial reward (same as bitcoin)
pub const BLOCKS_PER_HALVING: u64 = (60 * 60 * 24 * 288) / BLOCK_TIME; // halving every 288 days (5x faster than bitcoin)

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Chain {
    pub height: u32,
    pub last_block_id: BlockId,
    pub accumulated_reward: u64,
    pub next_nft_block_log_id: u128,
}

impl Chain {
    pub fn new(

    ) -> Self {
        Self {
            height: 0,
            last_block_id: BlockHash::new(U256::ZERO),
            accumulated_reward: 0,
            next_nft_block_log_id: 0,
        }
    }
}
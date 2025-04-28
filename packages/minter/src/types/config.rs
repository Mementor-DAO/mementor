use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use super::{coin::Coin, nft_col::NftCollection};

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Config {
    pub administrator: Principal,
    pub team_fee: u64, // % in e8s
    pub treasury_fee: u64, // % in e8s
    pub block_reward_tiers: Vec<u64>, // %s in e8s
    pub meme_nft: NftCollection,
    pub meme_coin: Coin,
}
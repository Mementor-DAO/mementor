use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use super::{
    coin::{Coin, CoinConfig}, 
    nft_col::{NftCollection, NftCollectionConfig}
};

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct InitOrUpgradeArgs {
    pub oc_public_key: String,
    pub administrator: Principal,
    pub memes_json_bytes: Vec<u8>, 
    pub index_tar_bytes: Vec<u8>,
    pub meme_nft: NftCollection,
    pub meme_nft_config: NftCollectionConfig,
    pub meme_coin: Coin,
    pub meme_coin_config: CoinConfig,
}


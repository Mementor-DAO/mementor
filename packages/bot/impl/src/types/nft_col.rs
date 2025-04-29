use std::u32;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize, Default)]
pub enum NftCollectionType {
    #[default]
    Icrc7
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct NftCollection {
    pub ty: NftCollectionType,
    pub max_supply: u32,
    pub canister_id: Principal,
    pub url_template: String,
}

impl Default for NftCollection {
    fn default() -> Self {
        Self { 
            ty: Default::default(), 
            max_supply: u32::MAX,
            canister_id: Principal::anonymous(), 
            url_template: Default::default() 
        }
    }
}

#[derive(Clone, Serialize, Deserialize, CandidType, Default)]
pub struct NftCollectionConfig {
    pub min_num_reactions: u32,
    pub max_num_reactions: u32,
    pub min_minting_cost: u64,
    pub max_minting_cost: u64,
    pub min_chat_members: u32,
    pub min_user_creation_interval: u64,
    pub team_fee_p: u64,
}

use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub enum NftCollectionType {
    Icrc7
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct NftCollection {
    pub ty: NftCollectionType,
    pub canister_id: Principal,
}

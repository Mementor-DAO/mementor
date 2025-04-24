use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub enum CoinType {
    Icrc1
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Coin {
    pub ty: CoinType,
    pub canister_id: Principal,
}
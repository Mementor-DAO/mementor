use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub enum CoinType {
    Icrc1,
    IcpLedger,
}

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub struct Coin {
    pub ty: CoinType,
    pub canister_id: Principal,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct CoinConfig {
    pub minter_canister_id: Principal,
}

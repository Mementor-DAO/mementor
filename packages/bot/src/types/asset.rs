use candid::CandidType;
use serde::Deserialize;

pub type AssetPath = String;

#[derive(CandidType, Deserialize)]
pub struct Asset {
    pub mime_type: String,
    pub data: Vec<u8>,
}


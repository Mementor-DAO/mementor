use candid::CandidType;
use serde::{Deserialize, Serialize};
use super::config::Config;

#[derive(CandidType, Serialize, Deserialize)]
pub struct InitOrUpgradeArgs {
    pub config: Config,
}


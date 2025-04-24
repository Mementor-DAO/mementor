mod types;
mod memory;
mod state;
mod lifecycle;
mod services;
mod storage;
mod utils;
mod queries;

use std::collections::BTreeMap;
use icrc_ledger_types::icrc::generic_value::Value;
use getrandom::register_custom_getrandom;
use crate::types::init::InitOrUpgradeArgs;

ic_cdk::export_candid!();

fn custom_getrandom(
    _: &mut [u8]
) -> Result<(), getrandom::Error> {
    Err(getrandom::Error::UNSUPPORTED)
}

register_custom_getrandom!(custom_getrandom);
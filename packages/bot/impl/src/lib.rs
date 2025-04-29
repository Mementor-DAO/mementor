mod resources;
mod types;
mod utils;
mod services;
mod state;
mod memory;
mod router;
mod lifecycle;
mod http_request;
mod storage;
mod queries;
mod updates;

use std::collections::BTreeMap;
use ic_http_certification::{HttpRequest, HttpResponse};
use icrc_ledger_types::icrc::generic_value::Value;
use crate::types::init::InitOrUpgradeArgs;
use bot_api::
    insert_image::{ImageInsertRequest, ImageInsertResponse}
;

ic_cdk::export_candid!();
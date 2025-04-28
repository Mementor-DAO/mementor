use std::collections::BTreeMap;
use candid::Principal;
use icrc_ledger_types::{
    icrc::generic_value::Value, 
    icrc1::account::{Account, Subaccount}
};
use sha2::{Digest, Sha256};

use super::nat::nat_to_u128;

pub fn str_to_subaccount(
    text: &str
) -> Subaccount {
    const DOMAIN: &[u8] = b"str-id";
    const DOMAIN_LENGTH: [u8; 1] = [6];

    let mut hasher = Sha256::new();
    _ = hasher.update(&DOMAIN_LENGTH);
    _ = hasher.update(&DOMAIN);
    _ = hasher.update(&text.as_bytes());
    hasher.finalize().into()
}

pub fn get_tx_id(
    tx: &BTreeMap<String, Value>
) -> Option<u128> {
    if let Some(tid) = tx.get("tid") {
        if let Ok(tid) = tid.clone().as_nat() {
            Some(nat_to_u128(tid))
        }
        else {
            None
        }
    }
    else {
        None
    }
}

#[allow(unused)]
pub fn get_tx_ts(
    tx: &BTreeMap<String, Value>
) -> Option<u64> {
    if let Some(ts) = tx.get("ts") {
        if let Ok(ts) = ts.clone().as_nat() {
            Some(nat_to_u128(ts) as u64)
        }
        else {
            None
        }
    }
    else {
        None
    }
}

pub fn get_tx_meta(
    tx: &BTreeMap<String, Value>
) -> Option<BTreeMap<String, Value>> {
    if let Some(meta) = tx.get("meta") {
        if let Ok(meta) = meta.clone().as_map() {
            Some(meta)
        }
        else {
            None
        }
    }
    else {
        None
    }
}

pub fn get_meta_reactions(
    meta: &BTreeMap<String, Value>
) -> Option<u32> {
    if let Some(reactions) = meta.get("Reactions") {
        if let Ok(num) = reactions.clone().as_nat() {
            Some(nat_to_u128(num) as u32)
        }
        else {
            None
        }
    }
    else {
        None
    }
}

pub fn get_tx_to(
    tx: &BTreeMap<String, Value>
) -> Option<Account> {
    if let Some(to) = tx.get("to") {
        if let Ok(to) = to.clone().as_array() {
            if let Ok(owner) = to[0].clone().as_blob() {
                if let Ok(subaccount) = to[1].clone().as_blob() {
                    let owner = Principal::from_slice(owner.as_slice());
                    let subaccount = subaccount.first_chunk::<32>()
                        .unwrap().clone();
                    return Some(Account {
                        owner, 
                        subaccount: Some(subaccount)
                    });
                }
            }
        }
    }

    None
}
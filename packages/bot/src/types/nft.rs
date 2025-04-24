use std::{borrow::Cow, collections::BTreeMap};
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::icrc::generic_value::Value;
use serde::{Deserialize, Serialize};

use super::blob::BlobId;

pub type NftId = u128;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Nft {
    pub token_id: NftId,
    pub blob_id: BlobId,
    pub meta: Option<BTreeMap<String, Value>>,
    pub minted_at: u32, // in seconds
}

impl Nft {
    pub fn new(
        token_id: NftId,
        blob_id: BlobId,
        meta: Option<BTreeMap<String, Value>>,
        minted_at: u32
    ) -> Self {
        Self {
            token_id,
            blob_id,
            meta,
            minted_at,
        }
    }
}

impl Storable for Nft {
    fn to_bytes(
        &self
    ) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(
        bytes: std::borrow::Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

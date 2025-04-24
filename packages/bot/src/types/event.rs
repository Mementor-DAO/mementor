use std::borrow::Cow;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Clone, CandidType, Serialize, Deserialize)]
pub enum Event {
    NftMinted {
        token_id: u128,
        to: Account,
        timestamp: u32,
    },
    NftListed {
        token_id: u128,
        value: u64,
        timestamp: u32,
    },
    NftSold {
        token_id: u128,
        value: u64,
        timestamp: u32,
    },
}

impl Storable for Event {
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

    const BOUND: Bound = Bound::Bounded { 
        max_size: (size_of::<Event>() + size_of::<u32>() * 5) as _, 
        is_fixed_size: false
    };
}
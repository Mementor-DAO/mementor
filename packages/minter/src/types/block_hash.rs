use candid::{CandidType, Decode, Encode};
use crypto_bigint::U256;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};

pub type BlockId = BlockHash;

#[derive(Clone, Serialize, Deserialize, CandidType, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockHash {
    pub inner: [u8; 32]
}

impl BlockHash {
    pub fn new(
        hash: U256
    ) -> Self {
        Self{
            inner: hash.to_le_bytes()
        }
    }
}

impl Storable for BlockHash {
    fn to_bytes(
        &self
    ) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(
        bytes: std::borrow::Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded { 
        max_size: size_of::<BlockHash>() as u32 + size_of::<u32>() as u32 * 5, 
        is_fixed_size: false
    };
}
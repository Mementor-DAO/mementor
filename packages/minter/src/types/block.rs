use candid::{CandidType, Decode, Encode};
use crypto_bigint::U256;
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use crate::utils::hasher::DblHasher;
use super::{block_hash::BlockHash, transaction::TxId};

pub const BLOCK_V1_00: u32 = 1_00;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct BlockHeader {
    pub version: u32,
    pub prev: BlockHash,
    pub merkle_root: BlockHash,
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Block {
    pub height: u32,
    pub header: BlockHeader,
    pub txs: Vec<TxId>,
}

impl Storable for Block {
    fn from_bytes(
        bytes: std::borrow::Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    fn to_bytes(
        &self
    ) -> std::borrow::Cow<[u8]> {
        std::borrow::Cow::Owned(Encode!(self).unwrap())
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Block {
    pub fn calc_merkle_root(
        hashes: &Vec<BlockHash>
    ) -> BlockHash {
        match hashes.len() {
            0 => {
                BlockHash::new(U256::ZERO)
            }
            1 => {
                hashes[0].clone()
            }
            _ => {
                let mut hasher = DblHasher::new();

                let mut branches = vec![];

                for chunk in hashes.chunks(2) {
                    let left = &chunk[0];
                    let right = if chunk.len() > 1 {
                        &chunk[1]
                    }
                    else {
                        &chunk[0]
                    };

                    let hash = hasher.hash(&[
                        left.inner, 
                        right.inner
                    ].concat());

                    branches.push(BlockHash::new(hash));
                }

                Self::calc_merkle_root(&branches)
            }
        }
    }
    
    pub fn calc_id(
        &self,
        hasher: &mut DblHasher
    ) -> BlockHash {
        BlockHash::new(hasher.hash(&[
            self.height.to_bytes(),
            self.header.version.to_bytes(),
            self.header.prev.to_bytes(),
            self.header.merkle_root.to_bytes(),
            self.header.timestamp.to_bytes(),
            self.header.bits.to_bytes(),
            self.header.nonce.to_bytes(),
        ].concat()))
    }
}
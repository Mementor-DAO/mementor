use std::borrow::Cow;
use candid::{CandidType, Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use crate::utils::hasher::DblHasher;
use super::block_hash::BlockHash;

pub type TxId = BlockHash;

pub const TX_V1_00: u32 = 1_00;

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub enum MintReason {
    TeamFee,
    TreasuryFee,
    NftMinter(Vec<u128>),
    RaffleWinner,
}

impl MintReason {
    pub fn to_bytes(
        &self
    ) -> Cow<[u8]> {
        Cow::Owned(vec![match self {
            MintReason::TeamFee => 0,
            MintReason::TreasuryFee => 1,
            MintReason::NftMinter(_) => 2,
            MintReason::RaffleWinner => 3,
        }])
    }
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub enum TransactionOp {
    Mint {
        to: Account,
        amount: u64,
        reason: MintReason,
    },
}

impl TransactionOp {
    pub fn to_bytes(
        &self
    ) -> Cow<[u8]> {
        Cow::Owned(match self {
            TransactionOp::Mint { to, amount, reason } => {
                [
                    to.effective_subaccount().to_bytes(),
                    amount.to_bytes(),
                    reason.to_bytes()
                ].concat()
            },
        })
    }
}

#[derive(Clone, Serialize, Deserialize, CandidType)]
pub struct Transaction {
    pub version: u32,
    pub op: TransactionOp,
    pub timestamp: u32,
}
impl Transaction {
    pub fn calc_id(
        &self,
        hasher: &mut DblHasher
    ) -> BlockHash {
        BlockHash::new(hasher.hash(&[
            self.version.to_bytes(),
            self.op.to_bytes(),
            self.timestamp.to_bytes(),
        ].concat()))
    }
}

impl Storable for Transaction {
    fn to_bytes(
        &self
    ) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(
        bytes: Cow<[u8]>
    ) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
use std::{borrow::Cow, collections::HashMap};
use candid::{CandidType, Decode, Encode, Principal};
use ic_ledger_types::AccountIdentifier;
use ic_stable_structures::{storable::Bound, Storable};
use icrc_ledger_types::icrc1::account::Account;
use oc_bots_sdk::types::{MessageId, MessageIndex, TimestampMillis};
use serde::Deserialize;
use super::{blob::BlobId, meme::MemeId, nft::NftId};

pub type UserId = Principal;

#[derive(CandidType, Deserialize, Clone)]
pub struct UserMeme {
    pub meme_id: MemeId,
    pub tmp_blob_id: BlobId,
}

impl UserMeme {
    pub fn new(
        meme_id: MemeId,
        tmp_blob_id: BlobId
    ) -> Self {
        Self { 
            meme_id,
            tmp_blob_id,
        }
    }
}

#[derive(Default, CandidType, Deserialize)]
pub struct UserMemes {
    pub list: HashMap<MemeId, UserMeme>,
    pub last: Option<MemeId>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserPost {
    pub blob_id: BlobId,
    pub meme_id: MemeId,
    pub message_index: MessageIndex,
    pub message_id: MessageId,
}

#[derive(Default, CandidType, Deserialize)]
pub struct UserPosts {
    pub list: HashMap<MemeId, UserPost>,
    pub last: Option<MemeId>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct UserMint {
    pub canister_id: Principal,
    pub token_id: NftId,
    pub timestamp: TimestampMillis,
}

#[derive(Default, CandidType, Deserialize)]
pub struct UserMints {
    pub list: HashMap<MemeId, UserMint>,
    pub last: Option<MemeId>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum UserTransaction {
    NftTransfer {
        token_id: NftId,
        to: Account,
        tx_id: u128,
        timestamp: u32,
    },
    IcpWithdraw {
        amount: u64,
        to: AccountIdentifier,
        block_num: u64,
        timestamp: u32,
    },
}

#[derive(Default, CandidType, Deserialize)]
pub struct User {
    pub memes: UserMemes,
    pub posts: UserPosts,
    pub mints: UserMints,
    pub txs: Vec<UserTransaction>,
}

impl Storable for User {
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
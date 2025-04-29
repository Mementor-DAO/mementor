use std::cell::RefCell;
use ic_stable_structures::BTreeMap;
use crate::{
    memory::{get_blobs_memory, Memory}, 
    types::blob::{Blob, BlobId}, 
    utils::rng
};

use super::temp_blob::{TempBlobStorage, MAX_TEMP_BLOBS};

pub struct BlobStorage;

thread_local! {
    static BLOBS: RefCell<BTreeMap<BlobId, Blob, Memory>> = RefCell::new(
        BTreeMap::init(
            get_blobs_memory()
        )
    );
}

impl BlobStorage {
    pub fn save(
        blob: Blob,
        is_temp: bool
    ) -> BlobId {
        if is_temp {
            TempBlobStorage::save(blob)
        }
        else {
            let id: BlobId = rng::gen_range(MAX_TEMP_BLOBS as u128..u128::MAX);
            BLOBS.with_borrow_mut(|blobs| {
                blobs.insert(id, blob)
            });
            id
        }
    }

    pub fn load(
        id: BlobId
    ) -> Option<Blob> {
        if id < MAX_TEMP_BLOBS as u128 {
            TempBlobStorage::load(id)
        }
        else {
            BLOBS.with_borrow(|blobs| {
                blobs.get(&id)
            })
        }
    }
}
use std::cell::RefCell;
use ic_stable_structures::Vec;
use crate::{
    memory::{get_temp_blobs_memory, Memory}, state, types::blob::{Blob, BlobId} 
};

pub const MAX_TEMP_BLOBS: usize = 10_000;

pub struct TempBlobStorage;

thread_local! {
    static TEMP_BLOBS: RefCell<Vec<Blob, Memory>> = RefCell::new(
        Vec::init(
            get_temp_blobs_memory()
        ).unwrap()
    );
}

impl TempBlobStorage {
    pub fn save(
        blob: Blob
    ) -> BlobId {
        let index = state::mutate(|s| {
            let index = s.temp_bobs_index();
            let last = *index;
            *index = (*index + 1) % MAX_TEMP_BLOBS;
            last
        });

        TEMP_BLOBS.with_borrow_mut(|blobs| {
            if index >= blobs.len() as usize {
                blobs.push(&blob).unwrap();
            }
            else {
                blobs.set(index as _, &blob);
            }
        });

        index as _
    }

    pub fn load(
        id: BlobId
    ) -> Option<Blob> {
        TEMP_BLOBS.with_borrow(|blobs| {
            blobs.get(id as _)
        })
    }
}
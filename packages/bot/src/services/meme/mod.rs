pub mod meme;
pub use meme::*;

use std::sync::RwLock;

static MEME_SERVICE: RwLock<Option<MemeService>> = RwLock::new(None);

pub fn init(
    memes_json_bytes: Vec<u8>,
    index_tar_bytes: Vec<u8>
) {
    *MEME_SERVICE.write().unwrap() = Some(MemeService::new(
        memes_json_bytes,
        index_tar_bytes
    ));
}

pub fn read<F, R>(
    fun: F
) -> R 
    where F: FnOnce(&MemeService) -> R {
    fun(MEME_SERVICE.read().unwrap().as_ref().unwrap())
}

pub fn mutate<F, R>(
    fun: F
) -> R 
    where F: FnOnce(&mut MemeService) -> R {
    fun(MEME_SERVICE.write().unwrap().as_mut().unwrap())
}
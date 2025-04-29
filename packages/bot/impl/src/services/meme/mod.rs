pub mod meme;
pub use meme::*;

use std::sync::RwLock;
use crate::utils::gz::decompress_to_vec;

static MEME_SERVICE: RwLock<Option<MemeService>> = RwLock::new(None);

pub fn init(
    memes_json_gz: Vec<u8>,
    index_tar_gz: Vec<u8>
) -> Result<(), String> {
    let memes_json_bytes = decompress_to_vec(&memes_json_gz)?;
    let index_tar_bytes = decompress_to_vec(&index_tar_gz)?;
    
    *MEME_SERVICE.write().unwrap() = Some(MemeService::new(
        memes_json_bytes,
        index_tar_bytes
    ));
    Ok(())
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
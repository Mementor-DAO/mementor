pub mod nft;
pub use nft::*;

use std::sync::{LazyLock, RwLock};

static NFT_SERVICE: LazyLock<RwLock<NftService>> = LazyLock::new(|| 
    RwLock::new(
        NftService::default()
    )
);

pub fn read<F, R>(
    fun: F
) -> R 
    where F: FnOnce(&NftService) -> R {
    fun(&NFT_SERVICE.read().unwrap())
}

pub fn mutate<F, R>(
    fun: F
) -> R 
    where F: FnOnce(&mut NftService) -> R {
    fun(&mut NFT_SERVICE.write().unwrap())
}
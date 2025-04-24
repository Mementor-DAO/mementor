use std::cell::RefCell;
use ic_stable_structures::BTreeMap;
use crate::{
    memory::{
        get_meme_to_nft_memory, 
        get_nfts_memory, 
        Memory
    }, 
    types::{
        meme::MemeId, nft::{Nft, NftId}
    }
};

pub struct NftStorage;

thread_local! {
    static NFTS: RefCell<BTreeMap<NftId, Nft, Memory>> = RefCell::new(
        BTreeMap::init(
            get_nfts_memory()
        )
    );
    static MEME_TO_NFT: RefCell<BTreeMap<MemeId, NftId, Memory>> = RefCell::new(
        BTreeMap::init(
            get_meme_to_nft_memory()
        )
    );
}

impl NftStorage {
    pub fn save(
        token_id: NftId,
        meme_id: &MemeId,
        nft: Nft
    ) {
        NFTS.with_borrow_mut(|nfts| {
            nfts.insert(token_id, nft)
        });

        MEME_TO_NFT.with_borrow_mut(|dic| {
            dic.insert(meme_id.clone(), token_id)
        });
    }

    pub fn load(
        token_id: &NftId
    ) -> Option<Nft> {
        NFTS.with_borrow(|nfts| {
            nfts.get(token_id)
        })
    }

    pub fn load_by_meme_id(
        meme_id: &MemeId
    ) -> Option<Nft> {
        MEME_TO_NFT.with_borrow(|dic| {
            match dic.get(meme_id) {
                Some(token_id) => {
                    NFTS.with_borrow(|nfts| {
                        nfts.get(&token_id)
                    })
                },
                None => {
                    None
                },
            }
        })
    }

    pub fn size(
    ) -> u64 {
        NFTS.with_borrow(|nfts| nfts.len())
    }
}
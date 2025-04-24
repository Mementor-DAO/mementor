use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

const UPGRADES: MemoryId            = MemoryId::new(0);
const IMAGES: MemoryId              = MemoryId::new(1);
const THUMBS: MemoryId              = MemoryId::new(2);
const BLOBS: MemoryId               = MemoryId::new(3);
const TEMP_BLOBS: MemoryId          = MemoryId::new(4);
const USERS: MemoryId               = MemoryId::new(5);
const NFTS: MemoryId                = MemoryId::new(6);
const MEME_TO_NFT: MemoryId         = MemoryId::new(7);
const EVENTS: MemoryId              = MemoryId::new(9);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl>
        = MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 128);
}

fn get_memory(id: MemoryId) -> Memory {
    MEMORY_MANAGER.with(|m| m.get(id))
}

pub fn get_upgrades_memory() -> Memory {
    get_memory(UPGRADES)
}

pub fn get_images_memory() -> Memory {
    get_memory(IMAGES)
}

pub fn get_thumbs_memory() -> Memory {
    get_memory(THUMBS)
}

pub fn get_blobs_memory() -> Memory {
    get_memory(BLOBS)
}

pub fn get_temp_blobs_memory() -> Memory {
    get_memory(TEMP_BLOBS)
}

pub fn get_users_memory() -> Memory {
    get_memory(USERS)
}

pub fn get_nfts_memory() -> Memory {
    get_memory(NFTS)
}

pub fn get_meme_to_nft_memory() -> Memory {
    get_memory(MEME_TO_NFT)
}

pub fn get_events_memory() -> Memory {
    get_memory(EVENTS)
}


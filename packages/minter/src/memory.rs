use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

const UPGRADES: MemoryId            = MemoryId::new(0);
const BLOCKS: MemoryId              = MemoryId::new(1);
const HEIGHT_TO_BLOCKID: MemoryId   = MemoryId::new(2);
const TRANSACTIONS: MemoryId        = MemoryId::new(3);
const TRANSACTIONS_ORD: MemoryId    = MemoryId::new(4);

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

pub fn get_blocks_memory() -> Memory {
    get_memory(BLOCKS)
}

pub fn get_height_to_blockid_memory() -> Memory {
    get_memory(HEIGHT_TO_BLOCKID)
}

pub fn get_txs_memory() -> Memory {
    get_memory(TRANSACTIONS)
}

pub fn get_txs_ord_memory() -> Memory {
    get_memory(TRANSACTIONS_ORD)
}


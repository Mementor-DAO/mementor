use std::cell::RefCell;
use ic_stable_structures::BTreeMap;
use crate::{
    memory::{
        get_blocks_memory, 
        get_height_to_blockid_memory, 
        Memory
    }, 
    types::{
        block::Block, 
        block_hash::BlockId
    }
};

pub struct BlockStorage;

thread_local! {
    static BLOCKS: RefCell<BTreeMap<BlockId, Block, Memory>> = RefCell::new(
        BTreeMap::init(
            get_blocks_memory()
        )
    );
    static HEIGHT_TO_BLOCKID: RefCell<BTreeMap<u32, BlockId, Memory>> = RefCell::new(
        BTreeMap::init(
            get_height_to_blockid_memory()
        )
    );
}

impl BlockStorage {
    pub fn save(
        id: BlockId,
        block: Block
    ) {
        HEIGHT_TO_BLOCKID.with_borrow_mut(|map| {
            map.insert(block.height, id.clone());
        });
        BLOCKS.with_borrow_mut(|blocks| {
            blocks.insert(id, block)
        });
    }

    #[allow(unused)]
    pub fn load(
        id: &BlockId
    ) -> Option<Block> {
        BLOCKS.with_borrow(|blocks| {
            blocks.get(&id)
        })
    }

    #[allow(unused)]
    pub fn load_by_height(
        height: u32
    ) -> Option<Block> {
        HEIGHT_TO_BLOCKID.with_borrow(|map| {
            if let Some(id) = map.get(&height) {
                BLOCKS.with_borrow(|blocks| {
                    blocks.get(&id)
                })
            }
            else {
                None
            }
        })
    }
}
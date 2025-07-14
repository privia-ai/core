mod tokens_storage;
mod index_storage;
mod metadata_storage;

pub use tokens_storage::TokenStoreStable;
pub use metadata_storage::MetadataStoreStable;
pub use  index_storage::IndexStoreStable;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

type IcpMemory = VirtualMemory<DefaultMemoryImpl>;

const TOKENS_MEMORY_ID: MemoryId = MemoryId::new(1);
const TX_INDEX_MEMORY: MemoryId = MemoryId::new(2);
const DUPLICATE_MEMORY: MemoryId = MemoryId::new(3);
const METADATA_MEMORY: MemoryId = MemoryId::new(4);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

fn get_tokens_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TOKENS_MEMORY_ID))
}

fn get_tx_index_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TX_INDEX_MEMORY))
}

fn get_duplicate_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DUPLICATE_MEMORY))
}

fn get_metadata_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(METADATA_MEMORY))
}


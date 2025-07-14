use ic_stable_structures::{StableBTreeMap, StableCell};
use crate::domain::interfaces::storage::{IIndexStore};
use crate::icp::stable_storage::{get_duplicate_memory, get_tx_index_memory, IcpMemory};

pub struct IndexStoreStable {
    index: StableCell<u64, IcpMemory>,
    seen: StableBTreeMap<u64, u64, IcpMemory>
}

impl IndexStoreStable {
    pub fn init() -> Self {
        Self {
            index: StableCell::init(get_tx_index_memory(), 0).unwrap(),
            seen: StableBTreeMap::init(get_duplicate_memory())
        }
    }
}

impl IIndexStore for IndexStoreStable {
    fn next_index(&mut self) -> u64 {
        let current = self.index.get().clone();
        self.index.set(current + 1).unwrap();
        current
    }

    fn is_duplicate(&self, created_at: u64) -> Option<u64> {
        self.seen.get(&created_at)
    }

    fn record(&mut self, created_at: u64, index: u64) {
        self.seen.insert(created_at, index);
    }
}
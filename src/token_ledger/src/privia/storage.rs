use crate::privia::{IStakingRegistry, StakingLogEntry};
use candid::Principal;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableLog, Storable};
use std::borrow::Cow;
use std::cell::RefCell;

type IcpMemory = VirtualMemory<DefaultMemoryImpl>;

const STAKING_MAPPING_MEMORY_ID: MemoryId = MemoryId::new(10);
const STAKING_LOG_DATA_MEMORY_ID: MemoryId = MemoryId::new(11);
const STAKING_LOG_INDEX_MEMORY_ID: MemoryId = MemoryId::new(12);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static STAKINGS: RefCell<StakingRegistryStable> = RefCell::new(StakingRegistryStable::init());
}

fn get_log_index_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_MAPPING_MEMORY_ID))
}

fn get_log_idx_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_LOG_INDEX_MEMORY_ID))
}

fn get_log_data_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_LOG_DATA_MEMORY_ID))
}

impl Storable for StakingLogEntry {
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut buf = vec![];
        ciborium::ser::into_writer(&self, &mut buf).unwrap();
        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        ciborium::de::from_reader(bytes.as_ref()).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub struct StakingRegistryStable {
    pub log_index: StableBTreeMap<(Principal, u64), u64, IcpMemory>,
    pub log: StableLog<StakingLogEntry, IcpMemory, IcpMemory>,
}

impl StakingRegistryStable {
    pub fn init() -> Self {
        Self {
            log_index: StableBTreeMap::init(get_log_index_memory()),
            log: StableLog::init(get_log_idx_memory(), get_log_data_memory())
                .expect("log initialization failed"),
        }
    }
}

impl IStakingRegistry for StakingRegistryStable {
    fn get_log_entries(
        &self,
        address: Principal,
        from: u64,
        to: u64,
    ) -> Vec<StakingLogEntry> {
        let mut result: Vec<StakingLogEntry> = Vec::new();

        let indexes = self
            .log_index
            .range((address, from.clone())..(address, to.clone()));

        for index in indexes {
            let log_entry = self
                .log
                .get(index.1)
                .expect(&format!("log entry with index {} not found", index.1));

            result.push(log_entry);
        }

        result
    }

    fn get_latest_log_entry(
        &self,
        address: Principal,
        timestamp: u64,
    ) -> Option<StakingLogEntry> {
        let latest_index = self.log_index.range(..=(address, timestamp)).last();
        match latest_index {
            None => None,
            Some(index) => self.log.get(index.1),
        }
    }

    fn add_log_entry(&mut self, address: Principal, log: &StakingLogEntry) {
        let entry_index = self.log.append(log).expect("log append failed");
        self.log_index.insert((address, log.timestamp.clone()), entry_index);
    }
}

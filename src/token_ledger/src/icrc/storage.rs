use super::types::Configuration;
use candid::{Decode, Encode};
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::Bound,
    DefaultMemoryImpl, StableCell, StableVec, Storable,
};
use icrc_ledger_types::icrc3::transactions::Transaction;
use std::{borrow::Cow, cell::RefCell};

type VMem = VirtualMemory<DefaultMemoryImpl>;

const CONFIGURATION_MEMORY_ID: MemoryId = MemoryId::new(1);
const TRANSACTION_LOG_MEMORY_ID: MemoryId = MemoryId::new(2);
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STATE: RefCell<Ledger> = MEMORY_MANAGER.with(|cell| {
        let mm = cell.borrow();
        let configuration = ConfigCell::init(mm.get(CONFIGURATION_MEMORY_ID), Configuration::default())
            .expect("failed to initialize the config cell");
        let transaction_log = TransactionLog::init(mm.get(TRANSACTION_LOG_MEMORY_ID))
            .expect("failed to initialize the transaction log");
        RefCell::new(Ledger {
            configuration,
            transaction_log,
        })
    });
}

pub fn with_ledger<R>(f: impl FnOnce(&Ledger) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}

pub fn with_ledger_mut<R>(f: impl FnOnce(&mut Ledger) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

pub struct Ledger {
    pub configuration: ConfigCell,
    pub transaction_log: TransactionLog,
}

pub type ConfigCell = StableCell<Configuration, VMem>;
pub type TransactionLog = StableVec<StorableTransaction, VMem>;
pub struct StorableTransaction(pub Transaction);

impl Storable for Configuration {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self).expect("failed to serialize Configuration").into())
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Decode!(&bytes, Configuration).expect("failed to deserialize Configuration")
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for StorableTransaction {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(Encode!(&self.0).expect("failed to serialize Transaction").into())
    }
    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(Decode!(&bytes, Transaction).expect("failed to deserialize Transaction"))
    }
    const BOUND: Bound = Bound::Bounded {
        max_size: 1000,
        is_fixed_size: false,
    };
}

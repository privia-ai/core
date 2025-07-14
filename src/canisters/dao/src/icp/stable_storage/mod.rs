mod config_storage;
mod discount_storage;
mod hiving_storage;
mod voting_storage;

pub use config_storage::ConfigStorageStable;
pub use discount_storage::DiscountStorageStable;
pub use voting_storage::VotingStorageStable;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::DefaultMemoryImpl;
use std::cell::RefCell;

type IcpMemory = VirtualMemory<DefaultMemoryImpl>;

const PROPOSALS_MEMORY_ID: MemoryId = MemoryId::new(1);
const VOTES_MEMORY_ID: MemoryId = MemoryId::new(2);
const CONFIG_MEMORY_ID: MemoryId = MemoryId::new(3);
const HIVING_WALLETS_MEMORY_ID: MemoryId = MemoryId::new(4);
const WALLET_USAGES_MEMORY_ID: MemoryId = MemoryId::new(5);
const CYCLE_DISCOUNTS_INDEX_MEMORY_ID: MemoryId = MemoryId::new(6);
const DISCOUNTS_MEMORY_ID: MemoryId = MemoryId::new(7);
const ACCOUNT_CYCLE_INDEX_MEMORY_ID: MemoryId = MemoryId::new(8);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
}

fn get_proposals_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(PROPOSALS_MEMORY_ID))
}

fn get_votes_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(VOTES_MEMORY_ID))
}

fn get_config_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CONFIG_MEMORY_ID))
}

fn get_hiving_wallets_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(HIVING_WALLETS_MEMORY_ID))
}

fn get_wallet_usages_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(WALLET_USAGES_MEMORY_ID))
}

fn get_cycle_discounts_index_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CYCLE_DISCOUNTS_INDEX_MEMORY_ID))
}

fn get_discounts_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(DISCOUNTS_MEMORY_ID))
}

fn get_account_cycles_index_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(ACCOUNT_CYCLE_INDEX_MEMORY_ID))
}

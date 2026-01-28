use crate::domain::interfaces::storage::*;
use crate::icp::stable_storage::{get_wallet_usages_memory, IcpMemory};
use candid::Principal;
use ic_stable_structures::{StableBTreeMap};
use icrc_ledger_types::icrc1::account::Account;

pub struct HivingStorageStorable {
    wallet_usages: StableBTreeMap<(u64, Account), u32, IcpMemory>,
    hiving_canisters: StableBTreeMap<Principal, (), IcpMemory>,
}

impl IHivingStorage for HivingStorageStorable {
    fn add_hiving_canister(&mut self, canister_id: Principal) {
        self.hiving_canisters.insert(canister_id, ());
    }

    fn remove_hiving_canister(&mut self, canister_id: Principal) {
        self.hiving_canisters.remove(&canister_id).unwrap();
    }

    fn get_hiving_canisters(&self) -> Vec<Principal> {
        todo!()
    }

    fn add_wallet_usage_per_cycle(&mut self, cycle_number: u64, wallet: Account) -> u32 {
        let current_usage = self.get_wallet_usage_per_cycle(cycle_number, wallet);
        let new_usage = current_usage + 1;
        self.wallet_usages.insert((cycle_number, wallet), current_usage + 1);
        new_usage
    }

    fn get_wallet_usage_per_cycle(&self, cycle_number: u64, wallet: Account) -> u32 {
        self.wallet_usages.get(&(cycle_number, wallet)).unwrap_or(0)
    }
}

impl HivingStorageStorable {
    pub fn init() -> Self {
        Self {
            wallet_usages: StableBTreeMap::init(get_wallet_usages_memory()),
            hiving_canisters: StableBTreeMap::init(get_wallet_usages_memory()),
        }
    }
}

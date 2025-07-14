use crate::domain::interfaces::storage::*;
use crate::icp::stable_storage::{get_hiving_wallets_memory, get_wallet_usages_memory, IcpMemory};
use ic_stable_structures::{StableBTreeMap, StableVec};
use icrc_ledger_types::icrc1::account::Account;

pub struct HivingStorageStorable {
    hiving_wallets: StableVec<Account, IcpMemory>,
    wallet_usages: StableBTreeMap<(u64, Account), u32, IcpMemory>,
}

impl IHivingStorage for HivingStorageStorable {
    fn add_hiving_wallet(&mut self, wallet: Account) {
        self.hiving_wallets.push(&wallet).unwrap()
    }

    fn get_hiving_wallets(&self) -> Vec<Account> {
        self.hiving_wallets.iter().collect()
    }

    fn add_wallet_usage_per_cycle(&mut self, cycle_number: u64, wallet: Account) -> u32 {
        let current_usage = self.get_wallet_usage_per_cycle(cycle_number, wallet);
        let new_usage = current_usage + 1;
        self.wallet_usages
            .insert((cycle_number, wallet), current_usage + 1);
        new_usage
    }

    fn get_wallet_usage_per_cycle(&self, cycle_number: u64, wallet: Account) -> u32 {
        self.wallet_usages.get(&(cycle_number, wallet)).unwrap_or(0)
    }
}

impl HivingStorageStorable {
    pub fn init() -> Self {
        Self {
            hiving_wallets: StableVec::init(get_hiving_wallets_memory()).unwrap(),
            wallet_usages: StableBTreeMap::init(get_wallet_usages_memory()),
        }
    }
}

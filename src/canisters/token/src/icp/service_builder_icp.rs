use super::stable_storage::{BalanceStoreStable, ConfigurationStoreStable, StakingStoreStable, TransactionsStoreStable};
use crate::domain::interfaces::{IBalanceStore, IConfigurationStore, IStakingStore, ITransactionStore};
use abstractions::runtime::ICanisterRuntime;
use canister_runtime::RuntimeIcp;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static RUNTIME: Rc<RefCell<dyn ICanisterRuntime>> = Rc::new(RefCell::new(RuntimeIcp::new()));
    static CONFIG_STORAGE: Rc<RefCell<dyn IConfigurationStore>> = Rc::new(RefCell::new(ConfigurationStoreStable::init()));
    static BALANCES: Rc<RefCell<dyn IBalanceStore>> = Rc::new(RefCell::new(BalanceStoreStable::init()));
    static TRANSACTIONS: Rc<RefCell<dyn ITransactionStore>> = Rc::new(RefCell::new(TransactionsStoreStable::init()));
    static STAKING: Rc<RefCell<dyn IStakingStore>> = Rc::new(RefCell::new(StakingStoreStable::init()));
}

pub fn build_runtime() -> Rc<RefCell<dyn ICanisterRuntime>> {
    RUNTIME.with(|rc| rc.clone())
}

pub fn build_config_storage() -> Rc<RefCell<dyn IConfigurationStore>> {
    CONFIG_STORAGE.with(|rc| rc.clone())
}

pub fn build_balances_storage() -> Rc<RefCell<dyn IBalanceStore>> {
    BALANCES.with(|rc| rc.clone())
}

pub fn build_transacions_storage() -> Rc<RefCell<dyn ITransactionStore>> {
    TRANSACTIONS.with(|rc| rc.clone())
}

pub fn build_staking_storage() -> Rc<RefCell<dyn IStakingStore>> {
    STAKING.with(|rc| rc.clone())
}

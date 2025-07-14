use std::cell::RefCell;
use std::rc::Rc;
use icrc_ledger_types::icrc1::account::Account;
use crate::domain::interfaces::storage::*;

pub struct HivingService {
    storage: Rc<RefCell<dyn IHivingStorage>>
}

impl HivingService {
    pub fn new(storage: Rc<RefCell<dyn IHivingStorage>>) -> Self {
        Self { storage }
    }

    pub fn get_hiving_wallets(&self) -> Vec<Account> {
        self.storage.borrow().get_hiving_wallets()
    }
}
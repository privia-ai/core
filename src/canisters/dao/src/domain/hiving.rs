use std::cell::RefCell;
use std::rc::Rc;
use candid::Principal;
use abstractions::runtime::ICanisterRuntime;
use crate::domain::interfaces::storage::*;

pub struct HivingService {
    storage: Rc<RefCell<dyn IHivingStorage>>,
    runtime: Rc<RefCell<dyn ICanisterRuntime>>,
}

impl HivingService {
    pub fn new(storage: Rc<RefCell<dyn IHivingStorage>>, runtime: Rc<RefCell<dyn ICanisterRuntime>>) -> Self {
        Self { storage, runtime }
    }

    pub fn add_hiving_canister(&self) {
        let caller = self.runtime.borrow().get_caller();
        self.storage.borrow_mut().add_hiving_canister(caller);
    }

    pub fn remove_hiving_canister(&self) {
        let caller = self.runtime.borrow().get_caller();
        self.storage.borrow_mut().remove_hiving_canister(caller);
    }

    pub fn get_hiving_canisters(&self) -> Vec<Principal>{
        self.storage.borrow().get_hiving_canisters()
    }
}
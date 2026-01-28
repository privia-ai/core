use super::stable_storage::{ConfigStorageStable, DiscountStorageStable, HivingStorageStorable, VotingStorageStable};
use crate::app::IConfigStorage;
use crate::domain::interfaces::storage::*;
use abstractions::nft::NftClient;
use abstractions::runtime::ICanisterRuntime;
use abstractions::token::TokenClient;
use candid::Principal;
use canister_runtime::{CdkCallContext, RuntimeIcp};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static RUNTIME: Rc<RefCell<dyn ICanisterRuntime>> = Rc::new(RefCell::new(RuntimeIcp::new()));

    static VOTING_STORAGE: Rc<RefCell<dyn IVotingStorage>> = Rc::new(RefCell::new(VotingStorageStable::init()));
    static CONFIG_STORAGE: Rc<RefCell<dyn IConfigStorage>> = Rc::new(RefCell::new(ConfigStorageStable::init()));
    static DISCOUNT_STORAGE: Rc<RefCell<dyn IDiscountStorage>> = Rc::new(RefCell::new(DiscountStorageStable::init()));
    static HIVING_STORAGE: Rc<RefCell<dyn IHivingStorage>> = Rc::new(RefCell::new(HivingStorageStorable::init()));
}

pub fn build_runtime() -> Rc<RefCell<dyn ICanisterRuntime>> {
    RUNTIME.with(|rc| rc.clone())
}

pub fn build_voting_storage() -> Rc<RefCell<dyn IVotingStorage>> {
    VOTING_STORAGE.with(|rc| rc.clone())
}

pub fn build_config_storage() -> Rc<RefCell<dyn IConfigStorage>> {
    CONFIG_STORAGE.with(|rc| rc.clone())
}

pub fn build_discount_storage() -> Rc<RefCell<dyn IDiscountStorage>> {
    DISCOUNT_STORAGE.with(|rc| rc.clone())
}

pub fn build_hiving_storage() -> Rc<RefCell<dyn IHivingStorage>> {
    HIVING_STORAGE.with(|rc| rc.clone())
}

pub fn build_token_service(canister_id: Principal) -> Rc<RefCell<TokenClient<CdkCallContext>>> {
    let runtime = CdkCallContext {};
    let client = TokenClient {
        runtime: Rc::new(RefCell::new(runtime)),
        canister_id,
    };
    Rc::new(RefCell::new(client))
}

pub fn build_nft_service(canister_id: Principal) -> Rc<RefCell<NftClient<CdkCallContext>>> {
    let runtime = CdkCallContext {};
    let client = NftClient {
        runtime: Rc::new(RefCell::new(runtime)),
        canister_id,
    };
    Rc::new(RefCell::new(client))
}

use crate::{
    app::IConfigStorage,
    domain::{
        cycles::CycleService, discounts::DiscountService, hiving::HivingService, interfaces::storage::*, staking::StakingService,
        voting::VotingService,
    },
    icp::service_builder_icp,
};

use abstractions::{nft::NftClient, runtime::ICanisterRuntime, token::TokenClient};
use canister_runtime::CdkCallContext;
use std::{cell::RefCell, rc::Rc};

// runtime

pub fn build_runtime() -> Rc<RefCell<dyn ICanisterRuntime>> {
    service_builder_icp::build_runtime()
}

// storages

pub fn build_config_storage() -> Rc<RefCell<dyn IConfigStorage>> {
    service_builder_icp::build_config_storage()
}

fn build_voting_storage() -> Rc<RefCell<dyn IVotingStorage>> {
    service_builder_icp::build_voting_storage()
}

fn build_discount_storage() -> Rc<RefCell<dyn IDiscountStorage>> {
    service_builder_icp::build_discount_storage()
}

fn build_hiving_storage() -> Rc<RefCell<dyn IHivingStorage>> {
    service_builder_icp::build_hiving_storage()
}

// canister clients

pub fn build_token_service() -> Rc<RefCell<TokenClient<CdkCallContext>>> {
    let token_canister_id = build_config_storage().borrow().get_config().token_canister_id;
    service_builder_icp::build_token_service(token_canister_id)
}

pub fn build_nft_service() -> Rc<RefCell<NftClient<CdkCallContext>>> {
    let nft_canister_id = build_config_storage().borrow().get_config().nft_canister_id;
    service_builder_icp::build_nft_service(nft_canister_id)
}

// domain services

pub fn build_voting_service() -> VotingService {
    let cycles_service = build_cycles_service();
    let voting_storage = build_voting_storage();
    let runtime = build_runtime();
    let token = build_token_service();

    let voting_service = VotingService::new(cycles_service, voting_storage, runtime, token);

    voting_service
}

pub fn build_cycles_service() -> Rc<RefCell<CycleService>> {
    let cycles_config = build_config_storage().borrow().get_config().cycles.clone();
    let runtime = build_runtime();
    let cycle_service = CycleService::new(cycles_config, runtime);
    let cycle_service = Rc::new(RefCell::new(cycle_service));

    cycle_service
}

pub fn build_discount_service() -> DiscountService {
    let config = build_config_storage().borrow().get_config().discounts.clone();
    let cycles = build_cycles_service();
    let storage = build_discount_storage();
    let nft = build_nft_service();
    let staking = Rc::new(RefCell::new(build_staking_service()));
    let runtime = build_runtime();

    DiscountService::new(config, cycles, storage, nft, staking, runtime)
}

pub fn build_staking_service() -> StakingService {
    let config = build_config_storage().borrow().get_config().staking.clone();
    let token = build_token_service();
    let cycles_service = build_cycles_service();

    StakingService::new(config, token, cycles_service)
}

pub fn build_hiving_service() -> HivingService {
    let storage = build_hiving_storage();
    let runtime = build_runtime();

    HivingService::new(storage, runtime)
}

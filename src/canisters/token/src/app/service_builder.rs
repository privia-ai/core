use crate::domain::token::TokenService;
use crate::domain::StakingService;
use crate::icp::service_builder_icp;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_token_service() -> Rc<RefCell<TokenService>> {
    let runtime = service_builder_icp::build_runtime();
    let staking = build_staking_service();
    let configuration = service_builder_icp::build_config_storage();
    let balances = service_builder_icp::build_balances_storage();
    let transactions = service_builder_icp::build_transacions_storage();

    Rc::new(RefCell::new(TokenService::new(
        runtime,
        staking,
        configuration,
        balances,
        transactions,
    )))
}

pub fn build_staking_service() -> Rc<RefCell<StakingService>> {
    let staking_store = service_builder_icp::build_staking_storage();
    Rc::new(RefCell::new(StakingService::new(staking_store)))
}
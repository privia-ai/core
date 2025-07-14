use crate::app::service_builder;

pub mod mgmt {
    use super::*;
    use crate::app::app_services::config::AppConfig;

    pub fn init(config: AppConfig) {
        let config_storage = service_builder::build_config_storage();
        config_storage.borrow_mut().set_config(config)
    }
}

pub mod discounts {
    use super::*;
    use abstractions::dao::{Cycle, Discount};
    use candid::Nat;
    use icrc_ledger_types::icrc1::account::Account;

    pub fn get_current_cycle() -> Cycle {
        let service = service_builder::build_cycles_service();
        let result = service.borrow().get_current_cycle();
        result
    }

    pub async fn get_discount(discount_id: u128) -> Discount {
        let service = service_builder::build_discount_service();
        let result = service.get_discount(discount_id).await;
        result
    }

    pub async fn mint_discount(discount: Discount) -> u128 {
        let service = service_builder::build_discount_service();
        let result = service.mint_discount(discount).await;
        result
    }

    pub async fn get_staking_score(principal: Account) -> Nat {
        let service = service_builder::build_staking_service();
        let result = service.get_current_staking_score(principal).await;
        result
    }

    pub async fn calculate_discount(principal: Account, price: u128) -> f32 {
        let service = service_builder::build_discount_service();
        let result = service.get_max_discount(principal, price).await;
        result
    }
}

pub mod voting {
    use super::*;
    use abstractions::dao::{CodeProposalData, Proposal, ProposalType, Vote, VoteOption};

    pub async fn voting_create_proposal(proposal_type: ProposalType, data: String) -> u64 {
        match proposal_type {
            ProposalType::UpdateCode => {
                if let Err(msg) = validate_code_proposal(data.clone()) {
                    panic!("{msg}")
                }
            }
            _ => {}
        };
        let voting_service = service_builder::build_voting_service();
        let proposal_id = voting_service.create_proposal(proposal_type, data).await;

        proposal_id
    }

    fn validate_code_proposal(json: String) -> Result<CodeProposalData, String> {
        match serde_json::from_str::<CodeProposalData>(&json) {
            Ok(data) => Ok(data),
            Err(err) => Err(format!("Failed to parse JSON: {}", err)),
        }
    }

    pub fn voting_get_proposal(proposal_id: u64) -> Option<Proposal> {
        let voting_service = service_builder::build_voting_service();
        let proposal = voting_service.get_proposal(&proposal_id);

        proposal
    }

    pub async fn voting_vote(proposal_id: u64, vote: VoteOption) -> u64 {
        let mut voting_service = service_builder::build_voting_service();
        voting_service.vote(proposal_id, vote).await
    }

    pub fn voting_get_vote(vote_id: u64) -> Option<Vote> {
        service_builder::build_voting_service().get_vote(&vote_id)
    }

    pub fn voting_get_all_votes(proposal_id: u64) -> Vec<Vote> {
        service_builder::build_voting_service().get_all_votes(&proposal_id)
    }
}

pub mod config {
    use super::{super::IConfigStorage, service_builder};
    use crate::domain::cycles::CyclesConfig;
    use crate::domain::staking::StakingConfig;
    use candid::{CandidType, Deserialize, Principal};
    use serde::Serialize;
    use std::cell::RefCell;
    use std::rc::Rc;
    use crate::domain::discounts::DiscountConfig;

    pub struct ConfigService {
        storage: Rc<RefCell<dyn IConfigStorage>>,
    }

    impl ConfigService {
        pub fn new(storage: Rc<RefCell<dyn IConfigStorage>>) -> Self {
            Self { storage }
        }

        pub fn set_config(&self, config: AppConfig) {
            self.storage.borrow_mut().set_config(config);
        }

        pub fn get_config(&self) -> AppConfig {
            self.storage.borrow().get_config()
        }
    }

    pub fn set_config(config: AppConfig) {
        let config_service = build_config_service();
        config_service.set_config(config);
    }

    pub fn get_config() -> AppConfig {
        let config_service = build_config_service();
        config_service.get_config()
    }

    fn build_config_service() -> ConfigService {
        let config_storage = service_builder::build_config_storage();
        ConfigService::new(config_storage)
    }

    #[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
    pub struct AppConfig {
        pub staking: StakingConfig,
        pub cycles: CyclesConfig,
        pub discounts: DiscountConfig,
        pub token_canister_id: Principal,
        pub nft_canister_id: Principal,
    }

    impl Default for AppConfig {
        fn default() -> Self {
            Self {
                staking: StakingConfig::default(),
                cycles: CyclesConfig::default(),
                discounts: DiscountConfig::default(),
                token_canister_id: Principal::anonymous(),
                nft_canister_id: Principal::anonymous(),
            }
        }
    }
}

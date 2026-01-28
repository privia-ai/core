mod calculators;

use calculators::ProportionCalculator;

use super::cycles::CycleService;
use super::interfaces::storage::*;
use super::staking::StakingService;

use abstractions::dao::{Cycle, Discount, DiscountRequest};
use abstractions::MetadataValue;
use abstractions::nft::NftClient;
use canister_runtime::CdkCallContext;

use std::cell::RefCell;
use std::rc::Rc;
use candid::{CandidType, Deserialize};
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;
use abstractions::runtime::ICanisterRuntime;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct DiscountConfig {
    pub discounts_per_cycle: u128,
}

impl Default for DiscountConfig {
    fn default() -> Self {
        Self {
            discounts_per_cycle: 5,
        }
    }
}

pub struct DiscountService {
    config: DiscountConfig,
    cycles: Rc<RefCell<CycleService>>,
    storage: Rc<RefCell<dyn IDiscountStorage>>,
    nft: Rc<RefCell<NftClient<CdkCallContext>>>,
    staking: Rc<RefCell<StakingService>>,
    runtime: Rc<RefCell<dyn ICanisterRuntime>>,
 }

impl DiscountService {
    const MAX_DISCOUNT: f32 = 25.0;

    pub fn new(
        config: DiscountConfig,
        cycles: Rc<RefCell<CycleService>>,
        storage: Rc<RefCell<dyn IDiscountStorage>>,
        nft: Rc<RefCell<NftClient<CdkCallContext>>>,
        staking: Rc<RefCell<StakingService>>,
        runtime: Rc<RefCell<dyn ICanisterRuntime>>,
    ) -> Self {
        Self {
            config,
            cycles,
            storage,
            nft,
            staking,
            runtime
        }
    }

    pub async fn get_max_discount(&self, hiver: Account, price: u128) -> f32 {
        let score = self
            .staking
            .borrow()
            .get_current_staking_score(hiver)
            .await;

        let calculator = self.build_calculator();
        let result = calculator.calculate_discount(price, score);

        result
    }

    fn build_calculator(&self) -> ProportionCalculator {
        ProportionCalculator::new(Self::MAX_DISCOUNT)
    }

    fn validate_account(&self, account: &Account, cycle: &Cycle) -> Result<(), String> {
        let discounts_count = self.storage.borrow().get_discount_index(account, cycle.number);
        if discounts_count >= self.config.discounts_per_cycle {
            return Err("Max number of discounts per cycle reached".to_string())
        }

        Ok(())
    }

    pub async fn mint_discount(&self, hiver: Account, discount_request: DiscountRequest) -> u128 {
        let current_cycle = self.cycles.borrow().get_current_cycle();
        if let Err(msg) = self.validate_account(&hiver, &current_cycle) {
             panic!("{}", msg);
        }

        let token_id = self.nft.borrow().privia_mint_token(discount_request.owner, discount_request.to_metadata()).await.unwrap();

        let discount = Discount::new(token_id, discount_request.value, discount_request.owner);
        self.storage
            .borrow_mut()
            .add_discount(current_cycle.number, discount);

        self.storage.borrow_mut().increase_discount_index(hiver, current_cycle.number);
        token_id
    }

    pub async fn get_discount(&self, token_id: u128) -> Discount {
        let req_param = Vec::from([token_id]);

        let metadata_response = self.nft.borrow().icrc7_token_metadata(req_param.clone()).await.unwrap();
        let metadata_opt = metadata_response.first().unwrap().clone();
        let metadata = metadata_opt.unwrap();

        let owner_response = self.nft.borrow().icrc7_owner_of(req_param).await.unwrap();
        let owner_opt = owner_response.first().unwrap().clone();
        let owner = owner_opt.unwrap();

        let discount = Self::build_discount(token_id, owner, &metadata);
        discount
    }

    fn build_discount(id: u128, owner: Account, metadata: &Vec<(String, MetadataValue)>) -> Discount {
        let discount_value_md =
            Self::find_metadata_value(metadata, "value".to_string()).unwrap();
        let discount_value = match discount_value_md {
            MetadataValue::Text(discount_value) => discount_value.parse::<f32>().unwrap(),
            _ => panic!("value"),
        };

        let discount = Discount {
            id,
            owner,
            value: discount_value,
        };

        discount
    }

    fn find_metadata_value(
        all: &Vec<(String, MetadataValue)>,
        key: String,
    ) -> Option<MetadataValue> {
        for md in all {
            if md.0 == key {
                return Some(md.1.clone());
            }
        }
        None
    }
}

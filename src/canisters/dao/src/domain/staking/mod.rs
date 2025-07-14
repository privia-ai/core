mod scorers;

use crate::domain::cycles::CycleService;
use abstractions::Timestamp;
use abstractions::token::{StakingLogResult, TokenClient};
use candid::{CandidType, Deserialize, Nat};
use canister_runtime::CdkCallContext;
use icrc_ledger_types::icrc1::account::Account;
use scorers::LinearMinScorer;
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct StakingConfig {}

impl Default for StakingConfig {
    fn default() -> Self {
        Self {}
    }
}

pub struct StakingService {
    config: StakingConfig,
    tokens: Rc<RefCell<TokenClient<CdkCallContext>>>,
    cycles: Rc<RefCell<CycleService>>,
}

impl StakingService {
    pub fn new(
        config: StakingConfig,
        tokens: Rc<RefCell<TokenClient<CdkCallContext>>>,
        cycles: Rc<RefCell<CycleService>>,
    ) -> Self {
        Self {
            config,
            tokens,
            cycles,
        }
    }

    pub async fn get_staking_log(
        &self,
        wallet: Account,
        start: Option<Timestamp>,
        end: Option<Timestamp>,
    ) -> StakingLogResult {
        self.tokens
            .borrow()
            .privia_staking_log(wallet, start, end)
            .await
            .unwrap()
    }

    pub async fn get_current_staking_score(&self, wallet: Account) -> Nat {
        let current_cycle = self.cycles.borrow().get_current_cycle();
        let log: StakingLogResult = self
            .tokens
            .borrow()
            .privia_staking_log(wallet, None, Some(current_cycle.start))
            .await
            .unwrap();

        let score_calculator = LinearMinScorer::new(Rc::clone(&self.cycles));
        let score = score_calculator
            .calculate_score(&log.log, current_cycle.number)
            .await;

        score
    }
}
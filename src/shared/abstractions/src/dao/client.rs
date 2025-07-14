use crate::dao::{Cycle, Discount, Proposal, ProposalType, Vote, VoteOption};
use crate::runtime::{CallMode, ICallContext};
use candid::{Encode, Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DaoClient<R: ICallContext> {
    pub runtime: Rc<RefCell<R>>,
    pub canister_id: Principal,
}

impl<R: ICallContext> DaoClient<R> {
    pub async fn voting_create_proposal(
        &self,
        proposal_type: ProposalType,
        data: String,
    ) -> Result<u64, R::Error> {
        let method = "voting_create_proposal";
        let args = Encode!(&proposal_type, &data).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }

    pub async fn voting_get_proposal(
        &self,
        proposal_id: u64,
    ) -> Result<Option<Proposal>, R::Error> {
        let method = "voting_get_proposal";
        let args = Encode!(&proposal_id).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn voting_vote(
        &self,
        proposal_id: u64,
        vote: VoteOption,
    ) -> Result<u64, R::Error> {
        let method = "voting_vote";
        let args = Encode!(&proposal_id, &vote).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }

    pub async fn voting_get_vote(&self, vote_id: u64) -> Result<Option<Vote>, R::Error> {
        let method = "voting_get_vote";
        let args = Encode!(&vote_id).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn voting_get_all_votes(&self, proposal_id: u64) -> Result<Vec<Vote>, R::Error> {
        let method = "voting_get_all_votes";
        let args = Encode!(&proposal_id).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    // discounts

    pub async fn mint_discount(&self, discount: Discount) -> Result<u128, R::Error> {
        let method = "mint_discount";
        let args = Encode!(&discount).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }

    pub async fn get_staking_score(&self, principal: Account) -> Result<Nat, R::Error> {
        let method = "get_staking_score";
        let args = Encode!(&principal).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn calculate_max_discount(
        &self,
        principal: Account,
        price: u128,
    ) -> Result<f32, R::Error> {
        let method = "calculate_discount";
        let args = Encode!(&principal, &price).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn get_current_cycle(&self) -> Result<Cycle, R::Error> {
        let method = "get_current_cycle";

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, &[])
            .await
    }
}
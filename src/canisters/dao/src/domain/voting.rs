use super::{cycles::CycleService};
use canister_runtime::CdkCallContext;
use std::{cell::RefCell, rc::Rc};
use candid::Nat;
use icrc_ledger_types::icrc1::account::Account;
use abstractions::dao::{Proposal, ProposalType, Vote, VoteOption};
use abstractions::runtime::ICanisterRuntime;
use abstractions::token::TokenClient;
use super::interfaces::storage::IVotingStorage;

pub struct VotingService {
    cycles: Rc<RefCell<CycleService>>,
    storage: Rc<RefCell<dyn IVotingStorage>>,
    runtime: Rc<RefCell<dyn ICanisterRuntime>>,
    token: Rc<RefCell<TokenClient<CdkCallContext>>>
}

impl VotingService {
    pub fn new(
        cycles: Rc<RefCell<CycleService>>,
        storage: Rc<RefCell<dyn IVotingStorage>>,
        runtime: Rc<RefCell<dyn ICanisterRuntime>>,
        token: Rc<RefCell<TokenClient<CdkCallContext>>>
    ) -> Self {
        Self {
            cycles,
            storage,
            runtime,
            token
        }
    }

    pub async fn create_proposal(&self, proposal_type: ProposalType, data: String) -> u64 {
        let now = self.runtime.borrow().get_time();
        let caller = self.runtime.borrow().get_caller();
        let voting_cycle = self.cycles.borrow().get_next_voting_cycle();
        let proposal = Proposal::new(
            now,
            caller,
            proposal_type,
            data,
            voting_cycle.start,
            voting_cycle.end,
        );

        let proposal_id = self.storage.borrow_mut().add_proposal(proposal);

        proposal_id
    }

    pub fn get_proposal(&self, proposal_id: &u64) -> Option<Proposal> {
        self.storage.borrow().get_proposal(proposal_id)
    }

    pub async fn vote(&mut self, proposal_id: u64, vote: VoteOption) -> u64 {
        let proposal = self.get_proposal(&proposal_id);
        if proposal.is_none() {
            panic!("Proposal does not exist!")
        }
        let mut proposal = proposal.unwrap();

        let now = self.runtime.borrow().get_time();
        let caller = self.runtime.borrow().get_caller();

        if now < proposal.start || now > proposal.end {
            panic!("Voting on proposal is not active")
        }

        let caller_acc = Account::from(caller);
        let balance = self.token.borrow().balance_of(caller_acc).await.unwrap();
        if balance <= Nat::from(0u32) {
            panic!("Staking balance is zero! Only token holders are eligible to vote on proposals")
        }

        let vote = Vote::new(proposal_id, caller, now, vote);

        let vote_id = self.storage.borrow_mut().add_vote(vote);
        proposal.votes.push(vote_id);
        self.storage.borrow_mut().update_proposal(proposal);

        vote_id
    }

    pub fn get_vote(&self, vote_id: &u64) -> Option<Vote> {
        self.storage.borrow().get_vote(vote_id)
    }

    pub fn get_all_votes(&self, proposal_id: &u64) -> Vec<Vote> {
        let proposal = self
            .get_proposal(proposal_id)
            .expect("Proposal does not exit");

        let mut result: Vec<Vote> = Vec::new();
        for vote_id in proposal.votes {
            let vote = self.get_vote(&vote_id).unwrap();
            result.push(vote);
        }

        result
    }
}

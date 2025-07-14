use crate::api::app_services::config::AppConfig;
use crate::app::app_services;
use abstractions::dao::*;
use abstractions::Account;
use candid::Nat;
use ic_cdk::{init, query, update};

// canister mgmt

#[init]
fn init(config: AppConfig) {
    app_services::mgmt::init(config);
}

// voting

#[update]
pub async fn voting_create_proposal(proposal_type: ProposalType, data: String) -> u64 {
    app_services::voting::voting_create_proposal(proposal_type, data).await
}

#[query]
pub fn voting_get_proposal(proposal_id: u64) -> Option<Proposal> {
    app_services::voting::voting_get_proposal(proposal_id)
}

#[update]
pub async fn voting_vote(proposal_id: u64, vote: VoteOption) -> u64 {
    app_services::voting::voting_vote(proposal_id, vote).await
}

#[query]
pub fn voting_get_vote(vote_id: u64) -> Option<Vote> {
    app_services::voting::voting_get_vote(vote_id)
}

#[query]
pub fn voting_get_all_votes(proposal_id: u64) -> Vec<Vote> {
    app_services::voting::voting_get_all_votes(proposal_id)
}

// cycles

#[query]
pub fn get_current_cycle() -> Cycle {
    app_services::discounts::get_current_cycle()
}

// staking

#[query(composite = true)]
pub async fn get_staking_score(principal: Account) -> Nat {
    app_services::discounts::get_staking_score(principal).await
}

// discounts

#[query(composite = true)]
pub async fn calculate_discount(principal: Account, price: u128) -> f32 {
    app_services::discounts::calculate_discount(principal, price).await
}

#[update]
pub async fn mint_discount(discount: Discount) -> u128 {
    app_services::discounts::mint_discount(discount).await
}

#[query(composite = true)]
pub async fn get_discount(dicount_id: u128) -> Discount {
    app_services::discounts::get_discount(dicount_id).await
}

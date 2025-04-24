use crate::types::Tokens;
use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Deserialize, Serialize, CandidType)]
pub struct StakingLogEntry {
    pub timestamp: u64,
    pub previous_amount: Tokens,
    pub current_amount: Tokens,
}

#[derive(CandidType)]
pub struct StakingLogResponse {
    pub log: Vec<StakingLogEntry>,
    pub from: u64,
    pub to: u64
}

#[derive(CandidType)]
pub struct StakingRewardsResponse {
    pub amount: Tokens,
    pub to: u64,
    pub from: u64
}
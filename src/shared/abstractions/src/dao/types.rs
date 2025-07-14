use crate::Timestamp;
use candid::{CandidType, Deserialize, Principal};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum ProposalType {
    UpdateCode,
    Generic,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum VoteOption {
    Approve,
    Decline,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Proposal {
    pub id: u64,
    pub created_on: Timestamp,
    pub created_by: Principal,
    pub proposal_type: ProposalType,
    pub votes: Vec<u64>,
    pub data: String,
    pub start: Timestamp,
    pub end: Timestamp,
    // pub state: ProposalState
}

impl Proposal {
    pub fn new(
        created_on: u64,
        author: Principal,
        proposal_type: ProposalType,
        data: String,
        start: u64,
        end: u64,
    ) -> Self {
        Self {
            id: 0,
            created_on,
            created_by: author,
            proposal_type,
            votes: vec![],
            data,
            start,
            end,
            // state: ProposalState::Pending
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub enum ProposalState {
    Pending,
    Active,
    Approved,
    Declined,
}

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Vote {
    pub id: u64,
    pub created_on: Timestamp,
    pub created_by: Principal,
    pub proposal_id: u64,
    pub result: VoteOption,
}

impl Vote {
    pub fn new(
        proposal_id: u64,
        voter: Principal,
        created_on: Timestamp,
        result: VoteOption,
    ) -> Self {
        Self {
            id: 0,
            created_on,
            created_by: voter,
            proposal_id,
            result,
        }
    }
}

#[derive(Debug, CandidType, Deserialize, Serialize)]
pub struct CodeProposalData {
    pub repo_url: String,
    pub commit: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Discount {
    pub id: u128,
    pub value: f32,
    pub owner: Account,
}

impl Discount {
    pub fn new(value: f32, owner: Account) -> Self {
        Self {
            id: 0,
            value,
            owner,
        }
    }
}

impl Discount {
    pub fn to_metadata(&self) -> Vec<(String, MetadataValue)> {
        Vec::from([            
            ("value".to_string(), MetadataValue::Text(self.value.to_string())),
        ])
    }
}

#[derive(CandidType, Deserialize)]
pub struct Cycle {
    pub number: u64,
    pub start: Timestamp,
    pub end: Timestamp,
}
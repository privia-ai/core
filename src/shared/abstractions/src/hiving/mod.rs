use candid::{CandidType, Deserialize, Nat, Principal};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use serde::Serialize;

use crate::Tokens;

pub type HiverId = u64;
pub type PoolId = u64;
pub type ContractId = u64;
pub type TimeUnits = u64;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct HiverRegistration {
    pub principal: Principal,
    pub metadata: Vec<(String, MetadataValue)>,
    pub price_per_time_unit: Nat,
    pub max_time_units: TimeUnits,
    pub cycle_cap: TimeUnits,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PoolParticipant {
    pub principal: Principal,
    pub declared_time: TimeUnits,
    pub share: Nat,
    pub pending_rewards: Tokens,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PriceQuote {
    pub time_units: TimeUnits,
    pub ckusdc_cost: Nat,
    pub discount_value: f32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum ContractStatus {
    Open,
    Paid,
    Fulfilled,
    Redeemed,
    Cancelled,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DiscountContract {
    pub id: ContractId,
    pub buyer: Account,
    pub seller: Principal,
    pub time_units: TimeUnits,
    pub price: Nat,
    pub discount_value: f32,
    pub status: ContractStatus,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RegisterHivingCanisterArgs {
    pub canister_id: Principal,
    pub metadata: Vec<(String, MetadataValue)>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct PoolJoinProof {
    pub principal: Principal,
    pub signature: Option<Vec<u8>>,
    pub note: Option<String>,
}

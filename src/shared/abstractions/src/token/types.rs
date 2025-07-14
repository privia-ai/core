use candid::{CandidType, Deserialize, Nat};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;

#[derive(CandidType, Deserialize)]
pub struct InitArgs {
    pub decimals: Option<u8>,
    pub token_symbol: String,
    pub transfer_fee: Nat,
    pub metadata: Vec<(String, MetadataValue)>,
    pub minting_account: Account,
    pub fee_collector_account: Option<Account>,
    pub max_memo_length: Option<u16>,
    pub token_name: String,
}

#[derive(CandidType, Deserialize)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct StakingLogEntry {
    pub previous_amount: Nat,
    pub current_amount: Nat,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Debug)]
pub struct StakingLogResult {
    pub to: u64,
    pub log: Vec<StakingLogEntry>,
    pub from: u64,
}
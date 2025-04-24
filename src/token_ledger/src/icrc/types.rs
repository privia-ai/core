use crate::types::Tokens;
use candid::{CandidType, Deserialize, Nat};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::{
        account::Account,
        transfer::Memo
    }
};

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct SupportedStandard {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
pub struct InitArgs {
    pub minting_account: Account,
    pub fee_collector_account: Option<Account>,
    pub transfer_fee: Nat,
    pub decimals: Option<u8>,
    pub token_name: String,
    pub token_symbol: String,
    pub metadata: Vec<(String, MetadataValue)>,
    pub max_memo_length: Option<u16>
}

#[derive(Debug)]
pub struct TxInfo {
    pub from: Account,
    pub to: Option<Account>,
    pub amount: Tokens,
    pub spender: Option<Account>,
    pub memo: Option<Memo>,
    pub fee: Option<Tokens>,
    pub created_at_time: Option<u64>,
    pub expected_allowance: Option<Tokens>,
    pub expires_at: Option<u64>,
    pub is_approval: bool,
}

#[derive(Debug, Default, CandidType, Deserialize)]
pub struct Configuration {
    pub token_name: String,
    pub token_symbol: String,
    pub transfer_fee: Tokens,
    pub decimals: u8,
    pub minting_account: Option<Account>,
}
use candid::{CandidType, Deserialize};
use icrc_ledger_types::icrc1::account::Account;

#[derive(Clone, CandidType, Deserialize)]
pub struct CollectionMetadata {
    pub symbol: String,
    pub name: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub supply_cap: Option<u128>,
    pub max_query_batch_size: Option<u128>,
    pub max_update_batch_size: Option<u128>,
    pub default_take_value: Option<u128>,
    pub max_take_value: Option<u128>,
    pub max_memo_size: Option<u128>,
    pub atomic_batch_transfers: Option<bool>,
    pub tx_window: Option<u128>,
    pub permitted_drift: Option<u128>,
}

impl Default for CollectionMetadata {
    fn default() -> Self {
        Self {
            symbol: "DSC".to_string(),
            name: "DSC_NFT".to_string(),
            description: Some("The collection of discount NFTs".to_string()),
            logo: None,
            supply_cap: None,
            max_query_batch_size: None,
            max_update_batch_size: None,
            default_take_value: None,
            max_take_value: None,
            max_memo_size: None,
            atomic_batch_transfers: None,
            tx_window: None,
            permitted_drift: None,
        }
    }
}

pub type TokenId = u128;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {
    pub id: TokenId,
    pub owner: Account,
    pub data: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferArg {
    pub to: Account,
    pub token_id: TokenId,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum TransferError {
    NonExistingTokenId,
    InvalidRecipient,
    Unauthorized,
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    Duplicate { duplicate_of: u64 },
    GenericError { error_code: u64, message: String },
    GenericBatchError { error_code: u64, message: String },
}

pub type TransferResult = Result<u64, TransferError>;

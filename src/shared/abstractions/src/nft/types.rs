use candid::{CandidType, Deserialize, Nat};
use icrc_ledger_types::icrc1::account::Account;

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub to: Account,
    pub token_id: Nat,
    pub memo: Option<serde_bytes::ByteBuf>,
    pub from_subaccount: Option<serde_bytes::ByteBuf>,
    pub created_at_time: Option<u64>,
}
#[derive(CandidType, Deserialize)]
pub enum TransferError {
    GenericError { message: String, error_code: Nat },
    Duplicate { duplicate_of: Nat },
    NonExistingTokenId,
    Unauthorized,
    CreatedInFuture { ledger_time: u64 },
    InvalidRecipient,
    GenericBatchError { message: String, error_code: Nat },
    TooOld,
}
pub type TransferResult = Result<Nat, TransferError>;

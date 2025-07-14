pub mod nft;
pub mod dao;
pub mod token;
pub mod runtime;

#[cfg(feature = "with-chrono")]
pub mod display_impls;

use candid::Nat;
pub use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
pub use icrc_ledger_types::icrc1::account::Account;

pub type Timestamp = u64;
pub type Tokens = Nat;
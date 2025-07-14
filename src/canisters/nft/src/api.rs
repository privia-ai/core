use ic_cdk::{init, post_upgrade, query, update};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use crate::app;
use crate::domain::types::{TransferArg, TransferResult};

#[init]
fn init() {
    app::init_service()
}

#[post_upgrade]
fn memories() {
    app::init_service()
}

#[query]
fn icrc7_total_supply() -> u128 {
    app::icrc7_total_supply()
}

#[query]
fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<u128> {
    app::icrc7_balance_of(accounts)
}

#[query]
fn icrc7_tokens(prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    app::icrc7_tokens(prev, take)
}

#[query]
fn icrc7_tokens_of(account: Account, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    app::icrc7_tokens_of(account, prev, take)
}

#[query]
fn icrc7_owner_of(token_ids: Vec<u128>) -> Vec<Option<Account>> {
    app::icrc7_owner_of(token_ids)
}

#[update]
fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    app::icrc7_transfer(args)
}

#[query]
fn icrc7_collection_metadata() -> Vec<(String, MetadataValue)> {
    app::icrc7_collection_metadata()
}

#[query]
fn icrc7_symbol() -> String {
    app::icrc7_symbol()
}

#[query]
fn icrc7_name() -> String {
    app::icrc7_name()
}

#[query]
fn icrc7_description() -> Option<String> {
    app::icrc7_description()
}

#[query]
fn icrc7_logo() -> Option<String> {
    app::icrc7_logo()
}

#[query]
fn icrc7_supply_cap() -> Option<u128> {
    app::icrc7_supply_cap()
}

#[query]
fn icrc7_max_query_batch_size() -> Option<u128> {
    app::icrc7_max_query_batch_size()
}

#[query]
fn icrc7_max_update_batch_size() -> Option<u128> {
    app::icrc7_max_update_batch_size()
}

#[query]
fn icrc7_default_take_value() -> Option<u128> {
    app::icrc7_default_take_value()
}

#[query]
fn icrc7_max_take_value() -> Option<u128> {
    app::icrc7_max_take_value()
}

#[query]
fn icrc7_max_memo_size() -> Option<u128> {
    app::icrc7_max_memo_size()
}

#[query]
fn icrc7_atomic_batch_transfers() -> Option<bool> {
    app::icrc7_atomic_batch_transfers()
}

#[query]
fn icrc7_tx_window() -> Option<u128> {
    app::icrc7_tx_window()
}

#[query]
fn icrc7_permitted_drift() -> Option<u128> {
    app::icrc7_permitted_drift()
}

#[query]
fn icrc7_token_metadata(token_ids: Vec<u128>) -> Vec<Option<Vec<(String, MetadataValue)>>> {
    app::icrc7_token_metadata(token_ids)
}

#[query]
fn icrc7_supported_standards() -> Vec<(String, String)> {
    app::icrc7_supported_standards()
}

#[update]
fn privia_mint_token(owner: Account, metadata: Vec<(String, MetadataValue)>) -> u128 {
    app::privia_mint_token(owner, metadata)
}

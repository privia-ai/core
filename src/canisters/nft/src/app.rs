use std::cell::{OnceCell};
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use crate::domain::{NftService, types::*};

thread_local! {
    static SERVICE: OnceCell<NftService> = OnceCell::new();
}

pub fn init_service() {
    SERVICE.with(|cell| {
        cell.set(build_nft_service()).ok().expect("NftService already initialized");
    });
}

fn with_service<R, F>(f: F) -> R
where
    F: FnOnce(&NftService) -> R,
{
    SERVICE.with(|cell| {
        let svc = cell.get().expect("NftService not initialized");
        f(svc)
    })
}

fn build_nft_service() -> NftService {
    use crate::icp::icp_service_builder as builder;

    let tokens = builder::build_token_store();
    let index = builder::build_index_store();
    let metadata = builder::build_metadata_store();
    let runtime = builder::build_runtime();

    NftService::new(tokens, index, metadata, runtime)
}

pub fn icrc7_total_supply() -> u128 {
    with_service(|s| s.icrc7_total_supply())
}

pub fn icrc7_balance_of(accounts: Vec<Account>) -> Vec<u128> {
    with_service(|s| s.icrc7_balance_of(accounts))
}

pub fn icrc7_tokens(prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    with_service(|s| s.icrc7_tokens(prev, take))
}

pub fn icrc7_tokens_of(account: Account, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
    with_service(|s| s.icrc7_tokens_of(account, prev, take))
}

pub fn icrc7_owner_of(token_ids: Vec<u128>) -> Vec<Option<Account>> {
    with_service(|s| s.icrc7_owner_of(token_ids))
}

pub fn icrc7_transfer(args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {
    with_service(|s| s.icrc7_transfer(args))
}

pub fn icrc7_collection_metadata() -> Vec<(String, MetadataValue)> {
    with_service(|s| s.icrc7_collection_metadata())
}

pub fn icrc7_symbol() -> String {
    with_service(|s| s.icrc7_symbol())
}

pub fn icrc7_name() -> String {
    with_service(|s| s.icrc7_name())
}

pub fn icrc7_description() -> Option<String> {
    with_service(|s| s.icrc7_description())
}

pub fn icrc7_logo() -> Option<String> {
    with_service(|s| s.icrc7_logo())
}

pub fn icrc7_supply_cap() -> Option<u128> {
    with_service(|s| s.icrc7_supply_cap())
}

pub fn icrc7_max_query_batch_size() -> Option<u128> {
    with_service(|s| s.icrc7_max_query_batch_size())
}

pub fn icrc7_max_update_batch_size() -> Option<u128> {
    with_service(|s| s.icrc7_max_update_batch_size())
}

pub fn icrc7_default_take_value() -> Option<u128> {
    with_service(|s| s.icrc7_default_take_value())
}

pub fn icrc7_max_take_value() -> Option<u128> {
    with_service(|s| s.icrc7_max_take_value())
}

pub fn icrc7_max_memo_size() -> Option<u128> {
    with_service(|s| s.icrc7_max_memo_size())
}

pub fn icrc7_atomic_batch_transfers() -> Option<bool> {
    with_service(|s| s.icrc7_atomic_batch_transfers())
}

pub fn icrc7_tx_window() -> Option<u128> {
    with_service(|s| s.icrc7_tx_window())
}

pub fn icrc7_permitted_drift() -> Option<u128> {
    with_service(|s| s.icrc7_permitted_drift())
}

pub fn icrc7_token_metadata(token_ids: Vec<u128>) -> Vec<Option<Vec<(String, MetadataValue)>>> {
    with_service(|s| s.icrc7_token_metadata(token_ids))
}

pub fn icrc7_supported_standards() -> Vec<(String, String)> {
    vec![("ICRC-7".to_string(), "1.0.0".to_string())]
}

pub fn privia_mint_token(owner: Account, metadata: Vec<(String, MetadataValue)>) -> u128 {
    with_service(|s| s.privia_mint_token(owner, metadata))
}
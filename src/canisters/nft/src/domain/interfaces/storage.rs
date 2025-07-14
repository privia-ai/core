use icrc_ledger_types::icrc1::account::Account;
use crate::domain::{CollectionMetadata, Token, TokenId};

pub trait IMetadataStore {
    fn get_collection_metadata(&self) -> CollectionMetadata;
}

pub trait ITokenStore {
    fn get(&self, id: &TokenId) -> Option<Token>;
    fn update_owner(&mut self, id: &TokenId, new_owner: Account);
    fn insert(&mut self, token: Token) -> u128;
    fn list(&self) -> Vec<Token>;
    fn list_ids(&self) -> Vec<TokenId>;
    fn len(&self) -> u128;
    fn count_owned_by(&self, owner: &Account) -> u128;
}

pub trait IIndexStore {
    fn next_index(&mut self) -> u64;
    fn is_duplicate(&self, created_at: u64) -> Option<u64>;
    fn record(&mut self, created_at: u64, index: u64);
}
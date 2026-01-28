use crate::domain::{interfaces::storage::ITokenStore, types::*};
use crate::icp::stable_storage::{get_tokens_memory, IcpMemory};
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{StableBTreeMap, Storable};
use icrc_ledger_types::icrc1::account::Account;
use std::borrow::Cow;
use std::ops::{Deref, DerefMut};

pub struct TokenStoreStable {
    tokens: StableBTreeMap<u128, StorableToken, IcpMemory>,
}

impl TokenStoreStable {
    pub fn init() -> Self {
        Self {
            tokens: StableBTreeMap::init(get_tokens_memory()),
        }
    }
}

impl ITokenStore for TokenStoreStable {
    fn get(&self, id: &TokenId) -> Option<Token> {
        self.tokens.get(id).map(|token| token.0)
    }

    fn update_owner(&mut self, id: &TokenId, new_owner: Account) {
        let token = self.tokens.get(id).map(|token| token.clone());
        if token.is_none() {
            panic!("Token with id '{}' not found", id)
        }

        let mut token = token.unwrap();
        token.owner = new_owner;
        self.tokens.insert(*id, StorableToken(token));
    }

    fn insert(&mut self, mut token: Token) -> u128 {
        let id = self.tokens.len() as u128;
        token.id = id;
        self.tokens.insert(id, StorableToken(token));
        id
    }

    fn list(&self) -> Vec<Token> {
        self.tokens.values().map(|token| token.clone()).collect()
    }

    fn list_ids(&self) -> Vec<TokenId> {
        self.tokens.keys().collect()
    }

    fn len(&self) -> u128 {
        self.tokens.len() as u128
    }

    fn count_owned_by(&self, owner: &Account) -> u128 {
        self.tokens
            .iter()
            .filter(|(_, token)| token.owner == *owner)
            .count() as u128
    }
}

pub struct StorableToken(pub Token);

impl Storable for StorableToken {
    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: Token = candid::decode_one(&bytes).unwrap();
        StorableToken(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl Deref for StorableToken {
    type Target = Token;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StorableToken {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

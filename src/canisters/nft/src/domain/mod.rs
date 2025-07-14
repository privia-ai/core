pub mod interfaces;
pub mod types;

use candid::{Nat};
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use interfaces::storage::*;
use std::cell::RefCell;
use std::rc::Rc;
use abstractions::runtime::ICanisterRuntime;
use crate::domain::types::*;

pub struct NftService {
    tokens: Rc<RefCell<dyn ITokenStore>>,
    index: Rc<RefCell<dyn IIndexStore>>,
    metadata: Rc<RefCell<dyn IMetadataStore>>,
    runtime: Rc<RefCell<dyn ICanisterRuntime>>,
}

impl NftService {
    pub fn new(
        tokens: Rc<RefCell<dyn ITokenStore>>,
        index: Rc<RefCell<dyn IIndexStore>>,
        metadata: Rc<RefCell<dyn IMetadataStore>>,
        runtime: Rc<RefCell<dyn ICanisterRuntime>>,
    ) -> Self {
        Self {
            tokens,
            index,
            metadata,
            runtime,
        }
    }

    pub fn icrc7_transfer(&self, args: Vec<TransferArg>) -> Vec<Option<TransferResult>> {

        let mut results = vec![];

        for arg in args {
            let token = self.tokens.borrow().get(&arg.token_id);
            if token.is_none() {
                results.push(Some(Err(TransferError::NonExistingTokenId)));
                continue;
            }
            let token = token.unwrap();
            let caller = Account::from(self.runtime.borrow().get_caller());
            if token.owner != caller {
                results.push(Some(Err(TransferError::Unauthorized)));
                continue;
            }

            if arg.to == caller {
                results.push(Some(Err(TransferError::InvalidRecipient)));
                continue;
            }

            let now = self.runtime.borrow().get_time();
            if let Some(created_at) = arg.created_at_time {
                if created_at > now + 5_000_000_000 {
                    results.push(Some(Err(TransferError::CreatedInFuture {
                        ledger_time: now,
                    })));
                    continue;
                }
                if created_at + 300_000_000_000 < now {
                    results.push(Some(Err(TransferError::TooOld)));
                    continue;
                }
                if let Some(index) = self.index.borrow().is_duplicate(created_at) {
                    results.push(Some(Err(TransferError::Duplicate {
                        duplicate_of: index,
                    })));
                    continue;
                }
            }
            
            self.tokens
                .borrow_mut()
                .update_owner(&arg.token_id, arg.to.clone());
            let index = self.index.borrow_mut().next_index();

            if let Some(created_at) = arg.created_at_time {
                self.index.borrow_mut().record(created_at, index);
            }

            results.push(Some(Ok(index)));
        }

        results
    }

    pub fn icrc7_total_supply(&self) -> u128 {
        let len = self.tokens.borrow().len();
        len
    }

    pub fn icrc7_balance_of(&self, accounts: Vec<Account>) -> Vec<u128> {
        let tokens = self.tokens.borrow();
        accounts
            .into_iter()
            .map(|account| tokens.count_owned_by(&account))
            .collect()
    }

    pub fn icrc7_tokens(&self, prev: Option<u128>, take: Option<u128>) -> Vec<u128> {
        let mut token_ids: Vec<_> = self.tokens.borrow().list_ids();
        token_ids.sort_unstable();
        let start_index = match prev {
            Some(prev_id) => token_ids
                .iter()
                .position(|&id| id > prev_id)
                .unwrap_or(token_ids.len()),
            None => 0,
        };
        let end_index = match take {
            Some(take_n) => (start_index + take_n as usize).min(token_ids.len()),
            None => token_ids.len(),
        };
        token_ids[start_index..end_index].to_vec()
    }

    pub fn icrc7_tokens_of(
        &self,
        account: Account,
        prev: Option<u128>,
        take: Option<u128>,
    ) -> Vec<u128> {
        let mut token_ids: Vec<_> = self
            .tokens
            .borrow()
            .list()
            .iter()
            .filter(|t| t.owner == account)
            .map(|t| t.id)
            .collect();

        token_ids.sort_unstable();

        let start_index = match prev {
            Some(prev_id) => token_ids
                .iter()
                .position(|&id| id > prev_id)
                .unwrap_or(token_ids.len()),
            None => 0,
        };

        let end_index = match take {
            Some(take_n) => (start_index + take_n as usize).min(token_ids.len()),
            None => token_ids.len(),
        };

        token_ids[start_index..end_index].to_vec()
    }

    pub fn icrc7_owner_of(&self, token_ids: Vec<u128>) -> Vec<Option<Account>> {
        let tokens = self.tokens.borrow();
        token_ids
            .into_iter()
            .map(|id| tokens.get(&id).map(|t| {
                let owner = t.owner.clone();
                owner
            }))
            .collect()
    }

    pub fn icrc7_symbol(&self) -> String {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .symbol
            .clone()
    }

    pub fn icrc7_name(&self) -> String {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .name
            .clone()
    }

    pub fn icrc7_description(&self) -> Option<String> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .description
            .clone()
    }

    pub fn icrc7_logo(&self) -> Option<String> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .logo
            .clone()
    }

    pub fn icrc7_supply_cap(&self) -> Option<u128> {
        self.metadata.borrow().get_collection_metadata().supply_cap
    }

    pub fn icrc7_max_query_batch_size(&self) -> Option<u128> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .max_query_batch_size
    }

    pub fn icrc7_max_update_batch_size(&self) -> Option<u128> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .max_update_batch_size
    }

    pub fn icrc7_default_take_value(&self) -> Option<u128> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .default_take_value
    }

    pub fn icrc7_max_take_value(&self) -> Option<u128> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .max_take_value
    }

    pub fn icrc7_max_memo_size(&self) -> Option<u128> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .max_memo_size
    }

    pub fn icrc7_atomic_batch_transfers(&self) -> Option<bool> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .atomic_batch_transfers
    }

    pub fn icrc7_tx_window(&self) -> Option<u128> {
        self.metadata.borrow().get_collection_metadata().tx_window
    }

    pub fn icrc7_permitted_drift(&self) -> Option<u128> {
        self.metadata
            .borrow()
            .get_collection_metadata()
            .permitted_drift
    }

    pub fn icrc7_collection_metadata(&self) -> Vec<(String, MetadataValue)> {
        let m = self.metadata.borrow().get_collection_metadata();
        let mut items = vec![
            ("icrc7:symbol".to_string(), MetadataValue::Text(m.symbol.clone())),
            ("icrc7:name".to_string(), MetadataValue::Text(m.name.clone())),
        ];
        if let Some(desc) = &m.description {
            items.push(("icrc7:description".to_string(), MetadataValue::Text(desc.clone())));
        }
        if let Some(logo) = &m.logo {
            items.push(("icrc7:logo".to_string(), MetadataValue::Text(logo.clone())));
        }
        if let Some(cap) = m.supply_cap {
            items.push(("icrc7:supply_cap".to_string(), MetadataValue::Nat(Nat::from(cap))));
        }
        if let Some(qbs) = m.max_query_batch_size {
            items.push(("icrc7:max_query_batch_size".to_string(), MetadataValue::Nat(Nat::from(qbs))));
        }
        if let Some(ubs) = m.max_update_batch_size {
            items.push(("icrc7:max_update_batch_size".to_string(), MetadataValue::Nat(Nat::from(ubs))));
        }
        if let Some(dtv) = m.default_take_value {
            items.push(("icrc7:default_take_value".to_string(), MetadataValue::Nat(Nat::from(dtv))));
        }
        if let Some(mtv) = m.max_take_value {
            items.push(("icrc7:max_take_value".to_string(), MetadataValue::Nat(Nat::from(mtv))));
        }
        if let Some(mms) = m.max_memo_size {
            items.push(("icrc7:max_memo_size".to_string(), MetadataValue::Nat(Nat::from(mms))));
        }
        if let Some(abt) = m.atomic_batch_transfers {
            items.push((
                "icrc7:atomic_batch_transfers".to_string(),
                MetadataValue::Text(abt.to_string()),
            ));
        }
        if let Some(txw) = m.tx_window {
            items.push(("icrc7:tx_window".to_string(), MetadataValue::Nat(Nat::from(txw))));
        }
        if let Some(pd) = m.permitted_drift {
            items.push(("icrc7:permitted_drift".to_string(), MetadataValue::Nat(Nat::from(pd))));
        }
        items
    }

    pub fn icrc7_token_metadata(&self, token_ids: Vec<u128>) -> Vec<Option<Vec<(String, MetadataValue)>>> {
        token_ids
            .into_iter()
            .map(|id| self.get_token_metadata(&id))
            .collect()
    }

    fn get_token_metadata(&self, token_id: &TokenId) -> Option<Vec<(String, MetadataValue)>> {
        let token = self.tokens.borrow().get(token_id);
        match token {
            None => None,
            Some(t) => {
                let res = serde_json::from_str(&t.data);
                if res.is_err() {
                    let kv = ("name".to_string(), MetadataValue::Text("value".to_string()));
                    Some(Vec::from([kv]))
                }
                else {
                    Some(res.unwrap())
                }

            }
        }
    }

    pub fn privia_mint_token(&self, owner: Account, metadata: Vec<(String, MetadataValue)>) -> u128 {
        let metadata_json = serde_json::to_string(&metadata).unwrap();

        let token = Token {
            id: 0,
            owner,
            data: metadata_json,
        };
        self.tokens.borrow_mut().insert(token)
    }
}
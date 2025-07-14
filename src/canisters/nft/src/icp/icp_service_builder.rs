use super::*;
use crate::domain::interfaces::storage::{IIndexStore, IMetadataStore, ITokenStore};
use crate::icp::stable_storage::{IndexStoreStable, MetadataStoreStable, TokenStoreStable};
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static TOKENS: Rc<RefCell<dyn ITokenStore>> = Rc::new(RefCell::new(TokenStoreStable::init()));
    static INDEXES: Rc<RefCell<dyn IIndexStore >> = Rc::new(RefCell::new(IndexStoreStable::init()));
    static METADATA: Rc<RefCell<dyn IMetadataStore>> = Rc::new(RefCell::new(MetadataStoreStable::init()));
    static RUNTIME: Rc<RefCell<dyn ICanisterRuntime>> = Rc::new(RefCell::new(RuntimeIcp {}));
}

pub fn build_runtime() -> Rc<RefCell<dyn ICanisterRuntime>> {
    RUNTIME.with(|rt| rt.clone())
}

pub fn build_token_store() -> Rc<RefCell<dyn ITokenStore>> {
    TOKENS.with(|t| t.clone())
}

pub fn build_index_store() -> Rc<RefCell<dyn IIndexStore>> {
    INDEXES.with(|t| t.clone())
}

pub fn build_metadata_store() -> Rc<RefCell<dyn IMetadataStore>> {
    METADATA.with(|m| m.clone())
}

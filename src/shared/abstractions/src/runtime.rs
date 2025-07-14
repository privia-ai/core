pub use ic_cdk::call::Error;

use async_trait::async_trait;
use candid::{CandidType, Principal};
use serde::Deserialize;
use crate::Timestamp;

pub trait ICanisterRuntime {
    fn get_caller(&self) -> Principal;
    fn get_time(&self) -> Timestamp;
}

#[async_trait]
pub trait ICallContext {
    type Error;

    async fn call<'a, Out>(
        &self,
        id: Principal,
        mode: CallMode,
        method: &str,
        args: &'a [u8],
    ) -> Result<Out, Self::Error>
    where
        Out: CandidType + for<'de> Deserialize<'de>;
}

pub enum CallMode {
    Query,
    Update,
}


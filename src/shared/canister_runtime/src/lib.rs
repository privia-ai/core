use abstractions::runtime::{CallMode, ICallContext, ICanisterRuntime};
use async_trait::async_trait;
use candid::{CandidType, Principal};
use ic_cdk::call::Call;
use serde::Deserialize;
use abstractions::Timestamp;

pub struct CdkCallContext;

#[async_trait]
impl ICallContext for CdkCallContext {
    type Error = ic_cdk::call::Error;

    async fn call<'a, Out>(
        &self,
        id: Principal,
        _mode: CallMode,
        method: &str,
        args: &'a [u8],
    ) -> Result<Out, Self::Error>
    where
        Out: CandidType + for<'de> Deserialize<'de>,
    {
        let call_result = Call::unbounded_wait(id, method)
            .with_raw_args(args)
            .await
            .map_err(Self::Error::from)?
            .candid::<Out>()
            .map_err(Self::Error::from)?;

        Ok(call_result)
    }
}

pub struct RuntimeIcp {}

impl RuntimeIcp {
    pub fn new() -> Self {
        Self {}
    }
}

impl ICanisterRuntime for RuntimeIcp {
    fn get_caller(&self) -> Principal {
        ic_cdk::api::msg_caller()
    }

    fn get_time(&self) -> Timestamp {
        ic_cdk::api::time()
    }
}

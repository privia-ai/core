use std::cell::RefCell;
use std::rc::Rc;

use candid::Principal;

use crate::runtime::ICallContext;
use crate::token::TokenClient;

/// Lightweight alias to make ckUSDC usage explicit while reusing the ICRC-1/2 client.
pub struct CkUsdcClient<R: ICallContext> {
    inner: TokenClient<R>,
}

impl<R: ICallContext> CkUsdcClient<R> {
    pub fn new(runtime: Rc<RefCell<R>>, canister_id: Principal) -> Self {
        Self {
            inner: TokenClient { runtime, canister_id },
        }
    }

    pub fn inner(&self) -> &TokenClient<R> {
        &self.inner
    }

    pub fn into_inner(self) -> TokenClient<R> {
        self.inner
    }
}

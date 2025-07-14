use std::cell::RefCell;
use std::rc::Rc;
use candid::{Encode, Nat};
use icrc_ledger_types::{
    icrc1::{
        account::Account,
        transfer::{BlockIndex, TransferArg, TransferError}
    },
    icrc2::{
        approve::{ApproveArgs, ApproveError},
        transfer_from::{TransferFromArgs, TransferFromError}
    },
};
use crate::runtime::{CallMode, ICallContext};
use crate::token::StakingLogResult;

pub struct TokenClient<R: ICallContext> {
    pub runtime: Rc<RefCell<R>>,
    pub canister_id: candid::Principal,
}

impl<R: ICallContext> TokenClient<R> {
    pub async fn privia_staking_log(
        &self,
        account: Account,
        log_start: Option<u64>,
        log_end: Option<u64>,
    ) -> Result<StakingLogResult, R::Error> {
        let method = "privia_staking_log";
        let args = Encode!(&account, &log_start, &log_end).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn balance_of(&self, account: Account) -> Result<Nat, R::Error> {
        let method = "icrc1_balance_of";
        let args = Encode!(&account).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn decimals(&self) -> Result<u8, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc1_decimals", &[])
            .await
    }

    pub async fn name(&self) -> Result<String, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc1_name", &[])
            .await
    }

    pub async fn metadata(&self) -> Result<Vec<(String, crate::MetadataValue)>, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc1_metadata", &[])
            .await
    }

    pub async fn symbol(&self) -> Result<String, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc1_symbol", &[])
            .await
    }

    pub async fn total_supply(&self) -> Result<Nat, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc1_total_supply", &[])
            .await
    }

    pub async fn fee(&self) -> Result<Nat, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc1_fee", &[])
            .await
    }

    pub async fn minting_account(&self) -> Result<Option<Account>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc1_minting_account",
                &[],
            )
            .await
    }

    pub async fn transfer(
        &self,
        args: TransferArg,
    ) -> Result<Result<BlockIndex, TransferError>, R::Error> {
        let method = "icrc1_transfer";
        let args = &Encode!(&args).unwrap();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }

    pub async fn transfer_from(
        &self,
        args: TransferFromArgs,
    ) -> Result<Result<BlockIndex, TransferFromError>, R::Error> {
        let method = "icrc2_transfer_from";
        let args = Encode!(&args).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }

    pub async fn approve(
        &self,
        args: ApproveArgs,
    ) -> Result<Result<BlockIndex, ApproveError>, R::Error> {
        let method = "icrc2_approve";
        let args = Encode!(&args).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }
}
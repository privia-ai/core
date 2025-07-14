use crate::nft::{TransferArg, TransferResult};
use crate::runtime::{CallMode, ICallContext};
use candid::Encode;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use icrc_ledger_types::icrc1::account::Account;
use std::cell::RefCell;
use std::rc::Rc;

pub struct NftClient<R: ICallContext> {
    pub runtime: Rc<RefCell<R>>,
    pub canister_id: candid::Principal,
}

impl<R: ICallContext> NftClient<R> {
    pub async fn icrc7_total_supply(&self) -> Result<u128, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_total_supply", &[])
            .await
    }

    pub async fn icrc7_balance_of(
        &self,
        accounts: Vec<Account>,
    ) -> Result<Vec<u128>, R::Error> {
        let method = "icrc7_balance_of";
        let args = Encode!(&accounts).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn icrc7_tokens(
        &self,
        prev: Option<u128>,
        take: Option<u128>,
    ) -> Result<Vec<u128>, R::Error> {
        let method = "icrc7_tokens";
        let args = Encode!(&prev, &take).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn icrc7_tokens_of(
        &self,
        account: Account,
        prev: Option<u128>,
        take: Option<u128>,
    ) -> Result<Vec<u128>, R::Error> {
        let method = "icrc7_tokens_of";
        let args = Encode!(&account, &prev, &take).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn icrc7_owner_of(
        &self,
        token_ids: Vec<u128>,
    ) -> Result<Vec<Option<Account>>, R::Error> {
        let method = "icrc7_owner_of";
        let args = Encode!(&token_ids).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn icrc7_collection_metadata(
        &self,
    ) -> Result<Vec<(String, MetadataValue)>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_collection_metadata",
                &[],
            )
            .await
    }

    pub async fn icrc7_symbol(&self) -> Result<String, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_symbol", &[])
            .await
    }

    pub async fn icrc7_name(&self) -> Result<String, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_name", &[])
            .await
    }

    pub async fn icrc7_description(&self) -> Result<Option<String>, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_description", &[])
            .await
    }

    pub async fn icrc7_logo(&self) -> Result<Option<String>, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_logo", &[])
            .await
    }

    pub async fn icrc7_supply_cap(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_supply_cap", &[])
            .await
    }

    pub async fn icrc7_max_query_batch_size(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_max_query_batch_size",
                &[],
            )
            .await
    }

    pub async fn icrc7_max_update_batch_size(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_max_update_batch_size",
                &[],
            )
            .await
    }

    pub async fn icrc7_default_take_value(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_default_take_value",
                &[],
            )
            .await
    }

    pub async fn icrc7_max_take_value(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_max_take_value",
                &[],
            )
            .await
    }

    pub async fn icrc7_max_memo_size(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_max_memo_size",
                &[],
            )
            .await
    }

    pub async fn icrc7_atomic_batch_transfers(&self) -> Result<Option<bool>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_atomic_batch_transfers",
                &[],
            )
            .await
    }

    pub async fn icrc7_tx_window(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, "icrc7_tx_window", &[])
            .await
    }

    pub async fn icrc7_permitted_drift(&self) -> Result<Option<u128>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_permitted_drift",
                &[],
            )
            .await
    }

    pub async fn icrc7_token_metadata(
        &self,
        token_ids: Vec<u128>,
    ) -> Result<Vec<Option<Vec<(String, MetadataValue)>>>, R::Error> {
        let method = "icrc7_token_metadata";
        let args = Encode!(&token_ids).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Query, method, args)
            .await
    }

    pub async fn icrc7_supported_standards(&self) -> Result<Vec<(String, String)>, R::Error> {
        self.runtime
            .borrow()
            .call(
                self.canister_id,
                CallMode::Query,
                "icrc7_supported_standards",
                &[],
            )
            .await
    }

    pub async fn icrc7_transfer(
        &self,
        args: Vec<TransferArg>,
    ) -> Result<Vec<Option<TransferResult>>, R::Error> {
        let method = "icrc7_transfer";
        let args = Encode!(&args).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }

    pub async fn privia_mint_token(
        &self,
        owner: Account,
        metadata: Vec<(String, MetadataValue)>,
    ) -> Result<u128, R::Error> {
        let method = "privia_mint_token";
        let args = Encode!(&owner, &metadata).unwrap();
        let args = args.as_slice();

        self.runtime
            .borrow()
            .call(self.canister_id, CallMode::Update, method, args)
            .await
    }
}
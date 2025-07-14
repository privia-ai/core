mod service_builder;

pub mod mgmt {
    use candid::{CandidType, Deserialize, Nat};
    use icrc_ledger_types::icrc1::account::Account;
    use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
    use crate::app::service_builder::build_token_service;
    use crate::domain::token::TokenConfiguration;

    #[derive(Clone, Eq, PartialEq, Debug, CandidType, Deserialize)]
    pub struct InitArgs {
        pub minting_account: Account,
        pub fee_collector_account: Option<Account>,
        pub transfer_fee: Nat,
        pub decimals: Option<u8>,
        pub token_name: String,
        pub token_symbol: String,
        pub metadata: Vec<(String, MetadataValue)>,
        pub max_memo_length: Option<u16>,
    }

    pub fn init(args: InitArgs) {
        let service = build_token_service();
        let token_config = TokenConfiguration {
            token_name: args.token_name,
            token_symbol: args.token_symbol,
            transfer_fee: args.transfer_fee,
            decimals: args.decimals.unwrap(),
            minting_account: Some(args.minting_account),
            fee_collector_account: args.fee_collector_account,
            metadata: args.metadata,
            max_memo_length: args.max_memo_length,
        };
        service.borrow_mut().init(token_config);
    }
}

pub mod token {
    use super::service_builder::build_token_service;
    use abstractions::{token::SupportedStandard, Account, MetadataValue, Tokens};
    use icrc_ledger_types::{
        icrc1::transfer::{BlockIndex, TransferArg, TransferError},
        icrc2::allowance::{Allowance, AllowanceArgs},
        icrc2::approve::{ApproveArgs, ApproveError},
        icrc2::transfer_from::{TransferFromArgs, TransferFromError},
    };

    pub fn icrc1_transfer(arg: TransferArg) -> Result<BlockIndex, TransferError> {
        let service = build_token_service();
        let res = service.borrow().icrc1_transfer(arg);
        res
    }

    pub fn icrc1_balance_of(account: Account) -> Tokens {
        let service = build_token_service();
        service.borrow().icrc1_balance_of(account)
    }

    pub fn icrc1_total_supply() -> Tokens {
        let service = build_token_service();
        service.borrow().icrc1_total_supply()
    }

    pub fn icrc1_minting_account() -> Option<Account> {
        let service = build_token_service();
        service.borrow().icrc1_minting_account()
    }

    pub fn icrc1_name() -> String {
        let service = build_token_service();
        service.borrow().icrc1_name()
    }

    pub fn icrc1_symbol() -> String {
        let service = build_token_service();
        service.borrow().icrc1_symbol()
    }

    pub fn icrc1_decimals() -> u8 {
        let service = build_token_service();
        service.borrow().icrc1_decimals()
    }

    pub fn icrc1_fee() -> Tokens {
        let service = build_token_service();
        service.borrow().icrc1_fee()
    }

    pub fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
        let service = build_token_service();
        service.borrow().icrc1_metadata()
    }

    pub fn icrc1_supported_standards() -> Vec<SupportedStandard> {
        let service = build_token_service();
        service.borrow().icrc1_supported_standards()
    }

    pub fn icrc2_approve(arg: ApproveArgs) -> Result<BlockIndex, ApproveError> {
        let service = build_token_service();
        service.borrow().icrc2_approve(arg)
    }

    pub fn icrc2_transfer_from(arg: TransferFromArgs) -> Result<BlockIndex, TransferFromError> {
        let service = build_token_service();
        service.borrow().icrc2_transfer_from(arg)
    }

    pub fn icrc2_allowance(arg: AllowanceArgs) -> Allowance {
        let service = build_token_service();
        service.borrow().icrc2_allowance(arg)
    }
}

pub mod staking {
    use super::service_builder::build_staking_service;
    use abstractions::token::StakingLogResult;
    use abstractions::Account;

    pub fn get_staking_log(
        target: Account,
        from: Option<u64>,
        to: Option<u64>,
    ) -> StakingLogResult {
        let service = build_staking_service();
        service.borrow().get_staking_log(target, from, to)
    }
}

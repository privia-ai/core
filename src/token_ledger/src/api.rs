use crate::{
    icrc::types::{InitArgs, SupportedStandard},
    privia::types::StakingLogResponse,
    privia::types::StakingRewardsResponse,
    types::*,
};
use candid::{Nat, Principal};
use ic_cdk::{init, query, update};
use icrc_ledger_types::{
    icrc::generic_metadata_value::MetadataValue,
    icrc1::{
        account::Account,
        transfer::{BlockIndex, TransferArg, TransferError},
    },
    icrc2::{
        allowance::{Allowance, AllowanceArgs},
        approve::{ApproveArgs, ApproveError},
        transfer_from::{TransferFromArgs, TransferFromError},
    },
};

pub mod canister_mgmt {
    use super::*;
    use crate::icrc as icrc_logic;

    #[init]
    fn init(args: InitArgs) {
        icrc_logic::init(args)
    }
}

pub mod icrc {
    use super::*;
    use crate::icrc as icrc_logic;

    #[update]
    pub fn icrc1_transfer(arg: TransferArg) -> Result<BlockIndex, TransferError> {
        icrc_logic::icrc1_transfer(arg)
    }

    #[query]
    pub fn icrc1_balance_of(account: Account) -> Tokens {
        icrc_logic::icrc1_balance_of(account)
    }

    #[query]
    pub fn icrc1_total_supply() -> Tokens {
        icrc_logic::icrc1_total_supply()
    }

    #[query]
    pub fn icrc1_minting_account() -> Option<Account> {
        icrc_logic::icrc1_minting_account()
    }

    #[query]
    pub fn icrc1_name() -> String {
        icrc_logic::icrc1_name()
    }

    #[query]
    pub fn icrc1_token_symbol() -> String {
        icrc_logic::icrc1_token_symbol()
    }

    #[query]
    pub fn icrc1_decimals() -> u8 {
        icrc_logic::icrc1_decimals()
    }

    #[query]
    pub fn icrc1_fee() -> Tokens {
        icrc_logic::icrc1_fee()
    }

    #[query]
    pub fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
        icrc_logic::icrc1_metadata()
    }

    #[query]
    pub fn icrc1_supported_standards() -> Vec<SupportedStandard> {
        icrc_logic::icrc1_supported_standards()
    }

    #[update]
    pub fn icrc2_approve(arg: ApproveArgs) -> Result<BlockIndex, ApproveError> {
        icrc_logic::icrc2_approve(arg)
    }

    #[update]
    pub fn icrc2_transfer_from(arg: TransferFromArgs) -> Result<BlockIndex, TransferFromError> {
        icrc_logic::icrc2_transfer_from(arg)
    }

    #[query]
    pub fn icrc2_allowance(arg: AllowanceArgs) -> Allowance {
        icrc_logic::icrc2_allowance(arg)
    }
}

pub mod privia {
    use super::*;
    use crate::privia as privia_logic;

    #[query]
    pub fn privia_staking_log(target: Principal, from: Option<u64>, to: Option<u64>) -> StakingLogResponse {
        privia_logic::privia_staking_log(target, from, to)
    }

    #[query]
    pub fn privia_staking_rewards(target: Principal, from: Option<u64>, to: Option<u64>) -> StakingRewardsResponse {
        privia_logic::privia_staking_rewards(target, from, to)
    }

    #[update]
    pub fn privia_split_balance(target: Principal) -> Result<Nat, TransferError> {
        privia_logic::privia_split_balance(target)
    }
}

ic_cdk::export_candid!();

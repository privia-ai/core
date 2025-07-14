use abstractions::token::{StakingLogResult, SupportedStandard};
use abstractions::Tokens;
use abstractions::{Account, MetadataValue};
use ic_cdk::{init, query, update};
use icrc_ledger_types::{
    icrc1::transfer::{BlockIndex, TransferArg, TransferError},
    icrc2::{
        allowance::{Allowance, AllowanceArgs},
        approve::{ApproveArgs, ApproveError},
        transfer_from::{TransferFromArgs, TransferFromError},
    },
};
use crate::app::{self};
use crate::app::mgmt::InitArgs;

#[init]
fn init(args: InitArgs) {
    app::mgmt::init(args)
}

#[update]
fn icrc1_transfer(arg: TransferArg) -> Result<BlockIndex, TransferError> {
    app::token::icrc1_transfer(arg)
}

#[query]
fn icrc1_balance_of(account: Account) -> Tokens {
    app::token::icrc1_balance_of(account)
}

#[query]
fn icrc1_total_supply() -> Tokens {
    app::token::icrc1_total_supply()
}

#[query]
fn icrc1_minting_account() -> Option<Account> {
    app::token::icrc1_minting_account()
}

#[query]
fn icrc1_name() -> String {
    app::token::icrc1_name()
}

#[query]
fn icrc1_symbol() -> String {
    app::token::icrc1_symbol()
}

#[query]
fn icrc1_decimals() -> u8 {
    app::token::icrc1_decimals()
}

#[query]
fn icrc1_fee() -> Tokens {
    app::token::icrc1_fee()
}

#[query]
fn icrc1_metadata() -> Vec<(String, MetadataValue)> {
    app::token::icrc1_metadata()
}

#[query]
fn icrc1_supported_standards() -> Vec<SupportedStandard> {
    app::token::icrc1_supported_standards()
}

#[update]
fn icrc2_approve(arg: ApproveArgs) -> Result<BlockIndex, ApproveError> {
    app::token::icrc2_approve(arg)
}

#[update]
fn icrc2_transfer_from(arg: TransferFromArgs) -> Result<BlockIndex, TransferFromError> {
    app::token::icrc2_transfer_from(arg)
}

#[query]
fn icrc2_allowance(arg: AllowanceArgs) -> Allowance {
    app::token::icrc2_allowance(arg)
}

#[query]
fn privia_staking_log(target: Account, from: Option<u64>, to: Option<u64>) -> StakingLogResult {
    app::staking::get_staking_log(target, from, to)
}

ic_cdk::export_candid!();

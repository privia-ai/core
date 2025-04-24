pub mod rewards;
mod storage;
pub mod types;

use crate::icrc as icrc_logic;
use crate::privia::types::{StakingLogEntry, StakingLogResponse, StakingRewardsResponse};
use crate::types::Tokens;
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::{
    account::Account,
    transfer::{TransferArg, TransferError},
};
use icrc_ledger_types::icrc3::transactions::Transaction;
use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use std::ops::Deref;
pub use storage::STAKINGS;

pub trait IStakingRegistry {
    fn get_log_entries(&self, address: Principal, from: u64, to: u64) -> Vec<StakingLogEntry>;
    fn get_latest_log_entry(&self, address: Principal, timestamp: u64) -> Option<StakingLogEntry>;
    fn add_log_entry(&mut self, address: Principal, log: &StakingLogEntry);
}

pub fn transaction_callback(transaction: &Transaction) {
    let mut from: Option<Account> = None;
    let mut to: Option<Account> = None;
    let mut from_init_balance = Tokens::from(0u8);
    let mut to_init_balance = Tokens::from(0u8);
    let mut effective_fee = Tokens::from(0u8);
    let amount: Tokens;

    if transaction.kind == "approve".to_string() {
        return
    }
    
    match transaction.clone().kind.as_str(){
        "mint" => {
            let tx = transaction.clone().mint.unwrap();
            amount = tx.amount;
            to = Some(tx.to);            
            to_init_balance = icrc_logic::icrc1_balance_of(to.unwrap());
        },
        "burn" => {
            let tx = transaction.clone().burn.unwrap();
            amount = tx.amount;
            from = Some(tx.from);
            from_init_balance = icrc_logic::icrc1_balance_of(from.unwrap());            
        },
        "transfer" => {
            let tx = transaction.clone().transfer.unwrap();
            amount = tx.amount;
            from = Some(tx.from);
            from_init_balance = icrc_logic::icrc1_balance_of(from.unwrap());
            to = Some(tx.to);
            to_init_balance = icrc_logic::icrc1_balance_of(to.unwrap());
            effective_fee = tx.fee.unwrap();
        },
        _ => panic!("Unexpected transaction kind")
    };

    update_stakings(
        &from,
        from_init_balance,
        &to,
        to_init_balance,
        &amount,
        &effective_fee,
        transaction.timestamp,
    );
}

pub fn split_by_two(amount: Tokens) -> Nat {
    let two = Nat::from(2u8);
    div(&amount, &two)
}

pub fn privia_staking_log(target: Principal, from: Option<u64>, to: Option<u64>) -> StakingLogResponse {
    let from = from.unwrap_or(0);
    let to = to.unwrap_or(ic_cdk::api::time());

    let log = STAKINGS.with(|s| s.borrow().get_log_entries(target, from, to));

    let mut entries: Vec<StakingLogEntry> = Vec::new();
    for entry in log {
        let entry_candid = StakingLogEntry {
            current_amount: entry.current_amount.into(),
            previous_amount: entry.previous_amount.into(),
            timestamp: entry.timestamp,
        };
        entries.push(entry_candid);
    }

    StakingLogResponse {
        log: entries,
        from,
        to,
    }
}

pub fn privia_staking_rewards(target: Principal, from: Option<u64>, to: Option<u64>) -> StakingRewardsResponse {
    let from = from.unwrap_or(0);
    let to = to.unwrap_or(ic_cdk::api::time());
    
    STAKINGS.with(|s| {
        let amount = rewards::calculate(s.borrow().deref(), target, from, to);
        StakingRewardsResponse{
            amount: amount.into(),
            to,
            from,
        }
    })
}

pub fn privia_split_balance(target: Principal) -> Result<Nat, TransferError> {
    let from_account = Account::from(ic_cdk::api::msg_caller());
    let to_account = Account::from(target);
    let from_balance = icrc_logic::balance(from_account);
    let fee = icrc_logic::icrc1_fee();
    let amount = split_by_two(from_balance);

    let transfer_req = TransferArg {
        from_subaccount: None,
        to: to_account,
        fee: Some(fee),
        created_at_time: None,
        memo: None,
        amount,
    };

    icrc_logic::icrc1_transfer(transfer_req)
}

fn update_stakings(
    from: &Option<Account>,
    from_init_balance: Tokens,
    to: &Option<Account>,
    to_init_balance: Tokens,
    amount: &Tokens,
    fee: &Tokens,
    timestamp: u64,
) {
    if from.is_some() {
        STAKINGS.with_borrow_mut(|s| {
            s.add_log_entry(
                from.unwrap().owner,
                &StakingLogEntry {
                    timestamp: timestamp.clone(),
                    previous_amount: from_init_balance.clone(),
                    current_amount: sub(&from_init_balance, &add(&amount, &fee))
                },
            )
        });
    };

    if to.is_some() {
        STAKINGS.with_borrow_mut(|s| {
            s.add_log_entry(
                to.unwrap().owner,
                &StakingLogEntry {
                    timestamp: timestamp.clone(),
                    previous_amount: to_init_balance.clone(),
                    current_amount: add(&to_init_balance, &amount)
                },
            )
        });
    }
}



fn multiply(one: &Nat, two: &Nat) -> Nat {
    Nat(one.0.checked_mul(&two.0).unwrap())
}

fn add(one: &Nat, two: &Nat) -> Nat {
    one.0.checked_add(&two.0).unwrap().into()
}

fn sub(one: &Nat, two: &Nat) -> Nat {
    one.0.checked_sub(&two.0).unwrap().into()
}

fn div(one: &Nat, two: &Nat) -> Nat {
    one.0.checked_div(&two.0).unwrap().into()    
}
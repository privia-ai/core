use abstractions::token::{StakingLogEntry, StakingLogResult};
use abstractions::{Account, Timestamp, Tokens};
use std::cell::RefCell;
use std::rc::Rc;

use crate::domain::interfaces::IStakingStore;
use crate::domain::staking::helpers::{add, sub};
use crate::domain::token::TokenService;

use icrc_ledger_types::icrc3::transactions::Transaction;

pub struct StakingService {
    staking_store: Rc<RefCell<dyn IStakingStore>>,
}

pub struct StakingContext {
    from: Option<Account>,
    from_init_balance: Tokens,
    to: Option<Account>,
    to_init_balance: Tokens,
    amount: Tokens,
    fee: Tokens,
    timestamp: Timestamp,
}

impl StakingService {
    pub fn new(staking_store: Rc<RefCell<dyn IStakingStore>>) -> Self {
        Self { staking_store }
    }

    pub fn build_staking_context<'a>(
        token_service: &'a TokenService,
        transaction: &'a Transaction,
    ) -> Option<StakingContext> {
        let mut from: Option<Account> = None;
        let mut to: Option<Account> = None;
        let mut from_init_balance = Tokens::from(0u8);
        let mut to_init_balance = Tokens::from(0u8);
        let mut effective_fee = Tokens::from(0u8);
        let amount: Tokens;

        if transaction.kind == "approve".to_string() {
            return None;
        }

        match transaction.clone().kind.as_str() {
            "mint" => {
                let tx = transaction.clone().mint.unwrap();
                amount = tx.amount;
                to = Some(tx.to);
                to_init_balance = token_service.icrc1_balance_of(to.unwrap());
            }
            "burn" => {
                let tx = transaction.clone().burn.unwrap();
                amount = tx.amount;
                from = Some(tx.from);
                from_init_balance = token_service.icrc1_balance_of(from.unwrap());
            }
            "transfer" => {
                let tx = transaction.clone().transfer.unwrap();
                amount = tx.amount;
                from = Some(tx.from);
                from_init_balance = token_service.icrc1_balance_of(from.unwrap());
                to = Some(tx.to);
                to_init_balance = token_service.icrc1_balance_of(to.unwrap());
                effective_fee = tx.fee.unwrap();
            }
            _ => panic!("Unexpected transaction kind"),
        };

        Some(StakingContext {
            from,
            from_init_balance,
            to,
            to_init_balance,
            amount,
            fee: effective_fee,
            timestamp: transaction.timestamp,
        })
    }

    pub fn transaction_callback(&self, ctx: StakingContext) {
        self.update_staking(ctx);
    }

    pub fn get_staking_log(
        &self,
        target: Account,
        from: Option<u64>,
        to: Option<u64>,
    ) -> StakingLogResult {
        let from = from.unwrap_or(0);
        let to = to.unwrap_or(ic_cdk::api::time());

        let log = self
            .staking_store
            .borrow()
            .get_log_entries(target, from, to);

        let mut entries: Vec<StakingLogEntry> = Vec::new();
        for entry in log {
            let entry_candid = StakingLogEntry {
                current_amount: entry.current_amount.into(),
                previous_amount: entry.previous_amount.into(),
                timestamp: entry.timestamp,
            };
            entries.push(entry_candid);
        }

        StakingLogResult {
            log: entries,
            from,
            to,
        }
    }

    fn update_staking(&self, ctx: StakingContext) {
        if ctx.from.is_some() {
            self.staking_store.borrow_mut().add_log_entry(
                ctx.from.unwrap(),
                &StakingLogEntry {
                    timestamp: ctx.timestamp.clone(),
                    previous_amount: ctx.from_init_balance.clone(),
                    current_amount: sub(&ctx.from_init_balance, &add(&ctx.amount, &ctx.fee)),
                },
            )
        };

        if ctx.to.is_some() {
            self.staking_store.borrow_mut().add_log_entry(
                ctx.to.unwrap(),
                &StakingLogEntry {
                    timestamp: ctx.timestamp.clone(),
                    previous_amount: ctx.to_init_balance.clone(),
                    current_amount: add(&ctx.to_init_balance, &ctx.amount),
                },
            )
        }
    }
}

mod helpers {
    use candid::Nat;
    use num_traits::{CheckedAdd, CheckedSub};

    pub fn add(one: &Nat, two: &Nat) -> Nat {
        one.0.checked_add(&two.0).unwrap().into()
    }

    pub fn sub(one: &Nat, two: &Nat) -> Nat {
        one.0.checked_sub(&two.0).unwrap().into()
    }
}

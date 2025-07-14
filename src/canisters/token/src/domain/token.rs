use crate::domain::interfaces::{IBalanceStore, IConfigurationStore, ITransactionStore};
use crate::domain::staking::StakingService;
use abstractions::runtime::ICanisterRuntime;
use abstractions::token::SupportedStandard;
use abstractions::{Account, MetadataValue, Tokens};
use candid::{CandidType, Deserialize, Int, Nat};
use icrc_ledger_types::{
    icrc1::transfer::{BlockIndex, Memo, TransferArg, TransferError},
    icrc2::{
        allowance::{Allowance, AllowanceArgs},
        approve::{ApproveArgs, ApproveError},
        transfer_from::{TransferFromArgs, TransferFromError},
    },
    icrc3::transactions::*,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::rc::Rc;

const MAX_MEMO_SIZE: usize = 32;
const PERMITTED_DRIFT_NANOS: u64 = 60_000_000_000;
const TRANSACTION_WINDOW_NANOS: u64 = 24 * 60 * 60 * 1_000_000_000;

const MEMO_TOO_LONG_ERROR_CODE: usize = 0;

#[derive(Debug, CandidType, Deserialize, Clone)]
pub struct TokenConfiguration {
    pub token_name: String,
    pub token_symbol: String,
    pub transfer_fee: Tokens,
    pub decimals: u8,
    pub minting_account: Option<Account>,
    pub fee_collector_account: Option<Account>,
    pub metadata: Vec<(String, MetadataValue)>,
    pub max_memo_length: Option<u16>,
}

impl Default for TokenConfiguration {
    fn default() -> Self {
        Self {
            token_name: "name".to_string(),
            token_symbol: "symbol".to_string(),
            transfer_fee: 0u32.into(),
            decimals: 0,
            minting_account: None,
            fee_collector_account: None,
            metadata: vec![],
            max_memo_length: None,
        }
    }
}

pub struct TokenService {
    runtime: Rc<RefCell<dyn ICanisterRuntime>>,
    staking: Rc<RefCell<StakingService>>,
    configuration: Rc<RefCell<dyn IConfigurationStore>>,
    balances: Rc<RefCell<dyn IBalanceStore>>,
    transactions: Rc<RefCell<dyn ITransactionStore>>,
}

impl TokenService {
    pub fn new(
        runtime: Rc<RefCell<dyn ICanisterRuntime>>,
        staking: Rc<RefCell<StakingService>>,
        configuration: Rc<RefCell<dyn IConfigurationStore>>,
        balances: Rc<RefCell<dyn IBalanceStore>>,
        transactions: Rc<RefCell<dyn ITransactionStore>>,
    ) -> Self {
        Self {
            staking,
            runtime,
            configuration,
            balances,
            transactions,
        }
    }

    pub fn init(&self, config: TokenConfiguration) {
        self.configuration.borrow_mut().set(config)
    }

    pub fn icrc1_transfer(&self, arg: TransferArg) -> Result<BlockIndex, TransferError> {
        let from = Account {
            owner: self.runtime.borrow().get_caller(),
            subaccount: arg.from_subaccount,
        };

        let tx = TxInfo {
            from,
            to: Some(arg.to),
            amount: arg.amount,
            spender: None,
            memo: arg.memo,
            fee: arg.fee,
            created_at_time: arg.created_at_time,
            expected_allowance: None,
            expires_at: None,
            is_approval: false,
        };

        self.apply_tx(tx)
    }

    pub fn icrc1_balance_of(&self, account: Account) -> Tokens {
        self.balance(account)
    }

    pub fn icrc1_total_supply(&self) -> Tokens {
        self.total_supply()
    }

    pub fn icrc1_minting_account(&self) -> Option<Account> {
        self.configuration.borrow().get().minting_account.clone()
    }

    pub fn icrc1_name(&self) -> String {
        self.configuration.borrow().get().token_name.clone()
    }

    pub fn icrc1_symbol(&self) -> String {
        self.configuration.borrow().get().token_symbol.clone()
    }

    pub fn icrc1_decimals(&self) -> u8 {
        self.configuration.borrow().get().decimals
    }

    pub fn icrc1_fee(&self) -> Tokens {
        self.configuration.borrow().get().transfer_fee.clone()
    }

    pub fn icrc1_metadata(&self) -> Vec<(String, MetadataValue)> {
        vec![
            (
                "icrc1:name".to_string(),
                MetadataValue::Text(self.icrc1_name()),
            ),
            (
                "icrc1:symbol".to_string(),
                MetadataValue::Text(self.icrc1_symbol()),
            ),
            (
                "icrc1:decimals".to_string(),
                MetadataValue::Nat(self.icrc1_decimals().into()),
            ),
            (
                "icrc1:fee".to_string(),
                MetadataValue::Nat(self.icrc1_fee()),
            ),
        ]
    }

    pub fn icrc1_supported_standards(&self) -> Vec<SupportedStandard> {
        vec![
            SupportedStandard {
                name: "ICRC-1".to_string(),
                url: "https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-1".to_string(),
            },
            SupportedStandard {
                name: "ICRC-2".to_string(),
                url: "https://github.com/dfinity/ICRC-1/tree/main/standards/ICRC-2".to_string(),
            },
        ]
    }

    pub fn icrc2_approve(&self, arg: ApproveArgs) -> Result<BlockIndex, ApproveError> {
        Self::validate_memo(arg.memo.as_ref()).map_err(Self::map_approve_error)?;
        let approver_account = Account {
            owner: self.runtime.borrow().get_caller(),
            subaccount: arg.from_subaccount,
        };
        let now = self.runtime.borrow().get_time();
        if let Some(expected_allowance) = arg.expected_allowance.as_ref() {
            let current_allowance = Self::allowance(approver_account, arg.spender, now).allowance;
            if current_allowance != *expected_allowance {
                return Err(ApproveError::AllowanceChanged { current_allowance });
            }
        }
        let tx = TxInfo {
            from: approver_account,
            to: None,
            amount: arg.amount,
            spender: Some(arg.spender),
            memo: arg.memo,
            fee: arg.fee,
            created_at_time: arg.created_at_time,
            expected_allowance: arg.expected_allowance,
            expires_at: arg.expires_at,
            is_approval: true,
        };
        self.apply_tx(tx).map_err(Self::map_approve_error)
    }

    pub fn icrc2_transfer_from(
        &self,
        arg: TransferFromArgs,
    ) -> Result<BlockIndex, TransferFromError> {
        if self.runtime.borrow().get_caller() == arg.from.owner {
            return self
                .icrc1_transfer(TransferArg {
                    to: arg.to,
                    from_subaccount: arg.from.subaccount,
                    amount: arg.amount,
                    fee: arg.fee,
                    memo: arg.memo,
                    created_at_time: arg.created_at_time,
                })
                .map_err(Self::map_transfer_from_error);
        }
        Self::validate_memo(arg.memo.as_ref()).map_err(Self::map_transfer_from_error)?;
        let spender = Account {
            owner: self.runtime.borrow().get_caller(),
            subaccount: arg.spender_subaccount,
        };
        let now = self.runtime.borrow().get_time();
        let allowance = Self::allowance(arg.from, spender, now);
        let transfer_fee = self.configuration.borrow().get().transfer_fee.clone();
        if allowance.allowance < arg.amount.clone() + transfer_fee {
            return Err(TransferFromError::InsufficientAllowance {
                allowance: allowance.allowance,
            });
        }
        let tx = TxInfo {
            from: arg.from,
            to: Some(arg.to),
            amount: arg.amount,
            spender: Some(spender),
            memo: arg.memo,
            fee: arg.fee,
            created_at_time: arg.created_at_time,
            expected_allowance: None,
            expires_at: None,
            is_approval: false,
        };
        self.apply_tx(tx).map_err(Self::map_transfer_from_error)
    }

    pub fn icrc2_allowance(&self, arg: AllowanceArgs) -> Allowance {
        let now = self.runtime.borrow().get_time();
        Self::allowance(arg.account, arg.spender, now)
    }

    pub fn balance(&self, account: Account) -> Tokens {
        self.balances.borrow().get_account_balance(&account)
    }

    fn total_supply(&self) -> Tokens {
        self.balances.borrow().get_total_supply()
    }

    #[allow(unused_variables)]
    fn allowance(account: Account, spender: Account, now: u64) -> Allowance {
        unimplemented!("not used")
    }

    fn validate_created_at_time(
        created_at_time: Option<u64>,
        now: u64,
    ) -> Result<(), TransferError> {
        if let Some(tx_time) = created_at_time {
            if tx_time > now && now - tx_time > TRANSACTION_WINDOW_NANOS + PERMITTED_DRIFT_NANOS {
                return Err(TransferError::CreatedInFuture { ledger_time: now });
            }
            if tx_time < now && now - tx_time > TRANSACTION_WINDOW_NANOS + PERMITTED_DRIFT_NANOS {
                return Err(TransferError::TooOld);
            }
        }
        Ok(())
    }

    fn validate_memo(memo: Option<&Memo>) -> Result<(), TransferError> {
        if let Some(memo) = memo {
            if memo.0.len() > MAX_MEMO_SIZE {
                return Err(TransferError::GenericError {
                    error_code: MEMO_TOO_LONG_ERROR_CODE.into(),
                    message: "Memo too long".into(),
                });
            }
        }
        Ok(())
    }

    fn store_transaction(&self, tx: Transaction, hash: String) -> BlockIndex {
        self.transactions.borrow_mut().add(tx, hash).into()
    }

    fn find_tx(&self, tx: &TxInfo) -> Option<BlockIndex> {
        let hash = tx.build_hash();
        self.transactions.borrow().find_tx(hash).map(|id| id.into())
    }

    fn classify_tx(&self, tx: TxInfo, now: u64) -> Result<Transaction, TransferError> {
        if tx.created_at_time.is_some() {
            if let Some(duplicate_of) = self.find_tx(&tx) {
                return Err(TransferError::Duplicate { duplicate_of });
            }
        }
        if let Some(specified_fee) = tx.fee {
            let expected_fee = self.configuration.borrow().get().transfer_fee.clone();
            if specified_fee != expected_fee {
                return Err(TransferError::BadFee { expected_fee });
            }
        }
        if tx.is_approval {
            return Ok(Transaction {
                kind: "approve".to_string(),
                mint: None,
                burn: None,
                transfer: None,
                approve: Some(Approve {
                    from: tx.from,
                    spender: tx.spender.expect("Bug: failed to forward spender"),
                    amount: tx.amount,
                    expected_allowance: tx.expected_allowance,
                    expires_at: tx.expires_at,
                    memo: tx.memo,
                    fee: Some(self.configuration.borrow().get().transfer_fee.clone()),
                    created_at_time: tx.created_at_time,
                }),
                timestamp: now,
            });
        } else if let Some(minter) = self.configuration.borrow().get().minting_account.clone() {
            if Some(tx.from) == Some(minter) {
                return Ok(Transaction {
                    kind: "mint".to_string(),
                    mint: Some(Mint {
                        amount: tx.amount,
                        to: tx.to.expect("Bug: failed to forward mint receiver"),
                        memo: tx.memo,
                        created_at_time: tx.created_at_time,
                    }),
                    burn: None,
                    transfer: None,
                    approve: None,
                    timestamp: now,
                });
            } else if tx.to == Some(minter) {
                let transfer_fee = self.configuration.borrow().get().transfer_fee.clone();
                if tx.amount < transfer_fee {
                    return Err(TransferError::BadBurn {
                        min_burn_amount: transfer_fee,
                    });
                }
                let balance = self.balance(tx.from);
                if balance < tx.amount.clone() + transfer_fee {
                    return Err(TransferError::InsufficientFunds { balance });
                }
                return Ok(Transaction {
                    kind: "burn".to_string(),
                    mint: None,
                    burn: Some(Burn {
                        amount: tx.amount,
                        from: tx.from,
                        spender: tx.spender,
                        memo: tx.memo,
                        created_at_time: tx.created_at_time,
                    }),
                    transfer: None,
                    approve: None,
                    timestamp: now,
                });
            }
        }
        let balance = self.balance(tx.from);
        if balance < tx.amount.clone() + self.configuration.borrow().get().transfer_fee.clone() {
            return Err(TransferError::InsufficientFunds { balance });
        }
        Ok(Transaction {
            kind: "transfer".to_string(),
            mint: None,
            burn: None,
            transfer: Some(Transfer {
                amount: tx.amount,
                from: tx.from,
                to: tx.to.expect("Bug: failed to forward transfer receiver"),
                spender: tx.spender,
                memo: tx.memo,
                fee: Some(self.configuration.borrow().get().transfer_fee.clone()),
                created_at_time: tx.created_at_time,
            }),
            approve: None,
            timestamp: now,
        })
    }

    fn calculate_account_balance_delta(tx: Transaction) -> HashMap<Account, Int> {
        let mut result = HashMap::new();

        let mut from: Option<Account> = None;
        let mut from_delta = Int::from(0);
        let mut to: Option<Account> = None;
        let mut to_delta = Int::from(0);

        if let Some(mint) = tx.mint {
            to = Some(mint.to);
            to_delta += Int::from(mint.amount);
        } else if let Some(burn) = tx.burn {
            from = Some(burn.from);
            from_delta -= Int::from(burn.amount);
        } else if let Some(transfer) = tx.transfer {
            to = Some(transfer.to);
            to_delta += Int::from(transfer.amount.clone());

            from = Some(transfer.from);
            from_delta -= Int::from(transfer.amount);
            if let Some(fee) = transfer.fee {
                from_delta -= Int::from(fee);
            }
        } else if let Some(approve) = tx.approve {
            if let Some(fee) = approve.fee {
                from = Some(approve.from);
                from_delta -= Int::from(fee);
            }
        }

        if let Some(to) = to {
            result.insert(to, to_delta);
        }
        if let Some(from) = from {
            result.insert(from, from_delta);
        }

        result
    }

    fn calculate_total_supply_delta(tx: Transaction) -> Int {
        let mut result = Int::from(0u8);

        if let Some(mint) = tx.mint {
            result = result.add(Int::from(mint.amount));
        }
        if let Some(burn) = tx.burn {
            result = result.sub(Int::from(burn.amount));
        }
        if let Some(transfer) = tx.transfer {
            if let Some(fee) = transfer.fee {
                result = result.sub(Int::from(fee));
            }
        }
        if let Some(approve) = tx.approve {
            if let Some(fee) = approve.fee {
                result = result.sub(Int::from(fee));
            }
        }

        result
    }

    fn apply_tx(&self, tx: TxInfo) -> Result<BlockIndex, TransferError> {
        Self::validate_memo(tx.memo.as_ref())?;
        let now = self.runtime.borrow().get_time();
        Self::validate_created_at_time(tx.created_at_time, now)?;
        let hash = tx.build_hash();
        let transaction = self.classify_tx(tx, now)?;

        let total_supply = self.balances.borrow().get_total_supply();

        if let Some(staking_context) = StakingService::build_staking_context(self, &transaction) {
            self.staking.borrow().transaction_callback(staking_context);
        }

        let block_index = self.store_transaction(transaction.clone(), hash);

        let total_supply_delta = Self::calculate_total_supply_delta(transaction.clone());
        let new_total_supply = Int::from(total_supply).add(total_supply_delta);
        if new_total_supply < Int::from(0) {
            panic!("total supply must not be less than 0");
        }
        let new_total_supply = Nat::from(new_total_supply.0.to_biguint().unwrap());
        self.balances
            .borrow_mut()
            .udpate_total_supply(new_total_supply);

        let balance_changes = Self::calculate_account_balance_delta(transaction);
        for balance_change in balance_changes {
            let account = balance_change.0;
            let delta = balance_change.1;
            let current_balance = self.balances.borrow().get_account_balance(&account);
            let new_balance = Int::from(current_balance).add(delta);
            if new_balance < Int::from(0) {
                panic!("account balance must not be less than 0");
            }
            let new_balance = Nat::from(new_balance.0.to_biguint().unwrap());
            self.balances
                .borrow_mut()
                .update_account_balance(account, new_balance);
        }

        Ok(block_index)
    }

    fn map_approve_error(err: TransferError) -> ApproveError {
        match err {
            TransferError::BadFee { expected_fee } => ApproveError::BadFee { expected_fee },
            TransferError::TooOld => ApproveError::TooOld,
            TransferError::CreatedInFuture { ledger_time } => {
                ApproveError::CreatedInFuture { ledger_time }
            }
            TransferError::TemporarilyUnavailable => ApproveError::TemporarilyUnavailable,
            TransferError::Duplicate { duplicate_of } => ApproveError::Duplicate { duplicate_of },
            TransferError::GenericError {
                error_code,
                message,
            } => ApproveError::GenericError {
                error_code,
                message,
            },
            TransferError::BadBurn { .. } | TransferError::InsufficientFunds { .. } => {
                ic_cdk::trap("Bug: cannot transform TransferError into ApproveError")
            }
        }
    }

    fn map_transfer_from_error(err: TransferError) -> TransferFromError {
        match err {
            TransferError::BadFee { expected_fee } => TransferFromError::BadFee { expected_fee },
            TransferError::TooOld => TransferFromError::TooOld,
            TransferError::CreatedInFuture { ledger_time } => {
                TransferFromError::CreatedInFuture { ledger_time }
            }
            TransferError::TemporarilyUnavailable => TransferFromError::TemporarilyUnavailable,
            TransferError::Duplicate { duplicate_of } => {
                TransferFromError::Duplicate { duplicate_of }
            }
            TransferError::GenericError {
                error_code,
                message,
            } => TransferFromError::GenericError {
                error_code,
                message,
            },
            TransferError::InsufficientFunds { balance } => {
                TransferFromError::InsufficientFunds { balance }
            }
            TransferError::BadBurn { min_burn_amount } => {
                TransferFromError::BadBurn { min_burn_amount }
            }
        }
    }
}

#[derive(Debug, CandidType)]
pub struct TxInfo {
    pub from: Account,
    pub to: Option<Account>,
    pub amount: Tokens,
    pub spender: Option<Account>,
    pub memo: Option<Memo>,
    pub fee: Option<Tokens>,
    pub created_at_time: Option<u64>,
    pub expected_allowance: Option<Tokens>,
    pub expires_at: Option<u64>,
    pub is_approval: bool,
}

impl TxInfo {
    pub fn build_hash(&self) -> String {
        let bytes = candid::encode_one(&self);
        let hex = hex::encode(bytes.unwrap());
        hex
    }
}

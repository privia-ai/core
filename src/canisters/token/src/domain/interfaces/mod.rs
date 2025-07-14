use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc3::transactions::Transaction;
use abstractions::token::StakingLogEntry;
use abstractions::Tokens;
use crate::domain::token::TokenConfiguration;

pub trait IStakingStore {
    fn get_log_entries(&self, address: Account, from: u64, to: u64) -> Vec<StakingLogEntry>;
    fn add_log_entry(&mut self, address: Account, log: &StakingLogEntry);
}

pub trait IBalanceStore {
    fn get_account_balance(&self, account: &Account) -> Tokens;
    fn get_total_supply(&self) -> Tokens;
    fn update_account_balance(&mut self, account: Account, new_value: Tokens);
    fn udpate_total_supply(&mut self, new_value: Tokens);
}

pub trait IConfigurationStore {
    fn get(&self) -> TokenConfiguration;
    fn set(&mut self, configuration: TokenConfiguration);
}

pub trait ITransactionStore {
    fn len(&self) -> u64;
    fn add(&mut self, transaction: Transaction, hash: String) -> u64;
    fn find_tx(&self, hash: String) -> Option<u64>;
}
use icrc_ledger_types::icrc1::account::Account;
use abstractions::dao::{Discount, Proposal, Vote};

pub trait IDiscountStorage {
    fn get_cycle_discounts_ids(&self, cycle_number: u64) -> Vec<u128>;
    fn add_discount(&mut self, cycle_number: u64, data: Discount) -> u128;
    fn get_discount_index(&self, account: &Account, cycle_number: u64) -> u128;
    fn increase_discount_index(&mut self, account: Account, cycle_number: u64) -> u128;
}

pub trait IHivingStorage {
    fn add_hiving_wallet(&mut self, wallet: Account);
    fn get_hiving_wallets(&self) -> Vec<Account>;
    fn add_wallet_usage_per_cycle(&mut self, cycle_number: u64, wallet: Account) -> u32;
    fn get_wallet_usage_per_cycle(&self, cycle_number: u64, wallet: Account) -> u32;
}

pub trait IVotingStorage {
    fn add_proposal(&mut self, proposal: Proposal) -> u64;
    fn get_proposal(&self, id: &u64) -> Option<Proposal>;
    fn update_proposal(&mut self, proposal: Proposal);

    fn add_vote(&mut self, vote: Vote) -> u64;
    fn get_vote(&self, id: &u64) -> Option<Vote>;
    fn get_all_votes(&self, proposal_id: &u64) -> Vec<Vote>;
}
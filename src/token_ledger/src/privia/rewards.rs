use crate::privia::{add, multiply, sub, IStakingRegistry, StakingLogEntry};
use candid::{Nat, Principal};
use num_bigint::BigUint;

pub fn calculate(repo: &dyn IStakingRegistry, address: Principal, from: u64, to: u64) -> Nat {
    let log_inside_range = repo.get_log_entries(address, from.clone(), to.clone());

    let mut sum: Nat = BigUint::ZERO.into();
    let log_len = log_inside_range.len();
    if log_len > 0 {
        let mut i = 0;

        let mut previous_entry: Option<StakingLogEntry> = None;

        for log_entry in log_inside_range {
            let entry_timestamp = log_entry.timestamp.clone();
            let current_amount = log_entry.current_amount.clone();
            let delta = sub(&current_amount, &log_entry.previous_amount);
            
            if i == 0 {
                let period_start = from;
                let period_end = entry_timestamp;
                let period_diff = Nat::from(period_end - period_start);
                let period_amount = sub(&current_amount, &delta);

                let period_reward = multiply(&period_diff, &period_amount);
                sum = add(&sum, &period_reward);
            } else {
                let previous_entry = previous_entry.unwrap();

                let period_start = previous_entry.timestamp;
                let period_end = entry_timestamp;
                let period_diff = Nat::from(period_end - period_start);
                let period_amount = previous_entry.current_amount;

                let period_reward = multiply(&period_diff, &period_amount);
                sum = add(&sum, &period_reward);
            }

            if i == log_len - 1 {
                let period_start = entry_timestamp;
                let period_end = to;
                let period_diff = Nat::from(period_end - period_start);
                let period_amount = current_amount;

                let period_reward = multiply(&period_diff, &period_amount);
                sum = add(&sum, &period_reward);
            }

            i += 1;
            previous_entry = Some(log_entry);
        }

        return sum;
    }

    let latest_log_entry = repo.get_latest_log_entry(address, from.clone());
    match latest_log_entry {
        None => BigUint::ZERO.into(),
        Some(log_entry) => multiply(&Nat::from(to - from), &log_entry.current_amount),
    }
}

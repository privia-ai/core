use std::cell::RefCell;
use std::rc::Rc;
use candid::Nat;
use abstractions::token::StakingLogEntry;
use crate::domain::cycles::CycleService;

pub struct LinearMinScorer {
    cycles: Rc<RefCell<CycleService>>,
}

impl LinearMinScorer {
    pub fn new(cycles: Rc<RefCell<CycleService>>) -> Self {
        Self { cycles }
    }

    /// calculates the staking score on the moment of 'target_cycle' start
    pub async fn calculate_score(&self, log: &[StakingLogEntry], cycle_number: u64) -> Nat {
        if log.is_empty() {
            return Nat::from(0u8);
        }

        let min_by_cycle: Vec<CycleBalanceChange> = self.split_into_cycles(log);
        let result = Self::calc_cycles_outcome(&min_by_cycle, cycle_number);

        result
    }

    /// calculates the staking score on the moment of 'target_cycle' start
    pub(self) fn calc_cycles_outcome(splitted: &[CycleBalanceChange], target_cycle: u64) -> Nat {
        let mut result = Nat::from(0u8);

        let first_item = splitted.first().unwrap();
        let mut last_change_cycle = first_item.cycle + 1;
        let mut last_change_value = first_item.min_amount.clone();

        for item in splitted.iter().skip(1) {
            let item_cycle = item.cycle;
            let item_value = item.min_amount.clone();

            if item_value > last_change_value {
                let so_far = (item_cycle + 1 - last_change_cycle) * last_change_value;
                result += so_far;
                last_change_value = item_value;
                last_change_cycle = item_cycle + 1;
            } else {
                let so_far = (item_cycle - last_change_cycle) * last_change_value.clone();
                result += so_far;
                last_change_cycle = item_cycle;
                last_change_value = item_value;
            }
        }

        result += (target_cycle - last_change_cycle) * last_change_value.clone();

        result
    }

    /// returns the list of changes in the logged in the form of pairs of values:
    /// cycle where the min stake value has changed and the corresponding stake value
    fn split_into_cycles(&self, log: &[StakingLogEntry]) -> Vec<CycleBalanceChange> {
        let mut result: Vec<CycleBalanceChange> = Vec::new();

        let mut last: CycleBalanceChange = CycleBalanceChange {
            cycle: 0,
            min_amount: Nat::from(0u8),
        };

        for entry in log.iter() {
            let entry_cycle = self.cycles.borrow().resolve_cycle(entry.timestamp);
            let current = CycleBalanceChange::new(entry_cycle.number, entry.current_amount.clone());

            if current.cycle > last.cycle {
                result.push(current.clone());
                last = current;
            } else {
                if current.min_amount < last.min_amount {
                    last.min_amount = current.min_amount;
                }
            }
        }

        result
    }
}

#[derive(Clone)]
struct CycleBalanceChange {
    pub cycle: u64,
    pub min_amount: Nat,
}

impl CycleBalanceChange {
    pub fn new(cycle: u64, min_amount: Nat) -> Self {
        Self { cycle, min_amount }
    }
}

#[cfg(test)]
mod scorer_tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = Vec::from([
            CycleBalanceChange {
                cycle: 5,
                min_amount: Nat::from(7u8),
            },
            CycleBalanceChange {
                cycle: 9,
                min_amount: Nat::from(15u8),
            },
            CycleBalanceChange {
                cycle: 11,
                min_amount: Nat::from(11u8),
            },
        ]);
        let target_cycle = 14;

        let result = LinearMinScorer::calc_cycles_outcome(&data, target_cycle);
        assert_eq!(result, Nat::from(76u8));
    }
}
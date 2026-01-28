use abstractions::dao::Cycle;
use abstractions::runtime::ICanisterRuntime;
use abstractions::Timestamp;
use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;

const NSEC_IN_SEC: u64 = 1_000_000_000;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct CyclesConfig {
    pub hiving_cycles: u64,
    pub voting_cycles: u64,
    pub genesis: Option<Timestamp>,
    pub cycle_len_ns: u64,
}

impl Default for CyclesConfig {
    fn default() -> Self {
        Self {
            hiving_cycles: 4,
            voting_cycles: 4,
            genesis: Some(1750777200 * NSEC_IN_SEC), // 24.06.2025 17:30 CET
            cycle_len_ns: 60 * NSEC_IN_SEC,          // 1 min
        }
    }
}

pub struct CycleService {
    pub config: CyclesConfig,
    runtime: Rc<RefCell<dyn ICanisterRuntime>>,
}

impl CycleService {
    pub fn new(config: CyclesConfig, runtime: Rc<RefCell<dyn ICanisterRuntime>>) -> Self {
        Self { config, runtime }
    }
    
    pub fn get_current_cycle(&self) -> Cycle {
        let now = self.runtime.borrow().get_time();
        let cycle = self.resolve_cycle(now);
        
        cycle
    }

    pub fn get_next_voting_cycle(&self) -> Cycle {
        fn next_strict_multiple(n: u64, divider: u64) -> u64 {
            n + divider - n % divider
        }

        let current_cycle = self.get_current_cycle();
        let next_voting_cycle_number =
            next_strict_multiple(current_cycle.number, self.config.voting_cycles);
        let cycle = self.get_cycle_details(next_voting_cycle_number);

        cycle
    }

    pub fn get_cycle_details(&self, cycle_number: u64) -> Cycle {
        let genesis = self.config.genesis.unwrap();
        let start = genesis + (cycle_number - 1) * self.config.cycle_len_ns;
        let end = start + self.config.cycle_len_ns;

        Cycle {
            number: cycle_number,
            start,
            end,
        }
    }
    
    pub fn resolve_cycle(&self, timestamp: Timestamp) -> Cycle {
        let genesis = self.config.genesis.unwrap();
        let elapsed_nanosec = timestamp - genesis;
        let div = elapsed_nanosec / self.config.cycle_len_ns;        
        let rem = elapsed_nanosec % self.config.cycle_len_ns;
        
        let cycle_number = if rem > 0 {div+1} else {div};
        let cycle = self.get_cycle_details(cycle_number);
        
        cycle
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    struct RtMock {
        time: u64,
    }

    impl RtMock {
        pub fn new() -> Self {
            Self { time: 0 }
        }

        pub fn set_time_since_genesis_sec(&mut self, time_sec: u64) {
            self.time = time_sec * NSEC_IN_SEC;
        }
    }

    impl ICanisterRuntime for RtMock {
        fn get_caller(&self) -> Principal {
            Principal::anonymous()
        }

        fn get_time(&self) -> Timestamp {
            self.time
        }
    }

    #[test]
    fn it_works() {
        let rt_mock = Rc::new(RefCell::new(RtMock::new()));
        let rt: Rc<RefCell<dyn ICanisterRuntime>> = rt_mock.clone();
        let cycles_service = CycleService::new(
            CyclesConfig {
                hiving_cycles: 4,
                voting_cycles: 4,
                genesis: Some(0),
                cycle_len_ns: 10 * NSEC_IN_SEC,
            },
            Rc::clone(&rt),
        );

        rt_mock.borrow_mut().set_time_since_genesis_sec(5);

        let _cycle = cycles_service.get_cycle_details(2);

        let cycle = cycles_service.get_current_cycle().number;
        assert_eq!(cycle, 1);
    }
    
    #[test]
    fn resolve_cycle() {
        let result = 13 / 4;
        let rem = 13 % 4;
        assert_eq!(result, 3);
        assert_eq!(rem, 1);
    }
}

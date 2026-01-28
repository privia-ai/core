use std::cell::RefCell;
use std::rc::Rc;

use abstractions::hiving::{
    ContractId, ContractStatus, DiscountContract, PoolJoinProof, PoolParticipant, PriceQuote,
    TimeUnits,
};
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Deserialize, Serialize, candid::CandidType)]
pub struct PoolState {
    pub participants: Vec<PoolParticipant>,
    pub contracts: Vec<DiscountContract>,
    pub pricing_per_time_unit: Nat,
    pub config: PoolConfig,
    pub available_time: TimeUnits,
    pub paused: bool,
}

impl PoolState {
    pub fn set_config(&mut self, cfg: PoolConfig) {
        self.config = cfg;
    }

    pub fn get_config(&self) -> PoolConfig {
        self.config.clone()
    }

    pub fn join(&mut self, proof: PoolJoinProof, declared_time: TimeUnits) {
        if let Some(existing) = self
            .participants
            .iter_mut()
            .find(|p| p.principal == proof.principal)
        {
            let prev = existing.declared_time;
            existing.declared_time = declared_time;
            existing.share = Nat::from(declared_time);
            // Adjust availability by diff
            if declared_time >= prev {
                self.available_time = self
                    .available_time
                    .saturating_add(declared_time - prev);
            } else {
                self.available_time = self
                    .available_time
                    .saturating_sub(prev - declared_time);
            }
            return;
        }

        let participant = PoolParticipant {
            principal: proof.principal,
            declared_time,
            share: Nat::from(declared_time),
            pending_rewards: Nat::from(0u32),
        };
        self.participants.push(participant);
        self.available_time = self.available_time.saturating_add(declared_time);
    }

    pub fn leave(&mut self, principal: candid::Principal) {
        if let Some(idx) = self
            .participants
            .iter()
            .position(|p| p.principal == principal)
        {
            let reclaimed = self.participants[idx].declared_time;
            self.available_time = self.available_time.saturating_sub(reclaimed);
            self.participants.remove(idx);
        }
    }

    pub fn total_declared_time(&self) -> TimeUnits {
        self.participants.iter().map(|p| p.declared_time).sum()
    }

    pub fn quote(&self, time_units: TimeUnits) -> PriceQuote {
        if self.paused {
            return PriceQuote {
                time_units,
                ckusdc_cost: Nat::from(0u32),
                discount_value: 0.0,
            };
        }
        let price = self.pricing_per_time_unit.clone() * Nat::from(time_units);
        let discount_value = time_units as f32;
        PriceQuote {
            time_units,
            ckusdc_cost: price,
            discount_value,
        }
    }

    pub fn reserve_time(&mut self, time_units: TimeUnits) -> Result<(), String> {
        if self.paused {
            return Err("Pool is paused".to_string());
        }
        if self.available_time < time_units {
            return Err("Not enough pool time".to_string());
        }
        self.available_time -= time_units;
        Ok(())
    }

    pub fn release_time(&mut self, time_units: TimeUnits) {
        self.available_time = self.available_time.saturating_add(time_units);
    }

    pub fn open_contract(
        &mut self,
        buyer: Account,
        time_units: TimeUnits,
        price: Nat,
        discount_value: f32,
    ) -> ContractId {
        let id = self.contracts.len() as ContractId;
        let seller = candid::Principal::anonymous();
        let contract = DiscountContract {
            id,
            buyer,
            seller,
            time_units,
            price,
            discount_value,
            status: ContractStatus::Paid,
        };
        self.contracts.push(contract);
        id
    }

    pub fn distribute_rewards(&mut self, amount: Nat) {
        let total_time = self.total_declared_time();
        if total_time == 0 {
            return;
        }
        let total_nat = Nat::from(total_time);
        for participant in self.participants.iter_mut() {
            let share = Nat::from(participant.declared_time);
            let portion = amount.clone() * share / total_nat.clone();
            participant.pending_rewards += portion;
        }
    }

    pub fn claim_rewards(&mut self, principal: &candid::Principal) -> Option<Nat> {
        let participant = self
            .participants
            .iter_mut()
            .find(|p| &p.principal == principal)?;
        let rewards = participant.pending_rewards.clone();
        participant.pending_rewards = Nat::from(0u32);
        Some(rewards)
    }

    pub fn get_contract(&self, id: ContractId) -> Option<&DiscountContract> {
        self.contracts.get(id as usize)
    }

    pub fn mark_redeemed(&mut self, id: ContractId) -> Option<DiscountContract> {
        if let Some(contract) = self.contracts.get_mut(id as usize) {
            contract.status = ContractStatus::Redeemed;
            return Some(contract.clone());
        }
        None
    }
}

thread_local! {
    static STATE: Rc<RefCell<PoolState>> = Rc::new(RefCell::new(PoolState::default()));
}

pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut PoolState) -> R,
{
    STATE.with(|s| f(&mut s.borrow_mut()))
}

#[derive(Clone, Debug, Deserialize, Serialize, candid::CandidType)]
pub struct PoolConfig {
    pub dao_canister_id: Principal,
    pub ckusdc_canister_id: Principal,
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            dao_canister_id: Principal::anonymous(),
            ckusdc_canister_id: Principal::anonymous(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distributes_rewards_pro_rata() {
        let mut state = PoolState::default();
        let p1 = Principal::anonymous();
        let p2 = Principal::from_slice(&[1u8]);
        state.join(PoolJoinProof { principal: p1, signature: None, note: None }, 10);
        state.join(PoolJoinProof { principal: p2, signature: None, note: None }, 30);

        state.distribute_rewards(Nat::from(40u32));

        let first = state
            .participants
            .iter()
            .find(|p| p.principal == p1)
            .unwrap();
        let second = state
            .participants
            .iter()
            .find(|p| p.principal == p2)
            .unwrap();

        assert_eq!(Nat::from(10u32), first.pending_rewards);
        assert_eq!(Nat::from(30u32), second.pending_rewards);

        let claimed = state.claim_rewards(&p1).unwrap();
        assert_eq!(Nat::from(10u32), claimed);
        assert_eq!(Nat::from(0u32), state.claim_rewards(&p1).unwrap());
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use abstractions::hiving::{
    ContractId, ContractStatus, DiscountContract, HiverId, HiverRegistration, PriceQuote,
    TimeUnits,
};
use candid::Nat;
use std::collections::BTreeMap;
use icrc_ledger_types::icrc1::account::Account;
use serde::{Deserialize, Serialize};
use candid::Principal;
use abstractions::MetadataValue;
use abstractions::dao::Discount;

#[derive(Default, Clone, Deserialize, Serialize, candid::CandidType)]
pub struct HivingState {
    pub config: HivingConfig,
    pub hivers: Vec<HiverRegistration>,
    pub available_time: Vec<TimeUnits>,
    pub contracts: Vec<DiscountContract>,
    pub cycle_usage: BTreeMap<(HiverId, u64), TimeUnits>,
}

impl HivingState {
    pub fn set_config(&mut self, cfg: HivingConfig) {
        self.config = cfg;
    }

    pub fn get_config(&self) -> HivingConfig {
        self.config.clone()
    }

    pub fn add_hiver(&mut self, reg: HiverRegistration) -> HiverId {
        let id = self.hivers.len() as HiverId;
        self.hivers.push(reg);
        self.available_time.push(self.hivers[id as usize].max_time_units);
        id
    }

    pub fn get_hiver(&self, id: HiverId) -> Option<&HiverRegistration> {
        self.hivers.get(id as usize)
    }

    pub fn get_available_time(&self, id: HiverId) -> Option<TimeUnits> {
        self.available_time.get(id as usize).copied()
    }

    pub fn quote(&self, hiver_id: HiverId, time_units: TimeUnits) -> Option<PriceQuote> {
        let hiver = self.get_hiver(hiver_id)?;
        let price = hiver.price_per_time_unit.clone() * Nat::from(time_units);
        let discount_value = time_units as f32;

        Some(PriceQuote {
            time_units,
            ckusdc_cost: price,
            discount_value,
        })
    }

    pub fn reserve_time(&mut self, hiver_id: HiverId, time_units: TimeUnits) -> Result<(), String> {
        let idx = hiver_id as usize;
        let available = self.available_time.get_mut(idx).ok_or("Hiver not found")?;
        if *available < time_units {
            return Err("Not enough available staking time".to_string());
        }
        *available -= time_units;
        Ok(())
    }

    pub fn release_time(&mut self, hiver_id: HiverId, time_units: TimeUnits) {
        if let Some(avail) = self.available_time.get_mut(hiver_id as usize) {
            *avail = avail.saturating_add(time_units);
        }
    }

    pub fn check_cycle_cap(
        &self,
        hiver_id: HiverId,
        cycle_number: u64,
        time_units: TimeUnits,
    ) -> Result<(), String> {
        let reg = self.get_hiver(hiver_id).ok_or("Hiver not found")?;
        let used = self
            .cycle_usage
            .get(&(hiver_id, cycle_number))
            .cloned()
            .unwrap_or(0);
        if used + time_units > reg.cycle_cap {
            return Err("Cycle cap exceeded".to_string());
        }
        Ok(())
    }

    pub fn mark_cycle_usage(
        &mut self,
        hiver_id: HiverId,
        cycle_number: u64,
        time_units: TimeUnits,
    ) {
        let entry = self
            .cycle_usage
            .entry((hiver_id, cycle_number))
            .or_insert(0);
        *entry = entry.saturating_add(time_units);
    }

    pub fn open_contract(
        &mut self,
        buyer: Account,
        seller: candid::Principal,
        time_units: TimeUnits,
        price: Nat,
        discount_value: f32,
    ) -> ContractId {
        let id = self.contracts.len() as ContractId;
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
    static STATE: Rc<RefCell<HivingState>> = Rc::new(RefCell::new(HivingState::default()));
}

pub fn with_state<F, R>(f: F) -> R
where
    F: FnOnce(&mut HivingState) -> R,
{
    STATE.with(|s| f(&mut s.borrow_mut()))
}

pub fn build_discount_from_contract(
    contract: &DiscountContract,
) -> Discount {
    let metadata = vec![("value".to_string(), MetadataValue::Text(contract.discount_value.to_string()))];
    let mut discount = Discount::new(contract.discount_value, contract.buyer.clone());
    discount.id = contract.id as u128;

    // The DAO discount service will recreate metadata; we keep it here for clarity.
    let _ = metadata;

    discount
}

#[derive(Clone, Debug, Deserialize, Serialize, candid::CandidType)]
pub struct HivingConfig {
    pub dao_canister_id: Principal,
    pub ckusdc_canister_id: Principal,
}

impl Default for HivingConfig {
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
    use candid::Principal;

    #[test]
    fn reserves_and_releases_time() {
        let mut state = HivingState::default();
        state.add_hiver(HiverRegistration {
            principal: Principal::anonymous(),
            metadata: vec![],
            price_per_time_unit: Nat::from(1u32),
            max_time_units: 10,
            cycle_cap: 5,
        });

        assert_eq!(Some(10), state.get_available_time(0));
        assert!(state.reserve_time(0, 4).is_ok());
        assert_eq!(Some(6), state.get_available_time(0));
        state.release_time(0, 2);
        assert_eq!(Some(8), state.get_available_time(0));
    }

    #[test]
    fn cycle_cap_blocks_excess() {
        let mut state = HivingState::default();
        state.add_hiver(HiverRegistration {
            principal: Principal::anonymous(),
            metadata: vec![],
            price_per_time_unit: Nat::from(1u32),
            max_time_units: 10,
            cycle_cap: 3,
        });

        assert!(state.check_cycle_cap(0, 1, 2).is_ok());
        assert!(state.check_cycle_cap(0, 1, 4).is_err());
        state.mark_cycle_usage(0, 1, 2);
        assert!(state.check_cycle_cap(0, 1, 2).is_err());
    }
}

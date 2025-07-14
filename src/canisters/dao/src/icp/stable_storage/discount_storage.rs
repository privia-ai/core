use std::borrow::Cow;
use ic_stable_structures::{StableBTreeMap, Storable};
use ic_stable_structures::storable::Bound;
use abstractions::dao::Discount;
use crate::domain::interfaces::storage::*;
use super::IcpMemory;

pub struct DiscountStorageStable {
    cycle_discounts_index: StableBTreeMap<u64, String, IcpMemory>,
    discounts: StableBTreeMap<u128, StorableDiscount, IcpMemory>,
    account_cycle_index: StableBTreeMap<(BoundedAccount, u64), u128, IcpMemory>
}

impl IDiscountStorage for DiscountStorageStable {
    fn get_cycle_discounts_ids(&self, cycle_number: u64) -> Vec<u128> {
        let ids_str = self
            .cycle_discounts_index
            .get(&cycle_number)
            .unwrap_or(String::new());

        if ids_str.is_empty() {
            return vec![];
        }

        let result = Self::parse_string_with_ids(ids_str);
        result
    }

    fn add_discount(&mut self, cycle_number: u64, discount: Discount) -> u128 {
        let id = self.discounts.len() as u128;
        self.discounts.insert(id, StorableDiscount(discount));

        let old_index = self.cycle_discounts_index.get(&cycle_number);
        let new_index = match old_index {
            Some(value) => value.add(&format!(",{id}")),
            None => id.to_string(),
        };
        self.cycle_discounts_index.insert(cycle_number, new_index);

        id
    }
    
    fn get_discount_index(&self, account: &Account, cycle_number: u64) -> u128 {
        let pk = (BoundedAccount(account.clone()), cycle_number);
        self.account_cycle_index.get(&pk).unwrap_or(0)
    }

    fn increase_discount_index(&mut self, account: Account, cycle_number: u64) -> u128 {
        let pk = (BoundedAccount(account.clone()), cycle_number);
        let current_count = self.account_cycle_index.get(&pk).unwrap_or(0);
        let new_count = current_count + 1;
        self.account_cycle_index.insert(pk, current_count + 1);
        new_count
    }
}

pub struct StorableDiscount(pub Discount);

impl Storable for StorableDiscount {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: Discount = candid::decode_one(&bytes).unwrap();
        StorableDiscount(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl DiscountStorageStable {
    pub fn init() -> Self {
        Self {
            cycle_discounts_index: StableBTreeMap::init(super::get_cycle_discounts_index_memory()),
            discounts: StableBTreeMap::init(super::get_discounts_memory()),
            account_cycle_index: StableBTreeMap::init(super::get_account_cycles_index_memory())
        }
    }

    fn parse_string_with_ids(ids_str: String) -> Vec<u128> {
        let mut result = Vec::new();

        let split =  ids_str.split(",");
        for id_str in split.into_iter() {
            let id = u128::from_str_radix(id_str, 10).unwrap();
            result.push(id);
        }

        result
    }
}

use std::ops::{Add, Deref, DerefMut};
use candid::Principal;
use icrc_ledger_types::icrc1::account::Account;

impl Deref for StorableDiscount {
    type Target = Discount;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StorableDiscount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Ord, PartialOrd, PartialEq, Eq)]
struct BoundedAccount(pub Account);

impl Storable for BoundedAccount {
    fn to_bytes(&self) -> Cow<[u8]> {
        let Account { owner, subaccount } = &self.0;

        let owner_bytes = owner.as_slice();
        let mut buf = Vec::with_capacity(100);

        // Encode Principal with fixed length: [len (1 byte)] + [29 bytes max]
        buf.push(owner_bytes.len() as u8);
        buf.extend_from_slice(owner_bytes);
        buf.resize(1 + 29, 0); // pad to 30 total bytes

        // Encode Option<[u8; 32]> as [present (1 byte)] + [32 bytes]
        match subaccount {
            Some(sa) => {
                buf.push(1);
                buf.extend_from_slice(sa);
            }
            None => {
                buf.push(0);
                buf.extend_from_slice(&[0u8; 32]);
            }
        }

        // Pad remaining to reach 100 bytes if needed
        buf.resize(100, 0);

        Cow::Owned(buf)
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let bytes = bytes.as_ref();

        let owner_len = bytes[0] as usize;
        let owner = Principal::from_slice(&bytes[1..1 + owner_len]);

        let sub_present = bytes[30] == 1;
        let mut sub = [0u8; 32];
        sub.copy_from_slice(&bytes[31..63]);

        let account = Account {
            owner,
            subaccount: if sub_present { Some(sub) } else { None },
        };

        BoundedAccount(account)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 100,
        is_fixed_size: true,
    };
}
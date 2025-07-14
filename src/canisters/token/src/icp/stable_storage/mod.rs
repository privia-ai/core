use crate::domain::interfaces::{
    IBalanceStore, IConfigurationStore, IStakingStore, ITransactionStore,
};
use crate::domain::token::TokenConfiguration;
use abstractions::token::StakingLogEntry;
use abstractions::{Account, Tokens};
use candid::Principal;
use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory}, storable::Bound, DefaultMemoryImpl, StableBTreeMap, StableCell, StableLog,
    StableVec,
    Storable,
};
use icrc_ledger_types::icrc3::transactions::Transaction;
use std::borrow::Cow;
use std::cell::RefCell;

type IcpMemory = VirtualMemory<DefaultMemoryImpl>;

const CONFIGURATION_MEMORY_ID: MemoryId = MemoryId::new(1);
const TRANSACTION_LOG_MEMORY_ID: MemoryId = MemoryId::new(2);
const ACCOUNT_BALANCE_MEMORY_ID: MemoryId = MemoryId::new(3);
const TOTAL_SUPPLY_MEMORY_ID: MemoryId = MemoryId::new(4);
const TRANSACTION_HASHES_MEMORY_ID: MemoryId = MemoryId::new(5);

const STAKING_MAPPING_MEMORY_ID: MemoryId = MemoryId::new(11);
const STAKING_LOG_DATA_MEMORY_ID: MemoryId = MemoryId::new(12);
const STAKING_LOG_INDEX_MEMORY_ID: MemoryId = MemoryId::new(13);

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

fn get_configuration_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(CONFIGURATION_MEMORY_ID))
}

fn get_transaction_log_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTION_LOG_MEMORY_ID))
}

fn get_transaction_hashes_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TRANSACTION_HASHES_MEMORY_ID))
}

fn get_account_balance_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(ACCOUNT_BALANCE_MEMORY_ID))
}

fn get_total_supply_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(TOTAL_SUPPLY_MEMORY_ID))
}

fn get_log_index_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_MAPPING_MEMORY_ID))
}

fn get_log_idx_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_LOG_INDEX_MEMORY_ID))
}

fn get_log_data_memory() -> IcpMemory {
    MEMORY_MANAGER.with(|m| m.borrow().get(STAKING_LOG_DATA_MEMORY_ID))
}

pub struct BalanceStoreStable {
    account_index: StableBTreeMap<Account, TokensStorable, IcpMemory>,
    total_supply: StableCell<TokensStorable, IcpMemory>,
}

impl IBalanceStore for BalanceStoreStable {
    fn get_account_balance(&self, account: &Account) -> Tokens {
        self.account_index
            .get(account)
            .map(|s| s.0)
            .unwrap_or(0u8.into())
    }

    fn get_total_supply(&self) -> Tokens {
        self.total_supply.get().0.clone()
    }

    fn update_account_balance(&mut self, account: Account, new_value: Tokens) {
        self.account_index
            .insert(account, TokensStorable(new_value));
    }

    fn udpate_total_supply(&mut self, new_value: Tokens) {
        self.total_supply.set(TokensStorable(new_value)).unwrap();
    }
}

struct TokensStorable(pub Tokens);

impl Storable for TokensStorable {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: Tokens = candid::decode_one(&bytes).unwrap();
        TokensStorable(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}

impl BalanceStoreStable {
    pub fn init() -> Self {
        Self {
            account_index: StableBTreeMap::init(get_account_balance_memory()),
            total_supply: StableCell::init(get_total_supply_memory(), TokensStorable(0u8.into()))
                .unwrap(),
        }
    }
}

pub struct ConfigurationStoreStable {
    configuration: StableCell<TokenConfigurationStorable, IcpMemory>,
}

impl ConfigurationStoreStable {
    pub fn init() -> Self {
        Self {
            configuration: StableCell::init(
                get_configuration_memory(),
                TokenConfigurationStorable(TokenConfiguration::default()),
            )
            .unwrap(),
        }
    }
}

impl IConfigurationStore for ConfigurationStoreStable {
    fn get(&self) -> TokenConfiguration {
        self.configuration.get().0.clone()
    }
    fn set(&mut self, configuration: TokenConfiguration) {
        self.configuration
            .set(TokenConfigurationStorable(configuration))
            .unwrap();
    }
}

pub struct TransactionsStoreStable {
    transaction_log: StableVec<TransactionStorable, IcpMemory>,
    hashes: StableBTreeMap<String, u64, IcpMemory>,
}

impl TransactionsStoreStable {
    pub fn init() -> Self {
        Self {
            transaction_log: StableVec::init(get_transaction_log_memory()).unwrap(),
            hashes: StableBTreeMap::init(get_transaction_hashes_memory()),
        }
    }
}

impl ITransactionStore for TransactionsStoreStable {
    fn len(&self) -> u64 {
        self.transaction_log.len()
    }

    fn add(&mut self, transaction: Transaction, hash: String) -> u64 {
        let storable = TransactionStorable(transaction);
        self.transaction_log.push(&storable).unwrap();
        let block_index = self.len();

        self.hashes.insert(hash, block_index);

        block_index
    }

    fn find_tx(&self, hash: String) -> Option<u64> {
        self.hashes.get(&hash)
    }
}

pub struct TokenConfigurationStorable(pub TokenConfiguration);

impl Storable for TokenConfigurationStorable {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: TokenConfiguration = candid::decode_one(&bytes).unwrap();
        TokenConfigurationStorable(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub struct TransactionStorable(pub Transaction);

impl Storable for TransactionStorable {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: Transaction = candid::decode_one(&bytes).unwrap();
        TransactionStorable(inner)
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: 1000,
        is_fixed_size: false,
    };
}

struct StakingLogEntryStorable(pub StakingLogEntry);

impl Storable for StakingLogEntryStorable {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(&self.0).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let inner: StakingLogEntry = candid::decode_one(&bytes).unwrap();
        StakingLogEntryStorable(inner)
    }

    const BOUND: Bound = Bound::Unbounded;
}

pub struct StakingStoreStable {
    log_index: StableBTreeMap<(BoundedAccount, u64), u64, IcpMemory>,
    log: StableLog<StakingLogEntryStorable, IcpMemory, IcpMemory>,
}

impl StakingStoreStable {
    pub fn init() -> Self {
        Self {
            log_index: StableBTreeMap::init(get_log_index_memory()),
            log: StableLog::init(get_log_idx_memory(), get_log_data_memory())
                .expect("log initialization failed"),
        }
    }
}

impl IStakingStore for StakingStoreStable {
    fn get_log_entries(&self, account: Account, from: u64, to: u64) -> Vec<StakingLogEntry> {
        let mut result: Vec<StakingLogEntry> = Vec::new();

        let account = BoundedAccount(account);
        let indexes = self
            .log_index
            .range((account.clone(), from.clone())..(account, to.clone()));

        for index in indexes {
            let log_entry = self
                .log
                .get(index.1)
                .expect(&format!("log entry with index {} not found", index.1));

            result.push(log_entry.0);
        }

        result
    }

    fn add_log_entry(&mut self, account: Account, log_entry: &StakingLogEntry) {
        let storable_entry = StakingLogEntryStorable(log_entry.clone());
        let entry_index = self.log.append(&storable_entry).expect("log append failed");

        let account = BoundedAccount(account);
        self.log_index
            .insert((account, log_entry.timestamp.clone()), entry_index);
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

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_bounded_account_storable() {
        let owner = Principal::from_text("aaaaa-aa").unwrap(); // Valid anonymous principal
        let subaccount = Some([42u8; 32]);

        let original = BoundedAccount(Account {
            owner: owner.clone(),
            subaccount,
        });

        let bytes = original.to_bytes();
        assert_eq!(bytes.len(), 100, "Serialized length should be 100 bytes");

        let decoded = BoundedAccount::from_bytes(bytes);
        assert_eq!(decoded.0.owner, owner);
        assert_eq!(decoded.0.subaccount, Some([42u8; 32]));
    }

    #[test]
    fn test_bounded_account_storable_none_subaccount() {
        let owner = Principal::management_canister(); // Another valid principal
        let original = BoundedAccount(Account {
            owner: owner.clone(),
            subaccount: None,
        });

        let bytes = original.to_bytes();
        assert_eq!(bytes.len(), 100);

        let decoded = BoundedAccount::from_bytes(bytes);
        assert_eq!(decoded.0.owner, owner);
        assert_eq!(decoded.0.subaccount, None);
    }
}

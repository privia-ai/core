use abstractions::dao::{DaoClient, DiscountRequest};
use abstractions::{Account, DiscountValue};
use candid::{CandidType, Deserialize, Principal};
use canister_runtime::CdkCallContext;
use ic_cdk::api::msg_caller;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn init(init_args: InitArgs) {
    let caller = msg_caller();
    let config = CanisterConfig {
        owner: caller,
        dao_address: init_args.dao_address,
    };
    let mut storage = ConfigStorage {};
    storage.set_config(config)
}

pub async fn join_dao() {
    let dao = build_dao_service();
    dao.hiving_join().await.unwrap();
}

pub async fn leave_dao() {
    let dao = build_dao_service();
    dao.hiving_leave().await.unwrap()
}

pub fn list_hivers() -> Vec<Account> {
    let storage = HiversStorage::new();
    storage.get_hivers()
}

pub fn join_pool() {
    let caller = msg_caller();
    let account = Account::from(caller);
    let mut storage = HiversStorage::new();

    storage.add_hiver(account);
}

pub fn leave_pool() {
    let caller = msg_caller();
    let account = Account::from(caller);
    let mut storage = HiversStorage::new();

    storage.remove_hiver(&account);
}

pub async fn quote_discounts(price: u128) -> Vec<DiscountQuotePool> {
    let storage = HiversStorage::new();
    let hivers = storage.get_hivers();

    let mut result = Vec::new();

    for hiver in hivers {
        let quote = quote_discount(hiver, price).await;
        let hiver_quote = DiscountQuotePool {
            hiver: hiver.owner,
            discount_value: quote.discount_value,
            price: quote.price,
        };
        result.push(hiver_quote);
    }

    result
}

async fn quote_discount(hiver: Account, product_price: u128) -> DiscountQuote {
    let dao = build_dao_service();

    let discount_value = dao.calculate_max_discount(&hiver, &product_price).await.unwrap();
    let discount_price = calculate_discount_price(product_price, discount_value);

    DiscountQuote {
        discount_value,
        price: discount_price,
    }
}

fn calculate_discount_price(product_price: u128, discount_value: DiscountValue) -> u128 {
    let discounted_money = product_price as f32 * (discount_value as f32 / 100.0);
    let discount_price = (discounted_money * 0.1).round() as u128;
    discount_price
}

pub async fn buy_discount(hiver: Principal, price: u128) -> u128 {
    let dao = build_dao_service();
    let hiver = Account::from(hiver);
    let discount_value = dao.calculate_max_discount(&hiver, &price).await.unwrap();

    let caller = msg_caller();
    let discount_request = DiscountRequest::new(discount_value, Account::from(caller));
    let nft = dao.mint_discount(hiver, discount_request).await.unwrap();

    nft
}

fn get_config() -> CanisterConfig {
    let storage = ConfigStorage {};
    storage.get_config()
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct InitArgs {
    pub dao_address: Principal,
}

fn build_dao_service() -> DaoClient<CdkCallContext> {
    let config = get_config();
    let runtime = CdkCallContext {};
    let client = DaoClient {
        runtime: Rc::new(RefCell::new(runtime)),
        canister_id: config.dao_address,
    };
    client
}

thread_local! {
    static CONFIG_STORAGE: RefCell<Option<CanisterConfig>> = RefCell::new(None);
    static HIVERS_STORAGE: RefCell<HashMap<Account, ()>> = RefCell::new(HashMap::new());
}

struct ConfigStorage {}

impl ConfigStorage {
    pub fn set_config(&mut self, config: CanisterConfig) {
        CONFIG_STORAGE.with(|cell| {
            *cell.borrow_mut() = Some(config);
        });
    }

    pub fn get_config(&self) -> CanisterConfig {
        CONFIG_STORAGE.with(|cell| cell.borrow().clone().expect("ConfigStorage is not initialized"))
    }
}

struct HiversStorage {}

impl HiversStorage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn add_hiver(&mut self, hiver: Account) {
        HIVERS_STORAGE.with(|cell| {
            cell.borrow_mut().insert(hiver, ());
        })
    }

    pub fn remove_hiver(&mut self, hiver: &Account) {
        HIVERS_STORAGE.with(|cell| {
            cell.borrow_mut().remove(hiver);
        })
    }

    pub fn get_hivers(&self) -> Vec<Account> {
        HIVERS_STORAGE.with(|cell| cell.borrow().iter().map(|(account, _)| account.clone()).collect())
    }
}

#[derive(Clone, Debug)]
pub struct CanisterConfig {
    pub owner: Principal,
    pub dao_address: Principal,
}

#[derive(Clone, Debug, Serialize, CandidType)]
pub struct DiscountQuotePool {
    hiver: Principal,
    discount_value: DiscountValue,
    price: u128,
}

pub struct DiscountQuote {
    discount_value: DiscountValue,
    price: u128,
}

use crate::DiscountQuote;
use abstractions::dao::{DaoClient, DiscountRequest};
use abstractions::{Account, DiscountValue};
use candid::{CandidType, Deserialize, Principal};
use canister_runtime::CdkCallContext;
use ic_cdk::api::msg_caller;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
    static CONFIG_STORAGE: RefCell<Option<CanisterConfig>> = RefCell::new(None);
}

#[derive(Clone, Debug, Deserialize, CandidType)]
pub struct InitArgs {
    pub dao_address: Principal,
}

struct ConfigStorage {}

impl ConfigStorage {
    pub fn set_config(&mut self, config: CanisterConfig) {
        CONFIG_STORAGE.with(|cell| {
            *cell.borrow_mut() = Some(config);
        });
    }

    pub fn get_config(&self) -> CanisterConfig {
        CONFIG_STORAGE.with(|cell| {
            cell.borrow()
                .clone()
                .expect("ConfigStorage is not initialized")
        })
    }
}

#[derive(Clone, Debug)]
pub struct CanisterConfig {
    pub owner: Principal,
    pub dao_address: Principal,
}

pub fn init(init_args: InitArgs) {
    let caller = msg_caller();
    let config = CanisterConfig {
        owner: caller,
        dao_address: init_args.dao_address,
    };
    let mut storage = ConfigStorage {};
    storage.set_config(config)
}

fn get_config() -> CanisterConfig {
    let storage = ConfigStorage {};
    storage.get_config()
}

pub async fn join_dao() {
    let dao = build_dao_service();
    dao.borrow().hiving_join().await.unwrap();
}

pub async fn leave_dao() {
    let dao = build_dao_service();
    dao.borrow().hiving_leave().await.unwrap()
}

pub async fn quote_discount(product_price: u128) -> DiscountQuote {
    let dao = build_dao_service();
    let config = get_config();
    let account = Account::from(config.owner);

    let discount_value = dao.borrow().calculate_max_discount(&account, &product_price).await.unwrap();
    let discount_price = calculate_discount_price(product_price, discount_value);

    DiscountQuote {
        discount_value,
        price: discount_price,
    }
}

fn calculate_discount_price(product_price: u128, discount_value: DiscountValue) -> u128 {
    1_000_000
}

pub async fn buy_discount(price: u128) -> u128 {
    let dao = build_dao_service();
    let config = get_config();
    let hiver = Account::from(config.owner);

    let discount_value = dao.borrow().calculate_max_discount(&hiver, &price).await.unwrap();
    let caller = msg_caller();
    let discount_request = DiscountRequest::new(discount_value, Account::from(caller));
    let nft = dao.borrow().mint_discount(hiver, discount_request).await.unwrap();

    nft
}

fn build_dao_service() -> Rc<RefCell<DaoClient<CdkCallContext>>> {
    let config = get_config();
    let runtime = CdkCallContext {};
    let client = DaoClient {
        runtime: Rc::new(RefCell::new(runtime)),
        canister_id: config.dao_address,
    };
    Rc::new(RefCell::new(client))
}

use crate::services;
use crate::services::{DiscountQuotePool, InitArgs};
use abstractions::Account;
use candid::Principal;
use ic_cdk::{init, query, update};

#[init]
fn init(init_args: InitArgs) {
    services::init(init_args);
}

#[update]
async fn join_dao() {
    services::join_dao().await;
}

#[update]
async fn leave_dao() {
    services::leave_dao().await;
}

#[update]
fn join_pool() {
    services::join_pool();
}

#[update]
fn leave_pool() {
    services::leave_pool();
}

#[query]
fn list_hivers() -> Vec<Account> {
    services::list_hivers()
}

#[update]
async fn quote_discounts(price: u128) -> Vec<DiscountQuotePool> {
    services::quote_discounts(price).await
}

#[update]
async fn buy_discount(hiver: Principal, price: u128) -> u128 {
    services::buy_discount(hiver, price).await
}

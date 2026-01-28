use crate::services::InitArgs;
use crate::{services, DiscountQuote};
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
async fn quote_discount(price: u128) -> DiscountQuote {
    services::quote_discount(price).await
}

#[update]
async fn buy_discount(price: u128) -> u128 {
    services::buy_discount(price).await
}

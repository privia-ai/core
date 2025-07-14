#![allow(dead_code)]

#![allow(unused_imports)]
use scenarios::demo_ms2;
use crate::utils::{agent, Actors};

mod smoke;
mod utils;
mod scenarios;

#[allow(unused_variables)]
#[tokio::main]
async fn main() {

    let (token, nft, dao) = agent::build_clients().await;
    let actors = Actors::init();

    demo_ms2::run(&token, &nft, &dao, &actors).await;
    // smoke::token_smoke::check(&token, &actors).await;
}
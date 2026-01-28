#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use scenarios::demo_ms2;
use crate::utils::{agent, Actors};
use config::Config;
use serde::Deserialize;

mod smoke;
mod utils;
mod scenarios;

#[allow(unused_variables)]
#[tokio::main]
async fn main() {
    let settings = read_config();

    let (token, nft, dao) = agent::build_clients(&settings).await;
    let actors = Actors::init(&settings);

    demo_ms2::run(&token, &nft, &dao, &actors).await;
    // smoke::token_smoke::check(&token, &actors).await;
}

fn read_config() -> AppSettings {
    let settings: AppSettings = Config::builder()
        .add_source(config::File::with_name("settings.ic.toml"))
        .build()
        .unwrap()
        .try_deserialize()
        .unwrap();

    settings
}

#[derive(Debug, Deserialize)]
pub (crate) struct AppSettings {
    pub api_url: String,
    pub network: String,
    pub ids_dir: String,
    pub token_minter_name: String
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            api_url: "http://localhost:5949".to_string(),
            network: "local".to_string(),
            ids_dir: "./src/smoke/identities".to_string(),
            token_minter_name: "pr_token_minter".to_string(),
        }
    }
}
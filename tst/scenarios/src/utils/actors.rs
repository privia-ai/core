use std::path::Path;
use candid::Principal;
use ic_agent::Identity;
use ic_agent::identity::Secp256k1Identity;
use icrc_ledger_types::icrc1::account::Account;

pub struct Actors {
    pub pr_token_controller: Actor,
    pub pr_token_minter: Actor,
    pub pr_nft_controller: Actor,
    pub pr_nft_minter: Actor,
    pub wallet1: Actor,
    pub wallet2: Actor,
    pub wallet3: Actor,
    pub wallet4: Actor,
    pub pr_data_buyer: Actor,
}

#[derive(Clone)]
pub struct Actor {
    pub identity: Secp256k1Identity,
    pub principal: Principal,
    pub account: Account,
    pub name: String
}

impl Actor {
    pub fn from_pem_file<P: AsRef<Path>>(file_path: P, name: String) -> Self {
        let identity = Secp256k1Identity::from_pem_file(file_path).unwrap().clone();
        let principal = identity.sender().unwrap();
        let account = Account::from(principal);

        Self {
            identity,
            principal,
            account,
            name
        }
    }
}

const ROOT_DIR: &str = "/home/misha/Code/Custom/privia/internal/";

impl Actors {
    pub fn init() -> Self {
        let ids_dir = Path::new(ROOT_DIR).join("src/smoke/identities/");
        Self {
            pr_token_controller: Actor::from_pem_file(ids_dir.join("pr_token_controller.pem"), "token_controller".to_string()),
            pr_token_minter: Actor::from_pem_file(ids_dir.join("pr_token_minter.pem"), "Token Minter".to_string()),
            pr_nft_controller: Actor::from_pem_file(ids_dir.join("pr_nft_controller.pem"), "nft_controller".to_string()),
            pr_nft_minter: Actor::from_pem_file(ids_dir.join("pr_nft_minter.pem"), "nft_minter".to_string()),
            wallet1: Actor::from_pem_file(ids_dir.join("pr_hiver.pem"), "Wallet 1".to_string()),
            wallet2: Actor::from_pem_file(ids_dir.join("pr_hiver_w1.pem"), "Wallet 2".to_string()),
            wallet3: Actor::from_pem_file(ids_dir.join("pr_hiver_w2.pem"), "Wallet 3".to_string()),
            wallet4: Actor::from_pem_file(ids_dir.join("pr_hiver_w3.pem"), "Wallet 4".to_string()),
            pr_data_buyer: Actor::from_pem_file(ids_dir.join("pr_data_buyer.pem"), "data_buyer".to_string()),
        }
    }
}
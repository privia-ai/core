use abstractions::{
    dao::DaoClient,
    nft::NftClient,
    runtime::{CallMode, ICallContext},
    token::TokenClient
};
use async_trait::async_trait;
use candid::{CandidType, Decode, Deserialize, Principal};
use ic_agent::{
    identity::Secp256k1Identity,
    Agent,
    AgentError
};
use std::{
    cell::RefCell,
    process::Command,
    rc::Rc
};

pub struct AgentCallContext {
    pub agent: Agent,
}

impl AgentCallContext {
    pub async fn build<S: Into<String>>(url: S) -> Self {
        let agent = Agent::builder()
            .with_url(url)
            .build()
            .expect("can't create agent");
        agent.fetch_root_key().await.unwrap();

        AgentCallContext { agent }
    }

    fn set_identity(&mut self, actor: Secp256k1Identity) {
        self.agent.set_identity(actor);
    }
}

#[async_trait]
impl ICallContext for AgentCallContext {
    type Error = AgentError;

    async fn call<'a, Out>(
        &self,
        canister_id: Principal,
        mode: CallMode,
        method: &str,
        args: &'a [u8],
    ) -> Result<Out, Self::Error>
    where
        Out: CandidType + for<'de> Deserialize<'de>,
    {
        let response = match mode {
            CallMode::Query => self.agent.query(&canister_id, method).with_arg(args).await,
            CallMode::Update => self.agent.update(&canister_id, method).with_arg(args).await,
        }?;

        let result = Decode!(response.as_slice(), Out).map_err(|e| Self::Error::from(e))?;
        Ok(result)
    }
}

const DFX_URL: &str = "http://localhost:4949";

pub async fn build_clients() -> (
    TokenClient<AgentCallContext>,
    NftClient<AgentCallContext>,
    DaoClient<AgentCallContext>,
) {
    let token_client = build_token_client().await;
    let nft_client = build_nft_client().await;
    let dao_client = build_dao_client().await;

    (token_client, nft_client, dao_client)
}

pub async fn build_token_client() -> TokenClient<AgentCallContext> {
    let agent_runtime = AgentCallContext::build(DFX_URL).await;
    let agent_runtime = Rc::new(RefCell::new(agent_runtime));

    let token_canister_id = get_canister_id("token").unwrap();
    let token_client = TokenClient {
        runtime: agent_runtime,
        canister_id: token_canister_id,
    };

    token_client
}

pub async fn build_nft_client() -> NftClient<AgentCallContext> {
    let agent_runtime = AgentCallContext::build(DFX_URL).await;
    let agent_runtime = Rc::new(RefCell::new(agent_runtime));

    let nft_canister_id = get_canister_id("nft").unwrap();
    let nft_client = NftClient {
        runtime: agent_runtime,
        canister_id: nft_canister_id,
    };

    nft_client
}

pub async fn build_dao_client() -> DaoClient<AgentCallContext> {
    let agent_runtime = AgentCallContext::build(DFX_URL).await;
    let agent_runtime = Rc::new(RefCell::new(agent_runtime));

    let dao_canister_id = get_canister_id("dao").unwrap();
    let dao_client = DaoClient {
        runtime: agent_runtime,
        canister_id: dao_canister_id,
    };

    dao_client
}

fn get_canister_id(canister_name: &str) -> Option<Principal> {
    let output = Command::new("dfx")
        .args(["canister", "id", canister_name])
        .output()
        .ok()?;

    match output.status.success() {
        true => {
            let str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let principal = Principal::from_text(str).unwrap();
            Some(principal)
        }
        false => None,
    }
}

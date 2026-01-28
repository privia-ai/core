use crate::utils::helpers::*;
use crate::utils::{Actor, Actors, AgentCallContext};
use abstractions::dao::{Cycle, DaoClient, Discount, Proposal, ProposalType, VoteOption};
use abstractions::nft::NftClient;
use abstractions::token::TokenClient;
use chrono::TimeDelta;
use ic_agent::AgentError;
use icrc_ledger_types::icrc1::account::Account;
use icrc_ledger_types::icrc1::transfer::{NumTokens, TransferArg};
use std::ops::Add;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;

fn build_transfer_req(to: Account, amount: u64) -> TransferArg {
    TransferArg {
        to,
        amount: NumTokens::from(amount),
        from_subaccount: None,
        fee: None,
        created_at_time: None,
        memo: None,
    }
}

pub async fn run<'a>(
    token: &'a TokenClient<AgentCallContext>,
    nft: &'a NftClient<AgentCallContext>,
    dao: &'a DaoClient<AgentCallContext>,
    actors: &'a Actors,
) {
    clear_console();

    let demo_runner = DemoMS2::new(&token, &nft, &dao, &actors);
    demo_runner.run().await;
}

pub struct DemoMS2<'a> {
    token: &'a TokenClient<AgentCallContext>,
    nft: &'a NftClient<AgentCallContext>,
    dao: &'a DaoClient<AgentCallContext>,
    actors: &'a Actors,
}

impl<'a> DemoMS2<'a> {
    pub async fn run(&self) {
        let w1 = &self.actors.wallet1;
        let w2 = &self.actors.wallet2;
        let w3 = &self.actors.wallet3;
        let w4 = &self.actors.wallet4;
        let big_spenda = &self.actors.pr_token_minter;

        self.change_actor(big_spenda);

        println!("Initial state:\n");

        // Current Cycle = x1
        self.print_current_cycle().await;
        // Wallet 1 = 0 PVT (Staking Score = 0)
        self.print_balance_and_score(w1).await;
        // Wallet 2 = 0 PVT (Staking Score = 0)
        self.print_balance_and_score(w2).await;
        // Wallet 3 = 0 PVT (Staking Score = 0)
        self.print_balance_and_score(w3).await;

        println!();

        println!("Transferring tokens:\n  100 PVT => Wallet 1\n  1000 PVT => Wallet 2\n");

        // Do a transfer to Wallet 1 and Wallet 2
        self.mint_to(w1, 100).await;
        self.mint_to(w2, 1000).await;

        // Current Cycle = x2
        let cycle = self.print_current_cycle().await;
        // Wallet 1 = 100 PVT (Staking Score = a1)
        self.print_balance_and_score(w1).await;
        // Wallet 2 = 1000 PVT (Staking Score = b1)
        self.print_balance_and_score(w2).await;
        // Wallet 3 = 0 PVT (Staking Score = c1=0)
        self.print_balance_and_score(w3).await;

        // Wait until the end of the current cycle
        Self::sleep_until_cycle_end(&cycle).await;

        println!("Transferring tokens:\n  100 PVT => Wallet 3\n");
        // then do a transfer to Wallet 3
        self.mint_to(w3, 100).await;
        // Current Cycle = x3
        let cycle = self.print_current_cycle().await;
        // Wallet 1 = 100 PVT (Staking Score = a2)
        self.print_balance_and_score(w1).await;
        // Wallet 2 = 1000 PVT (Staking Score = b2)
        self.print_balance_and_score(w2).await;
        // Wallet 3 = 100 PVT (Staking Score = c2)
        self.print_balance_and_score(w3).await;

        // Wait until the end of the current cycle
        Self::sleep_until_cycle_end(&cycle).await;

        // Current Cycle = x4
        self.print_current_cycle().await;
        // Wallet 1 = 100 PVT (Staking Score = a3)
        self.print_balance_and_score(w1).await;
        // Wallet 2 = 1000 PVT (Staking Score = b3)
        self.print_balance_and_score(w2).await;
        // Wallet 3 = 100 PVT (Staking Score = c3)
        self.print_balance_and_score(w3).await;

        self.change_actor(w2);

        // Price = 15’000
        let price: u128 = 15000;

        println!("Calculating discounts. Price = {price}\n");

        // Current Cycle = x5
        let cycle = self.print_current_cycle().await;

        // Wallet 1 = 100 PVT (Staking Score = a4, Discount = da1)
        self.print_balance_score_discount(w1, price).await;
        // Wallet 2 = 1000 PVT (Staking Score = b4, Discount = db1)
        self.print_balance_score_discount(w2, price).await;
        // Wallet 3 = 100 PVT (Staking Score = c4, Discount = dc1)
        self.print_balance_score_discount(w3, price).await;

        // Wait until the end of the current cycle
        Self::sleep_until_cycle_end(&cycle).await;

        // Current Cycle = x6
        self.print_current_cycle().await;

        // Wallet 1 = 100 PVT (Staking Score = a4, Discount = da1)
        self.print_balance_score_discount(w1, price).await;
        // Wallet 2 = 1000 PVT (Staking Score = b4, Discount = db1)
        self.print_balance_score_discount(w2, price).await;
        // Wallet 3 = 100 PVT (Staking Score = c4, Discount = dc1)
        self.print_balance_score_discount(w3, price).await;

        // Pause
        press_enter(None);

        println!("Creating proposal...\n");

        let proposal_text = Self::get_proposal_text();
        let proposal = self.create_and_show_proposal(proposal_text, w2).await;

        // Voting (Wallet 1) = eligible
        let vote_result = self.vote_on_proposal(proposal.id, w1).await;
        if vote_result.is_err() {
            if let Err(e) = vote_result
                && matches!(e, VotingError::ProposalNotActive)
            {
                println!("Wait for the voting start");
                sleep_until(&nanos_to_localtime(proposal.start)).await;
            }
        }

        // Voting (Wallet 1) = eligible
        _ = self.vote_on_proposal(proposal.id, w1).await;

        // Voting (Wallet 4) = not eligible
        _ = self.vote_on_proposal(proposal.id, w4).await;

        println!();
        press_enter(Some("Press ENTER to exit..."));
    }

    fn get_proposal_text() -> String {
        r#"Title: Upgrade Staking Rewards Policy

Summary:
We propose to update the staking rewards policy to better incentivize long-term participation in the DAO.

Details:
The current staking rewards mechanism provides a flat rate regardless of the staking duration. This proposal aims to introduce a tiered reward structure based on how long the tokens are locked:

- 1–3 months: 3%
- 3–6 months: 5%
- 6–12 months: 7%
- 12+ months: 10%

The goal is to encourage more long-term commitment from token holders, which will contribute to the DAO's stability and governance continuity.

Impact:
This change would require an update to the staking smart contract logic. Once approved, the new policy will take effect in the next governance cycle."#
            .to_string()
    }

    async fn vote_on_proposal(&self, proposal_id: u64, actor: &Actor) -> Result<(), VotingError> {
        self.change_actor(actor);
        let result = self.dao.voting_vote(proposal_id, VoteOption::Decline).await;
        console_log(format!("{} voted", actor.name));
        self.process_vote_result(result).await
    }

    async fn create_and_show_proposal(&self, text: String, actor: &Actor) -> Proposal {
        self.change_actor(actor);

        let proposal_id = self
            .dao
            .voting_create_proposal(ProposalType::Generic, text)
            .await
            .unwrap();

        let proposal = self
            .dao
            .voting_get_proposal(proposal_id)
            .await
            .unwrap()
            .unwrap();

        console_log(format!("Proposal created: {}", proposal));

        proposal
    }

    async fn sleep_until_cycle_end(cycle: &Cycle) {
        let end_local_time = nanos_to_localtime(cycle.end).add(TimeDelta::seconds(1));
        sleep_until(&end_local_time).await;
    }

    pub fn new(
        token: &'a TokenClient<AgentCallContext>,
        nft: &'a NftClient<AgentCallContext>,
        dao: &'a DaoClient<AgentCallContext>,
        actors: &'a Actors,
    ) -> Self {
        Self {
            token,
            dao,
            nft,
            actors,
        }
    }

    async fn print_balance_and_score(&self, actor: &Actor) {
        let balance = self.token.balance_of(actor.account).await.unwrap();
        let score = self.dao.get_staking_score(actor.account).await.unwrap();
        println!(
            "  Wallet {} = {} PVT (Staking score = {})",
            actor.name, balance, score
        );
    }

    async fn print_balance_score_discount(&self, actor: &Actor, price: u128) {
        let balance = self.token.balance_of(actor.account).await.unwrap();
        let score = self.dao.get_staking_score(actor.account).await.unwrap();
        let discount = self
            .dao
            .calculate_max_discount(&actor.account, &price)
            .await
            .unwrap();
        println!(
            "  Wallet {} = {} PVT (Staking Score = {}, Discount = {}%)",
            actor.name, balance, score, discount
        );
    }

    async fn print_current_cycle(&self) -> Cycle {
        let current_cycle = self.dao.get_current_cycle().await.unwrap();
        console_log(format!("Current cycle: {}", current_cycle));
        current_cycle
    }

    async fn mint_to(&self, actor: &Actor, amount: u64) {
        _ = self
            .token
            .transfer(build_transfer_req(actor.account, amount))
            .await
            .unwrap();
    }

    async fn process_vote_result(
        &self,
        vote_result: Result<u64, AgentError>,
    ) -> Result<(), VotingError> {
        {
            match vote_result {
                Ok(id) => {
                    console_log(format!("Vote accepted, vote_id: {id}",));
                    let vote = self.dao.voting_get_vote(id).await.unwrap();
                    console_log(format!("{}", vote.unwrap()));
                    Ok(())
                }
                Err(e) => match e {
                    AgentError::CertifiedReject {
                        reject,
                        operation: _operation,
                    } => {
                        console_log("Vote rejected".to_string());
                        pretty_print_reject(&reject.reject_message);

                        if reject.reject_message.contains("not active") {
                            Err(VotingError::ProposalNotActive)
                        } else if reject.reject_message.contains("balance is zero") {
                            Err(VotingError::ZeroBalnce)
                        } else {
                            Err(VotingError::Generic)
                        }
                    }
                    _ => {
                        console_log(format!("Vote rejected, error: {e}"));
                        Err(VotingError::Generic)
                    }
                },
            }
        }
    }

    fn change_actor(&self, actor: &Actor) {
        self.token
            .runtime
            .borrow_mut()
            .agent
            .set_identity(actor.identity.clone());
        self.dao
            .runtime
            .borrow_mut()
            .agent
            .set_identity(actor.identity.clone());
        self.nft
            .runtime
            .borrow_mut()
            .agent
            .set_identity(actor.identity.clone());
    }
}

enum VotingError {
    ZeroBalnce,
    ProposalNotActive,
    Generic,
}

use abstractions::dao::{DaoClient, Discount, DiscountRequest};
use crate::utils::{Actors, AgentCallContext};

pub async fn check(dao: &DaoClient<AgentCallContext>, actors: &Actors) {

    let _proposer = actors.wallet1.account;
    let _voter_wtih_tokens =actors.wallet2.account;
    let _voter_wtihout_tokens =actors.wallet3.account;

    let discounter = actors.wallet4.account;

    let _the_account = actors.wallet1.account;
    let _accs = Vec::from([actors.wallet1.account, actors.wallet2.account]);
    let _token_ids = Vec::from([1,2,3]);
    let _prev = Some(0);
    let _take = Some(10);

    // let proposal_type = ProposalType::Generic;
    // let data = "Proposal data ...".to_string();
    // let proposal_id = dao.voting_create_proposal(proposal_type, data).await.unwrap();
    // println!("voting_create_proposal: {}", proposal_id);
    //
    // let res = dao.voting_get_proposal(proposal_id).await.unwrap();
    // println!("voting_get_proposal: {:?}", res);
    //
    // let vote = VoteOption::Approve;
    // let vote_id = dao.voting_vote(proposal_id, vote).await.unwrap();
    // println!("voting_vote: {}", vote_id);
    //
    // let res = dao.voting_get_vote(vote_id).await.unwrap();
    // println!("voting_get_vote: {:?}", res);
    //
    // let res = dao.voting_get_all_votes(proposal_id).await.unwrap();
    // println!("voting_get_all_votes: {:?}", res);

    let res = dao.get_current_cycle().await.unwrap();
    println!("get_current_cycle: {}", res);

    let res = dao.get_staking_score(discounter).await.unwrap();
    println!("get_staking_score: {}", res);

    let price = 1500u128;
    let discount_value = dao.calculate_max_discount(&discounter, &price).await.unwrap();
    println!("calculate_max_discount: {}", discount_value);

    let discount = DiscountRequest {
        value: discount_value,
        owner: discounter
    };
    let token_id = dao.mint_discount(discounter, discount).await.unwrap();
    println!("mint_discount: {}", token_id);

}







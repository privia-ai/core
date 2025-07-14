use crate::utils::{Actors, AgentCallContext};
use abstractions::token::TokenClient;
use candid::Nat;
use icrc_ledger_types::icrc1::transfer::TransferArg;

fn nat_zero() -> Nat {
    Nat::from(0u8)
}

pub async fn check(token: &TokenClient<AgentCallContext>, actors: &Actors) {
    let res = token.decimals().await.unwrap();
    println!("decimals: {}", res);

    let res = token.name().await.unwrap();
    println!("name: {}", res);

    let res = token.metadata().await.unwrap();
    println!("metadata: {}", res.len());

    let res = token.symbol().await.unwrap();
    println!("symbol: {}", res);

    let res = token.fee().await.unwrap();
    println!("fee: {}", res);

    let res = token.minting_account().await.unwrap();
    println!(
        "minting_account: {}",
        res.map_or("not found".to_string(), |a| a.to_string())
    );

    let res = token
        .privia_staking_log(actors.wallet1.account, None, None)
        .await
        .unwrap();
    assert_eq!(0, res.log.len());

    let res = token.balance_of(actors.wallet1.account).await.unwrap();
    assert_eq!(nat_zero(), res);

    let res = token.total_supply().await.unwrap();
    assert_eq!(nat_zero(), res);

    let amount = Nat::from(100u32);
    let mut total_amount = amount.clone();

    token.runtime.borrow_mut().agent.set_identity(actors.pr_token_minter.identity.clone());

    let req = TransferArg {
        to: actors.wallet1.account,
        amount: amount.clone(),
        from_subaccount: None,
        fee: None,
        created_at_time: None,
        memo: None,
    };
    token.transfer(req).await.unwrap().unwrap();

    let res = token
        .privia_staking_log(actors.wallet1.account, None, None)
        .await
        .unwrap();
    assert_eq!(1, res.log.len());

    let res = token.balance_of(actors.wallet1.account).await.unwrap();
    assert_eq!(amount, res);

    let res = token.total_supply().await.unwrap();
    assert_eq!(total_amount, res);

    let req = TransferArg {
        to: actors.wallet1.account,
        amount: amount.clone(),
        from_subaccount: None,
        fee: None,
        created_at_time: None,
        memo: None,
    };
    token.transfer(req).await.unwrap().unwrap();
    total_amount += amount.clone();

    let res = token
        .privia_staking_log(actors.wallet1.account, None, None)
        .await
        .unwrap();
    assert_eq!(2, res.log.len());

    let res = token.balance_of(actors.wallet1.account).await.unwrap();
    assert_eq!(amount*Nat::from(2u8), res);

    let res = token.total_supply().await.unwrap();
    assert_eq!(total_amount, res);
}

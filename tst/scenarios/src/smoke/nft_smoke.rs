use candid::Nat;
use icrc_ledger_types::icrc::generic_metadata_value::MetadataValue;
use abstractions::nft::{NftClient, TransferArg};
use crate::utils::{Actors, AgentCallContext};

pub async fn check(nft: &NftClient<AgentCallContext>, actors: &Actors) {

    let the_account = actors.wallet1.account;
    let accs = Vec::from([actors.wallet1.account, actors.wallet2.account]);
    let token_ids = Vec::from([1,2,3]);
    let prev = Some(0);
    let take = Some(10);

    let res = nft.icrc7_total_supply().await.unwrap();
    println!("icrc7_total_supply: {}", res);

    let res = nft.icrc7_balance_of(accs).await.unwrap();
    println!("icrc7_balance_of: {}", res.len());

    let res = nft.icrc7_tokens(prev, take).await.unwrap();
    println!("icrc7_tokens: {}", res.len());

    let res = nft.icrc7_tokens_of(the_account, prev, take).await.unwrap();
    println!("icrc7_tokens_of: {}", res.len());

    let res = nft.icrc7_owner_of(token_ids).await.unwrap();
    println!("icrc7_owner_of: {}", res.len());

    let res = nft.icrc7_collection_metadata().await.unwrap();
    println!("icrc7_collection_metadata: {}", res.len());

    let res = nft.icrc7_symbol().await.unwrap();
    println!("icrc7_symbol: {}", res);

    let res = nft.icrc7_name().await.unwrap();
    println!("icrc7_name: {}", res);

    let res = nft.icrc7_description().await.unwrap();
    println!("icrc7_description: {}", res.unwrap_or("N/A".into()));

    let res = nft.icrc7_logo().await.unwrap();
    println!("icrc7_logo: {}", res.unwrap_or("N/A".into()));

    let res = nft.icrc7_supply_cap().await.unwrap();
    println!("icrc7_supply_cap: {}", res.unwrap_or(555));

    let res = nft.icrc7_max_query_batch_size().await.unwrap();
    println!("icrc7_max_query_batch_size: {}", res.unwrap_or(555));

    let res = nft.icrc7_max_update_batch_size().await.unwrap();
    println!("icrc7_max_update_batch_size: {}", res.unwrap_or(555));

    let res = nft.icrc7_default_take_value().await.unwrap();
    println!("icrc7_default_take_value: {}", res.unwrap_or(555));

    let res = nft.icrc7_max_take_value().await.unwrap();
    println!("icrc7_max_take_value: {}", res.unwrap_or(555));

    let res = nft.icrc7_max_memo_size().await.unwrap();
    println!("icrc7_max_memo_size: {}", res.unwrap_or(555));

    let res = nft.icrc7_atomic_batch_transfers().await.unwrap();
    println!("icrc7_atomic_batch_transfers: {}", res.unwrap_or(true));

    let res = nft.icrc7_tx_window().await.unwrap();
    println!("icrc7_tx_window: {}", res.unwrap_or(555));

    let res = nft.icrc7_permitted_drift().await.unwrap();
    println!("icrc7_permitted_drift: {}", res.unwrap_or(555));

    let res = nft.icrc7_supported_standards().await.unwrap();
    println!("icrc7_supported_standards: {}", res.len());

    nft.runtime.borrow_mut().agent.set_identity(actors.pr_nft_minter.identity.clone());
    let owner = the_account;
    let metadata = Vec::from([
        ("value".to_string(), MetadataValue::Nat(Nat::from(15u8))),
        ("max_price".to_string(), MetadataValue::Nat(Nat::from(15u8)))
    ]);
    let res = nft.privia_mint_token(owner, metadata).await.unwrap();
    println!("privia_mint_token: {}", res);

    let token_id  = Nat::from(res);
    let ids = Vec::from([res]);
    let res = nft.icrc7_token_metadata(ids).await.unwrap();
    println!("icrc7_token_metadata: {}", res.len());
    for opt in res {
        if let Some(md) = opt {
            print!("  metadata");
            for (k,v) in md {
                println!("    - {}: {:?}", k, v);
            }
        }
        else
        {
            println!("  empty");
        }
    }

    println!("before transfrer: icrc7_transfer");
    let arg = TransferArg {
        to: actors.wallet2.account,
        token_id: token_id,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
    };
    let req = Vec::from([arg]);
    nft.runtime.borrow_mut().agent.set_identity(actors.wallet1.identity.clone());
    let res = nft.icrc7_transfer(req).await.unwrap();
    println!("icrc7_transfer: {}", res.len());

}

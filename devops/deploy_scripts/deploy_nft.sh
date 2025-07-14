#!/bin/bash

TOKEN_MINTER=$(dfx canister id --ic dao)

dfx deploy nft --ic --argument="(
  record {
    minting_account = record {
      owner = principal \"${TOKEN_MINTER}\";
    };
    icrc7_symbol = \"PRDDSC\";
    icrc7_name = \"Privia discount Collection\";
    icrc7_description = opt \"An awesome NFT collection of privia discounts!\";
    icrc7_logo = opt \"https://coreledger.com\";
    icrc7_supply_cap = opt 10000;
    icrc7_max_query_batch_size = opt 20;
    icrc7_max_update_batch_size = opt 10;
    icrc7_max_take_value = opt 100;
    icrc7_default_take_value = opt 10;
    icrc7_max_memo_size = opt 512;
    icrc7_atomic_batch_transfers = opt true;
    tx_window = opt 60000000000;  // 60 seconds in nanoseconds
    permitted_drift = opt 5000000000; // 5 seconds in nanoseconds
  }
)"
#!/bin/bash

TOKEN_MINTER=$(dfx --identity pr_token_minter identity get-principal)

dfx deploy token --ic --argument "(
  record {
    token_symbol = \"PVD\";
    token_name = \"Privia token dev\";
    minting_account = record {
        owner = principal \"${TOKEN_MINTER}\";
    };
    transfer_fee = 0;
    metadata = vec {};
    decimals = opt 0;
  }
)"
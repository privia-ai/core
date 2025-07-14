#!/bin/bash

TOKEN_CANISTER=$(dfx canister id --ic token)
NFT_CANISTER=$(dfx canister id --ic nft)
GENESIS=$(date +%s)000000000 # now
CYCLE_LEN_SEC=864000 # 10 days

dfx deploy dao \
  --ic \
  --argument "(
    record {
      staking = record {};
      cycles = record {
        hiving_cycles = 4;
        voting_cycles = 4;
        genesis = opt ${GENESIS};
        cycle_len_ns = ${CYCLE_LEN_SEC}_000_000_000;
      };
      discounts = record {
        discounts_per_cycle = 5;
      };
      token_canister_id = principal \"${TOKEN_CANISTER}\";
      nft_canister_id = principal \"${NFT_CANISTER}\";
    }
  )"

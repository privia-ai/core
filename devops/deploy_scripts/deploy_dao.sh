#!/bin/bash

NETWORK="${1}"

TOKEN_CANISTER=$(dfx canister id --network "$NETWORK" token)
NFT_CANISTER=$(dfx canister id --network "$NETWORK" nft)
GENESIS=$(date +%s)000000000 # now

#CYCLE_LEN_SEC=864000 # 10 days
CYCLE_LEN_SEC=$((60*60)) # 60 minutes

ARGUMENT="(
  record {
    staking = record {};
    cycles = record {
      hiving_cycles = 4;
      voting_cycles = 4;
      genesis = opt (${GENESIS} : nat64);
      cycle_len_ns = (${CYCLE_LEN_SEC}_000_000_000 : nat64);
    };
    discounts = record {
      discounts_per_cycle = 5;
    };
    token_canister_id = principal \"${TOKEN_CANISTER}\";
    nft_canister_id = principal \"${NFT_CANISTER}\";
  }
)"

echo "$ARGUMENT"

#dfx deploy dao \
#  --network "$NETWORK" \
#  --argument "$ARGUMENT"

dfx deploy dao \
  --mode reinstall \
  --network "$NETWORK" \
  --argument "$ARGUMENT"

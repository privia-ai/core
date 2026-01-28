#!/bin/bash

NETWORK="${1}"
#OWNER="pr_token_minter"
OWNER="pvd_owner"

#TOKEN_MINTER=$(dfx --identity "$OWNER" identity get-principal)
TOKEN_MINTER="st66q-cb5xl-pbpfb-5xlgj-db42k-cycst-f6ybj-yie7x-hkmut-4fmhq-rqe" # token minter

ARGUMENT="(
  record {
    token_symbol = \"PVD\";
    token_name = \"Privia token dev\";
    minting_account = record {
        owner = principal \"${TOKEN_MINTER}\";
    };
    transfer_fee = 0;
    metadata = vec {};
    decimals = opt (0 : nat8);
  }
)"
echo "$ARGUMENT"

#dfx deploy token \
#  --network "$NETWORK" \
#  --argument "$ARGUMENT"

dfx deploy token \
  --mode reinstall \
  --network "$NETWORK" \
  --argument "$ARGUMENT"
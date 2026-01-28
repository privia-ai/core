#!/bin/bash

NETWORK="${1}"

DAO_CANISTER=$(dfx canister id --network "$NETWORK" dao)

ARGUMENT="(
  record {
    dao_address = principal \"${DAO_CANISTER}\";
  }
)"

echo "$ARGUMENT"

#dfx deploy hiving \
#  --network "$NETWORK" \
#  --argument "$ARGUMENT"

dfx deploy hiving \
  --mode reinstall \
  --network "$NETWORK" \
  --argument "$ARGUMENT"

#!/bin/bash

# Get current script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

NETWORK="${1:-local}"

dfx canister create --all --network "$NETWORK"
dfx build --all --network "$NETWORK"

bash "$SCRIPT_DIR/deploy_token.sh" "$NETWORK"
bash "$SCRIPT_DIR/deploy_nft.sh" "$NETWORK"
bash "$SCRIPT_DIR/deploy_dao.sh" "$NETWORK"
bash "$SCRIPT_DIR/deploy_hiving.sh" "$NETWORK"
bash "$SCRIPT_DIR/deploy_hiving_pool.sh" "$NETWORK"

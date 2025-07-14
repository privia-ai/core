#!/bin/bash

# Get current script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

bash "$SCRIPT_DIR/deploy_token.sh"

bash "$SCRIPT_DIR/deploy_nft.sh"

bash "$SCRIPT_DIR/deploy_dao.sh"

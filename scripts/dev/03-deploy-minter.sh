#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-unknown-unknown/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/dev" ]; then
  cd ../..
fi

. .env

if [[ "$(dfx canister minter 2>&1 | grep Error)" != "" ]]; then
  dfx canister create minter
fi

ADMIN_PRINCIPAL=$(dfx identity get-principal)
COIN_CANISTER_ID=$(dfx canister id coin)
NFT_COL_CANISTER_ID=$(dfx canister id nft)

TEAM_FEE=10000000 # 10% of the block reward in e8s
TREASURY_FEE=20000000 # 20% of the block reward in e8s
BLOCK_REWARD_TIERS="vec {50000000; 30000000; 20000000}" # 50%, 30%, 20% in e8s (after fees)

dfx deploy minter -v --identity default --with-cycles 10000000000000 --argument "(
    record {
      config = record {
        administrator = principal \"$ADMIN_PRINCIPAL\";
        team_fee = $TEAM_FEE;
        treasury_fee = $TREASURY_FEE;
        block_reward_tiers = $BLOCK_REWARD_TIERS;
        meme_nft = record { 
          ty = variant {Icrc7};
          canister_id = principal \"$NFT_COL_CANISTER_ID\";
        };
        meme_coin = record {
          ty = variant {Icrc1};
          canister_id = principal \"$COIN_CANISTER_ID\";
        };
      };
    }
)"

####
BOT_CANISTER_ID=$(dfx canister id bot)
dfx canister update-settings minter --add-controller $BOT_CANISTER_ID

popd
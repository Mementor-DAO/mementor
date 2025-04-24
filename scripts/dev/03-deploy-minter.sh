#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-unknown-unknown/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/dev" ]; then
  cd ../..
fi

. .env

dfx canister create minter >/dev/null

ADMIN_PRINCIPAL=$(dfx identity get-principal)
COIN_CANISTER_ID=$(dfx canister id coin)
NFT_COL_CANISTER_ID=$(dfx canister id nft)

dfx deploy minter -v --identity default --with-cycles 10000000000000 --argument "(
    record {
      config = record {
        administrator = principal \"$ADMIN_PRINCIPAL\";
        team_fee     = 10000000;
        treasury_fee = 20000000;
        max_owners_raffle = 5;
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
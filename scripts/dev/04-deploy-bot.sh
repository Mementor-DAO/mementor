#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-wasip1/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/dev" ]; then
  cd ../..
fi

. .env

ASSETS_DATA_DIR=./packages/assets/data

dfx canister create bot >/dev/null

ADMIN_PRINCIPAL=$(dfx identity get-principal)
BOT_CANISTER_ID=$(dfx canister id bot)
COIN_CANISTER_ID=$(dfx canister id coin)
MINTER_CANISTER_ID=$(dfx canister id minter)
NFT_COL_CANISTER_ID=$(dfx canister id nft)

NFT_MAX_SUPPLY=10000
NFT_COL_URL_TEMPLATE="http://$BOT_CANISTER_ID.raw.localhost:8080/assets/nfts/{}.jpg"
NFT_MIN_CHAT_MEMBERS=0 # chat where the meme was posted must have at least n members to a NFT be minted
NFT_MIN_USER_CREATION_INTERVAL=$((0*15*24*60*60*1000)) # a reaction only counts if the user reacting was created at least n days ago
NFT_MIN_NUM_REACTIONS=0
NFT_MAX_NUM_REACTIONS=10
NFT_MIN_MINTING_COST=10000000 # 0.1 ICP in e8s
NFT_MAX_MINTING_COST=10000000000 # 100.0 ICP in e8s
NFT_TEAM_FEE=10000000 # 10% in e8s

INDEX_TAR=$(od -t x1 -v -w1048576 -A n $ASSETS_DATA_DIR/index.tar | sed "s/ /\\\/g")
MEMES_JSON=$(od -t x1 -v -w1048576 -A n $ASSETS_DATA_DIR/memes.json | sed "s/ /\\\/g")

dfx deploy bot -v --identity default --with-cycles 10000000000000 --argument-file <(echo "(
    record {
      oc_public_key = \"$OC_PUBLIC_KEY\";
      administrator = principal \"$ADMIN_PRINCIPAL\";
      memes_json_bytes = blob \"$MEMES_JSON\";
      index_tar_bytes = blob \"$INDEX_TAR\"; 
      meme_coin = record {
        ty = variant {Icrc1};
        canister_id = principal \"$COIN_CANISTER_ID\";
      };
      meme_coin_config = record {
        minter_canister_id = principal \"$MINTER_CANISTER_ID\";
      };
      meme_nft = record { 
        ty = variant {Icrc7};
        max_supply = $NFT_MAX_SUPPLY;
        canister_id = principal \"$NFT_COL_CANISTER_ID\";
        url_template = \"$NFT_COL_URL_TEMPLATE\";
        min_reactions = 2;
      };
      meme_nft_config = record { 
        min_num_reactions = $NFT_MIN_NUM_REACTIONS;
        max_num_reactions = $NFT_MAX_NUM_REACTIONS;
        min_minting_cost = $NFT_MIN_MINTING_COST;
        max_minting_cost = $NFT_MAX_MINTING_COST;
        min_chat_members = $NFT_MIN_CHAT_MEMBERS;
        min_user_creation_interval = $NFT_MIN_USER_CREATION_INTERVAL;
        team_fee_p = $NFT_TEAM_FEE;
      };
    }
)")

popd
#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-wasip1/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/prod" ]; then
  cd ../..
fi

. .env

ASSETS_DATA_DIR=./packages/assets/data

dfx canister create bot --ic --subnet $SUBNET --identity deployer >/dev/null

ADMIN_PRINCIPAL=$(dfx identity get-principal --identity deployer)
BOT_CANISTER_ID=$(dfx canister id bot --ic)
COIN_CANISTER_ID=$(dfx canister id coin --ic)
MINTER_CANISTER_ID=$(dfx canister id minter --ic)
NFT_COL_CANISTER_ID=$(dfx canister id nft --ic)

NFT_MAX_SUPPLY=10000
NFT_COL_URL_TEMPLATE="http://$BOT_CANISTER_ID.raw.icp0.io/assets/nfts/{}.jpg"
NFT_MIN_CHAT_MEMBERS=50 # chat where the meme was posted must have at least n members to a NFT be minted
NFT_MIN_USER_CREATION_INTERVAL=$((15*24*60*60*1000)) # a reaction only counts if the user reacting was created at least n days ago
NFT_MIN_NUM_REACTIONS=1
NFT_MAX_NUM_REACTIONS=10
NFT_MIN_MINTING_COST=100000000 # 1.0 ICP in e8s
NFT_MAX_MINTING_COST=10000000000 # 100.0 ICP in e8s
NFT_TEAM_FEE=10000000 # 10% in e8s. 90% of the minting cost goes to treasury

INDEX_GZ=$(od -t x1 -v -w1048576 -A n $ASSETS_DATA_DIR/index.gz | sed "s/ /\\\/g")
MEMES_GZ=$(od -t x1 -v -w1048576 -A n $ASSETS_DATA_DIR/memes.gz | sed "s/ /\\\/g")

dfx deploy bot -v --ic --identity deployer --with-cycles 10000000000000 --argument-file <(echo "(
    record {
      oc_public_key = \"$OC_PUBLIC_KEY_PROD\";
      administrator = principal \"$ADMIN_PRINCIPAL\";
      memes_json_gz = blob \"$MEMES_GZ\";
      index_tar_gz = blob \"$INDEX_GZ\"; 
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
#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-wasip1/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/dev" ]; then
  cd ../..
fi

. .env

if [[ "$(dfx canister nft 2>&1 | grep Error)" != "" ]]; then
  dfx canister create nft
fi
if [[ "$(dfx canister bot 2>&1 | grep Error)" != "" ]]; then
  dfx canister create bot
fi

BOT_CANISTER_ID=$(dfx canister id bot)
LOGO_URL="http://$BOT_CANISTER_ID.raw.localhost:8080/assets/nft_logo.png"
MAX_SUPPLY=10000

dfx deploy nft -v --identity default --with-cycles 10000000000000 --argument "(record{
  minting_account= opt record {
    owner = principal \"$BOT_CANISTER_ID\";
    subaccount = null;
  };                  
  icrc7_supply_cap= opt $MAX_SUPPLY;
  icrc7_description= opt \"Meme NFT\";
  tx_window= null;
  permitted_drift= null;
  icrc7_max_take_value= opt 128;
  icrc7_max_memo_size= opt 32;
  icrc7_symbol= \"MEME\";
  icrc7_max_update_batch_size= opt 64;
  icrc7_max_query_batch_size= opt 128;
  icrc7_atomic_batch_transfers= null;
  icrc7_default_take_value= opt 128;
  icrc7_logo= opt \"$LOGO_URL\";
  icrc7_name= \"Meme\";
  approval_init= null;
  archive_init= opt record {
      maxRecordsToArchive= 1000;
      archiveIndexType= variant {Stable};
      maxArchivePages= 100;
      settleToRecords= 100;
      archiveCycles= 1000000000000;
      maxActiveRecords= 10;
      maxRecordsInArchiveInstance= 10;
      archiveControllers= null
  }
})"

####
dfx canister update-settings nft --add-controller $BOT_CANISTER_ID

popd
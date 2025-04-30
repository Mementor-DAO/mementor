#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-wasip1/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/prod" ]; then
  cd ../..
fi

. .env

if [[ "$(dfx canister nft 2>&1 | grep Error)" != "" ]]; then
  dfx canister create nft --ic --subnet $SUBNET --identity deployer
fi
if [[ "$(dfx canister bot 2>&1 | grep Error)" != "" ]]; then
  dfx canister create bot --ic --subnet $SUBNET --identity deployer
fi

BOT_CANISTER_ID=$(dfx canister id bot --ic)
LOGO_URL="http://$BOT_CANISTER_ID.raw.icp0.io/assets/nft_logo.png"
MAX_SUPPLY=10000

dfx deploy nft -v --ic --identity deployer --with-cycles 3000000000000 --argument "(record{
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
dfx canister update-settings nft --ic --identity deployer --add-controller $BOT_CANISTER_ID

popd
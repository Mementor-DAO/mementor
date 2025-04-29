#!/bin/bash

set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/prod" ]; then
  cd ../..
fi

BOT_CANISTER_ID=$(dfx canister id bot --ic)

cargo run --bin img-uploader -- $BOT_CANISTER_ID ../identity.pem

popd
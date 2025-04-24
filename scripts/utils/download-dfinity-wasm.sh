#!/usr/bin/env bash

set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/utils" ]; then
  cd ../..
fi

CANISTER_NAME=$1
FILE_NAME=$2

COMMIT_ID=${3:-6973bac7af9b370056635e3162a8811b31e2b1e4}

echo "Downloading $CANISTER_NAME at commit $COMMIT_ID"

mkdir -p dependencies
mkdir -p dependencies/$CANISTER_NAME

HTTP_CODE=$(curl -so dependencies/$CANISTER_NAME/$CANISTER_NAME.gz https://download.dfinity.systems/ic/$COMMIT_ID/canisters/$FILE_NAME.wasm.gz --write-out "%{http_code}")

if [[ ${HTTP_CODE} -ne 200 ]] ; then
    echo "Failed to download wasm. Response code: ${HTTP_CODE}"
    exit 1
fi

echo "$CANISTER_NAME wasm downloaded"

popd
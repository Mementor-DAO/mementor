#!/bin/bash

set -e

export RELEASE_DIR=./target/wasm32-unknown-unknown/release

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts" ]; then
  cd ..
fi

cd dependencies/icrc7_launchpad

cargo build --release --target wasm32-unknown-unknown --package icrc7
ic-wasm $RELEASE_DIR/icrc7.wasm -o $RELEASE_DIR/icrc7.wasm shrink
gzip -f -c $RELEASE_DIR/icrc7.wasm > $RELEASE_DIR/icrc7.gz

popd


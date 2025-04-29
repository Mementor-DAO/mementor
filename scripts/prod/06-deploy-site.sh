#!/bin/bash

set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/prod" ]; then
  cd ../..
fi

cd ./packages/site

npm run build

popd

dfx deploy site -v --network ic --identity deployer --with-cycles 1000000000000


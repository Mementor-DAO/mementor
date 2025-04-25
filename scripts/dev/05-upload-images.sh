#!/bin/bash

set -e

pushd `pwd`

if [ "$(basename "$PWD")" = "scripts/dev" ]; then
  cd ../..
fi

for entry in "./packages/assets/images/"*.jpg
do
  fname=$(basename "$entry" .jpg)
  echo "$fname"
  image=$(od -t x1 -v -w1048576 -A n $entry | sed "s/ /\\\/g")
  dfx canister call bot --identity default insert_image --argument-file <(echo "(
      record {
        id=$fname;
        data=blob \"$image\";
        mime_type=\"image/jpeg\";
      }
  )")
done

popd
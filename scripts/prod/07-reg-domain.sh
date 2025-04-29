#!/bin/bash

CUSTOM_DOMAIN=mementor.fun

curl -sL -X POST \
    -H 'Content-Type: application/json' \
    https://icp0.io/registrations \
    --data @- <<EOF
{
    "name": "$CUSTOM_DOMAIN"
}
EOF

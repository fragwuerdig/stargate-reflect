#! /bin/sh

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_code",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.0 ./
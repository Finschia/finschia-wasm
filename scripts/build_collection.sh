#!/bin/bash

set -e

# commands
DOCKER=${DOCKER:-docker}

# Build collection contract
WORKSPACE_DIR=$(realpath $(dirname $0)/../)

if [ ! -f "${WORKSPACE_DIR}/artifacts/collection.wasm" ]; then
  echo "##### Build collection.wasm #####" >&2
  ${DOCKER} run --rm -v "${WORKSPACE_DIR}":/code \
    --mount type=volume,source="devcontract_cache_collection",target=/code/examples/contracts/collection/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer:0.15.0 ./examples/contracts/collection
fi

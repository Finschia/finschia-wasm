#!/bin/bash

set -e

# commands
DOCKER=${DOCKER:-docker}

# Build collection contract
WORKSPACE_DIR=$(realpath $(dirname $0)/../)

# if arm64, add it to container name's suffix
SUFFIX=""
if [ "$(uname -m)" = "arm64" ]; then
  SUFFIX="-arm64"
fi

if [ ! -f "${WORKSPACE_DIR}/artifacts/collection.wasm" ]; then
  echo "##### Build collection.wasm #####" >&2
  ${DOCKER} run --rm -v "${WORKSPACE_DIR}":/code \
    --mount type=volume,source="devcontract_cache_collection",target=/code/examples/contracts/collection/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/rust-optimizer${SUFFIX}:0.15.0 ./examples/contracts/collection
fi

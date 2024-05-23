#!/bin/bash
# Compiles and optimizes contracts

set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

cd "$(git rev-parse --show-toplevel)"
docker run --rm -v "$(pwd)":/code --platform linux/amd64 \
	--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/workspace-optimizer:0.15.1 # https://hub.docker.com/r/cosmwasm/workspace-optimizer/tags
ls -al ./artifacts/*wasm

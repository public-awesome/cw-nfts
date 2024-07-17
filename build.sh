#!/bin/bash
# Compiles and optimizes contracts

set -o errexit -o nounset -o pipefail
command -v shellcheck >/dev/null && shellcheck "$0"

cd "$(git rev-parse --show-toplevel)"
docker run --rm -v "$(pwd)":/code \
	--mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/optimizer:0.16.0 # https://hub.docker.com/r/cosmwasm/optimizer/tags
ls -al ./artifacts/*wasm

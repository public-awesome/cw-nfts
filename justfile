# Check formatting
fmt:
    cargo fmt --all --check

# Run tests
test:
    cargo test --locked

# Run clippy lints
lint:
    cargo clippy --tests -- -D warnings

# Build wasm release
build:
    cargo build --release --locked --target wasm32-unknown-unknown

# Optimize contracts using cosmwasm optimizer
optimize:
    #!/usr/bin/env bash
    if [[ $(arch) == "arm64" ]]; then
      image="cosmwasm/optimizer-arm64"
    else
      image="cosmwasm/optimizer"
    fi
    docker run --rm -v "$(pwd)":/code \
      --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
      --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
      ${image}:0.17.0

# Generate schemas for all packages and contracts
schema:
    #!/usr/bin/env bash
    for d in packages/*; do
      if [ -d "$d" ]; then
        cd $d
        cargo schema
        cd ../..
      fi
    done
    for d in contracts/*; do
      if [ -d "$d" ]; then
        cd $d
        cargo schema
        cd ../..
      fi
    done
    rm -rf contracts/**/schema/raw

# Publish crates to crates.io
publish:
    #!/usr/bin/env bash
    crates=(
      cw721
      cw721-base
      cw721-fixed-price
      cw721-metadata-onchain
      cw721-non-transferable
    )
    for crate in "${crates[@]}"; do
      cargo publish -p $crate
      echo "Sleeping before publishing the next crate..."
      sleep 30
    done

# Format TOML files
fmt-toml:
    taplo fmt

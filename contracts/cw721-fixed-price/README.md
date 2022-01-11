# CW721 Fixed Price

This contract enables the creation of limited edition fixed price NFTs according to the cw721 token standard.

## Instantiation

To instantiate a new instance of this contract you must specify a contract owner, a cw20 contract address for payment, a maximum mint amount, the unit price for each NFT, the cw721 code ID, and the NFT token info and metadata. 

The cw721 is created dynamically during contract instantiation, so there's no need to instantiate a cw721 token contract separately.

## Minting
An NFT can be minted using the cw20 [Send / Receive](https://github.com/CosmWasm/cw-plus/blob/main/packages/cw20/README.md#receiver) flow. A buyer must trigger a Send from the cw20 token contract with a payment amount equal to the unit price. If the payment amount is not equal to the unit price the transaction will be rejected. This contract will mint a single cw721 to sender.

## Development
### Compiling

To generate a development build run:
```
cargo build
```

To generate an optimized build run:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.3
```

### Testing
To execute unit tests run:
```
cargo test
```

### Format code
To lint repo run:
```
cargo fmt
```



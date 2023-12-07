# CW721 Expiration

One typical use cases for an `cw721-expiration` contract is providing services for a limited period, like access cards, SLAs, cloud services, etc. Also check kudos below.

This contract enables the creation of NFTs that expire after a predetermined number of days. The `expiration_days` is set during instantiation of contract.
Expiration timestamp is: mint timestamp + expiration days.


## Custom `cw721-base` Contract

### Query Messages

This contract extends cw721-base by adding a new `invalid NFT` utility. The following `cw721-base` query messages have been extended with an optional `include_invalid` property:

- `OwnerOf`: Queries owner of NFT, by default it throws an invalid NFT error.
- `Approval`: Queries whether spender has approval for a NFT, by default it throws an invalid NFT error.
- `Approvals`: Queries all approvals for a NFT, by default it throws an invalid NFT error.
- `NftInfo`: Queries NFT Info data, by default it throws an invalid NFT error.
- `AllNftInfo`: Queries NFT Info data, owner, and approvals, by default it throws an invalid NFT error.
- `Tokens`: Queries all token IDs owned by given address, by default it filters invalid NFTs.
- `AllTokens`: Queries all token IDs, by default it filters invalid NFTs.

In case NFT is invalid (due to expiration) an error is thrown or filtered out. Above queries for including invalid NFTs must explicitly pass `include_invalid: Some(true)` (in all other cases (`None`, `Some(false)`) invalid NFTs are excluded).

### Execute Messages

Execute messages are kept unchanged, but during execution an error is thrown for invalid NFTs for these operations:

- `TransferNft`: Transfers a NFT to another account without triggering an action.
- `SendNft`: Sends a NFT to another account and triggering an action.
- `Approve`: Allows operator/spender to transfer, send, and burn an NFT.
- `Revoke`: Revokes above approval.
- `Burn`: Burns an NFT.

## Instantiation

To instantiate a new instance of this contract you must specify `expiration_days` - along with cw721-based properties: `owner` (aka minter), `name`, and `symbol`.

## Development

### Compiling

To generate a development build run:

```sh
cargo build
```

To generate an optimized build run:

```sh
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.14.0
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

## Kudos

Kudos to [timpi](https://timpi.io/) for requesting this kind of NFTs, allowing NFT holders running nodes for a limited period.
# CW721 Expiration

This contract enables the creation of NFTs that expire after a predetermined number of days.
The expiration days is set during instantiation of contract.
Expiration timestampe is mint timestamp + expiration days.

## Custom `cw721-base` Contract

### Query Messages

This contract extends cw721-base by adding a new `invalid NFT` utilitiy. The following `cw721-base` query messages have been extended with an optional `include_invalid` property:

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

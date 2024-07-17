# CW721 Spec: Non Fungible Tokens

CW721 is a specification for non-fungible tokens (NFTs) based on CosmWasm.
The name and design is based on Ethereum's [ERC721](https://eips.ethereum.org/EIPS/eip-721) standard,
with some enhancements. The types in here can be imported by
contracts that wish to implement this spec, or by contracts that call
to any standard cw721 contract.

The specification is split into multiple sections, a contract may only
implement some of this functionality, but must implement the base.

## `cw721` package

The CW721 package provides 2 traits with default implementations (aka `utilities`):

- `Cw721Execute` with `Cw721ExecuteMsg` e.g. for minting, burning,
    sending, approving, and transferring NFTs. It also allows updating
    collection info, withdraw address, creator and minter ownership.
- `Cw721Query` with `Cw721QueryMsg` e.g. for NFTs, tokens, approvals, various kinds of ownerships (creator and minter).

Default implementations are opionated and uses a `Cw721Config` store. Custom cw721
contracts may re-implement each utlitiy to their own need.

### `cw721-base`

This handles ownership, transfers, and allowances. These must be supported
as is by all CW721 contracts. Note that all tokens must have an owner,
as well as an ID. The ID is an arbitrary string, unique within the contract.

`cw721-base` contract is the base contract for handling NFT collections with
either offchain (stored in `token_uri`) or onchain (stored in `NftInfo`'s extension')
metadata. Contract itself is lightweight, since all logic is provided in `cw721` package.

### Messages

`TransferNft{recipient, token_id}` -
This transfers ownership of the token to `recipient` account. This is
designed to send to an address controlled by a private key and _does not_
trigger any actions on the recipient if it is a contract.

Requires `token_id` to point to a valid token, and `env.sender` to be
the owner of it, or have an allowance to transfer it.

`SendNft{contract, token_id, msg}` -
This transfers ownership of the token to `contract` account. `contract`
must be an address controlled by a smart contract, which implements
the CW721Receiver interface. The `msg` will be passed to the recipient
contract, along with the token_id.

Requires `token_id` to point to a valid token, and `env.sender` to be
the owner of it, or have an allowance to transfer it.

`Approve{spender, token_id, expires}` - Grants permission to `spender` to
transfer or send the given token. This can only be performed when
`env.sender` is the owner of the given `token_id` or an `operator`.
There can be multiple spender accounts per token, and they are cleared once
the token is transferred or sent.

`Revoke{spender, token_id}` - This revokes a previously granted permission
to transfer the given `token_id`. This can only be granted when
`env.sender` is the owner of the given `token_id` or an `operator`.

`ApproveAll{operator, expires}` - Grant `operator` permission to transfer or send
all tokens owned by `env.sender`. This approval is tied to the owner, not the
tokens and applies to any future token that the owner receives as well.

`RevokeAll{operator}` - Revoke a previous `ApproveAll` permission granted
to the given `operator`.

### Queries

`OwnerOf{token_id, include_expired}` - Returns the owner of the given token,
as well as anyone with approval on this particular token. If the token is
unknown, returns an error. Return type is `OwnerOfResponse`. If
`include_expired` is set, show expired owners in the results, otherwise, ignore
them.

`Approval{token_id, spender, include_expired}` - Return an approval of `spender`
about the given `token_id`. Return type is `ApprovalResponse`. If
`include_expired` is set, show expired owners in the results, otherwise, ignore
them.

`Approvals{token_id, include_expired}` - Return all approvals that owner given
access to. Return type is `ApprovalsResponse`. If `include_expired` is set, show
expired owners in the results, otherwise, ignore them.

`AllOperators{owner, include_expired, start_after, limit}` - List all
operators that can access all of the owner's tokens. Return type is
`OperatorsResponse`. If `include_expired` is set, show expired owners in the
results, otherwise, ignore them. If `start_after` is set, then it returns the
first `limit` operators _after_ the given one.

`NumTokens{}` - Total number of tokens issued

### Receiver

The counter-part to `SendNft` is `ReceiveNft`, which must be implemented by
any contract that wishes to manage CW721 tokens. This is generally _not_
implemented by any CW721 contract.

`ReceiveNft{sender, token_id, msg}` - This is designed to handle `SendNft`
messages. The address of the contract is stored in `env.sender`
so it cannot be faked. The contract should ensure the sender matches
the token contract it expects to handle, and not allow arbitrary addresses.

The `sender` is the original account requesting to move the token
and `msg` is a `Binary` data that can be decoded into a contract-specific
message. This can be empty if we have only one default action,
or it may be a `ReceiveMsg` variant to clarify the intention. For example,
if I send to an exchange, I can specify the price I want to list the token
for.

## Metadata

### Queries

`CollectionInfo{}` - This returns top-level metadata about the contract.
Namely, `name` and `symbol`.

`NftInfo{token_id}` - This returns metadata about one particular token.
The return value is based on _ERC721 Metadata JSON Schema_, but directly
from the contract, not as a Uri. Only the image link is a Uri.

`AllNftInfo{token_id}` - This returns the result of both `NftInfo`
and `OwnerOf` as one query as an optimization for clients, which may
want both info to display one NFT.

## Enumerable

### Queries

Pagination is achieved via `start_after` and `limit`. Limit is a request
set by the client, if unset, the contract will automatically set it to
`DefaultLimit` (suggested 10). If set, it will be used up to a `MaxLimit`
value (suggested 30). Contracts can define other `DefaultLimit` and `MaxLimit`
values without violating the CW721 spec, and clients should not rely on
any particular values.

If `start_after` is unset, the query returns the first results, ordered
lexicographically by `token_id`. If `start_after` is set, then it returns the
first `limit` tokens _after_ the given one. This allows straightforward
pagination by taking the last result returned (a `token_id`) and using it
as the `start_after` value in a future query.

`Tokens{owner, start_after, limit}` - List all token_ids that belong to a given owner.
Return type is `TokensResponse{tokens: Vec<token_id>}`.

`AllTokens{start_after, limit}` - Requires pagination. Lists all token_ids controlled by
the contract.

### NftInfo Extension - CW721 Metadata Onchain

NFT creators may want to store their NFT metadata on-chain so other contracts are able to interact with it.
With CW721 in CosmWasm, we allow you to store any data on chain you wish, using a generic `extension: T`.

In order to support on-chain metadata, and to demonstrate how to use the extension ability, we have created this simple contract.
There is no business logic here, but looking at `lib.rs` will show you how do define custom data that is included when minting and
available in all queries.

In particular, here we define:

```rust
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;
```

In particular, the fields defined conform to the properties supported in the [OpenSea Metadata Standard](https://docs.opensea.io/docs/metadata-standards).


This means when you query `NftInfo{name: "Enterprise"}`, you will get something like:

```json
{
  "name": "Enterprise",
  "token_uri": "https://starships.example.com/Starship/Enterprise.json",
  "extension": {
    "image": null,
    "image_data": null,
    "external_url": null,
    "description": "Spaceship with Warp Drive",
    "name": "Starship USS Enterprise",
    "attributes": null,
    "background_color": null,
    "animation_url": null,
    "youtube_url": null
  }
}
```

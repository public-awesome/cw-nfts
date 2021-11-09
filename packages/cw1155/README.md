# CW1155 Spec: Multi Token Standard

CW1155 is a specification for multiple token types on CosmWasm.
The name and design is based on Ethereum's ERC1155 standard,
with some enhancements. The types in here can be imported by 
contracts that wish to implement this  spec, or by contracts that call 
to any standard 1155 contract.

The specification is split into multiple sections, a contract may only
implement some of this functionality, but must implement the base.

## Base

This handles ownership, transfers, and allowances. These must be supported
as is by all CW721 contracts. Note that all tokens must have an owner, 
as well as an ID. The ID is an arbitrary string, unique within the contract.

### Messages

`TransferNft{recipient, token_id, amount}` - 
Moves `amount` tokens to `recipient` account. This is 
designed to send to an address controlled by a private key and *does not* 
trigger any actions on the recipient if it is a contract.

Requires `token_id` to point to a valid token, `info.sender` to be 
the owner of it and `info.sender` to have a balance of at least amount
tokens.

`SendNft{contract, token_id, amount, msg}` - 
Moves `amount` tokens to `contract` account. `contract` 
must be an address controlled by a smart contract, which implements
the CW1155Receiver interface. The `msg` will be passed to the recipient 
contract, along with the token_id.

Requires `token_id` to point to a valid token, `info.sender` to be 
the owner of it and `info.sender` to have a balance of at least amount
tokens.

`IncreaseAllowance{spender, token_id, amount, expires}`
Set or increase the allowance such that `spender` may access up to
`amount + current_allowance` `token_id` tokens from the `info.sender`
account. This may optionally come with an `Expiration`time, which 
if set limits when the approval can be used (by time or height).

`DecreaseAllowance{spender, token_id, amount, expires}`
Decrease or clear the allowance such that `spender` may access up to
`amount + current_allowance` `token_id` tokens from the `info.sender`
account. This may optionally come with an `Expiration`time, which 
if set limits when the approval can be used (by time or height).
If `amount >= current_allowance`, this will clear the allowance (delete it).

`ApproveAll{operator, expires}` - Grant `operator` permission to transfer or send
all tokens owned by `info.sender`. This approval is tied to the owner, not the
tokens and applies to any future token that the owner receives as well.

`RevokeAll{operator}` - Revoke a previous `ApproveAll` permission granted
to the given `operator`.


### WIP
<!-- 
### Queries
`OwnerOf{token_id}` - Returns the owner of the given token,
as well as anyone with approval on this particular token.
If the token is unknown, returns an error. Return type is
`OwnerResponse{owner}`.

`ApprovedForAll{owner, include_expired}` - List all operators that can
access all of  the owner's tokens. Return type is `ApprovedForAllResponse`.
If `include_expired` is set, show expired owners in the results, otherwise,
ignore them.

`NumTokens{}` - Total number of tokens issued

### Receiver

The counter-part to `SendNft` is `ReceiveNft`, which must be implemented by
any contract that wishes to manage CW721 tokens. This is generally *not*
implemented by any CW721 contract.

`ReceiveNft{sender, token_id, msg}` - This is designed to handle `SendNft`
messages. The address of the contract is stored in `info.sender`
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

`ContractInfo{}` - This returns top-level metadata about the contract.
Namely, `name` and `symbol`.

`NftInfo{token_id}` - This returns metadata about one particular token.
The return value is based on *ERC721 Metadata JSON Schema*, but directly
from the contract, not as a Uri. Only the image link is a Uri.

`AllNftInfo{token_id}` - This returns the result of both `NftInfo`
and `OwnerOf` as one query as an optimization for clients, which may
want both info to display one NFT.

## Enumerable

### Queries

Pagination is acheived via `start_after` and `limit`. Limit is a request
set by the client, if unset, the contract will automatically set it to
`DefaultLimit` (suggested 10). If set, it will be used up to a `MaxLimit`
value (suggested 30). Contracts can define other `DefaultLimit` and `MaxLimit`
values without violating the CW721 spec, and clients should not rely on
any particular values.

If `start_after` is unset, the query returns the first results, ordered by
lexogaphically by `token_id`. If `start_after` is set, then it returns the
first `limit` tokens *after* the given one. This allows straight-forward 
pagination by taking the last result returned (a `token_id`) and using it
as the `start_after` value in a future query. 

`Tokens{owner, start_after, limit}` - List all token_ids that belong to a given owner.
Return type is `TokensResponse{tokens: Vec<token_id>}`.

`AllTokens{start_after, limit}` - Requires pagination. Lists all token_ids controlled by 
the contract. -->

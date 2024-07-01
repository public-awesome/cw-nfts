# CW-2981 Token-level Royalties

An example of porting EIP-2981 to implement royalties at a token mint level.

Builds on top of the metadata pattern in `cw721-metadata-onchain`.

All of the CW-721 logic and behaviour you would expect for an NFT is implemented as normal, but additionally at mint time, royalty information can be attached to a token.

Exposes two new query message types that can be called:

```rust
// Should be called on sale to see if royalties are owed
// by the marketplace selling the NFT.
// See https://eips.ethereum.org/EIPS/eip-2981
RoyaltyInfo {
    token_id: String,
    // the denom of this sale must also be the denom returned by RoyaltiesInfoResponse
    sale_price: Uint128,
},
// Called against the contract to signal that CW-2981 is implemented
CheckRoyalties {},
```

The responses are:

```rust
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RoyaltiesInfoResponse {
    pub address: String,
    // Note that this must be the same denom as that passed in to RoyaltyInfo
    // rounding up or down is at the discretion of the implementer
    pub royalty_amount: Uint128,
}

/// Shows if the contract implements royalties
/// if royalty_payments is true, marketplaces should pay them
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct CheckRoyaltiesResponse {
    pub royalty_payments: bool,
}
```


To set this information, new meta fields are available on mint:

```rust
    /// specify whether royalties are set on this token
    pub royalty_payments: bool,
    /// This is how much the minter takes as a cut when sold
    pub royalty_percentage: Option<u64>,
    /// The payment address, may be different to or the same
    /// as the minter addr
    /// question: how do we validate this?
    pub royalty_payment_address: Option<String>,
```

Note that the `royalty_payment_address` could of course be a single address, a multisig, or a DAO.

## A note on CheckRoyalties

For this contract, there's nothing to check. This hook is expected to be present to check if the contract does implement CW2981 and signal that on sale royalties should be checked. With the implementation at token level it should always return true because it's up to the token.

Of course contracts that extend this can determine their own behaviour and replace this function if they have more complex behaviour (for example, you could maintain a secondary index of which tokens actually have royalties).

In this super simple case that isn't necessary.

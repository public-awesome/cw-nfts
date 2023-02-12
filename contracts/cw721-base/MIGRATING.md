## 0.16 -> 0.17

The minter has been replaced by an owner which is updatable via the
two-step ownership transfer process of
[cw_ownable](https://crates.io/crates/cw-ownable). To retreive the
owner, `QueryMsg::Ownership {}` may be executed.

`QueryMsg::Minter {}` has not bee removed, though after version 0.16
the response type has made the minter field optional as it may be
unset. For all intents and purposes, whenever the word minter is used,
it means owner, and owner in turn means minter, minter iff owner.

Before 0.16:

```rust
pub struct MinterResponse {
    pub minter: String,
}
```

After 0.16:

```rust
pub struct MinterResponse {
    pub minter: Option<String>,
}
```

NFTs on version 0.16 may migrate to the new version. For an example of
doing so, see
[this](https://github.com/CosmWasm/cw-nfts/blob/zeke/updatable-minter/contracts/cw721-base/src/multi_tests.rs#L83)
integration test.


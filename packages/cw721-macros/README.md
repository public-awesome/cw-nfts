# CW721 Macros

This package defines macros that may be used to derive the CosmWasm-721 NFT interface.

The implementation used here is heavily inspired by
[DAO DAO](https://github.com/DA0-DA0/dao-contracts/tree/main/packages/cwd-macros).

## How to use

```rust
use cosmwasm_schema::cw_serde;
use cw721_macros::{cw721_execute, cw721_query};

#[cw721_execute]
#[cw_serde]
pub enum ExecuteMsg {
    Foo,
}

#[cw721_query]
#[cw_serde]
pub enum QueryMsg {
    Bar,
}
```

# Cw721 Basic

This is a basic implementation of a cw721 NFT contract. It implements
the [CW721 spec](../../packages/cw721/README.md) and is designed to
be deployed as is, or imported into other contracts to easily build
cw721-compatible NFTs with custom logic.

Implements:

- [x] CW721 Base
- [x] Metadata extension
- [x] Enumerable extension

## Implementation

The `ExecuteMsg` and `QueryMsg` implementations follow the [CW721 spec](../../packages/cw721/README.md) and are described there.
Beyond that, we make a few additions:

* `InstantiateMsg` takes name and symbol (for metadata), as well as a **Minter** address. This is a special address that has full
power to mint new NFTs (but not modify existing ones)
* `ExecuteMsg::Mint{token_id, owner, token_uri}` - creates a new token with given owner and (optional) metadata. It can only be called by
the Minter set in `instantiate`.
* `QueryMsg::Minter{}` - returns the minter address for this contract.

It requires all tokens to have defined metadata in the standard format (with no extensions). For generic NFTs this may often be enough.

The *Minter* can either be an external actor (e.g. web server, using PubKey) or another contract. If you just want to customize
the minting behavior but not other functionality, you could extend this contract (importing code and wiring it together)
or just create a custom contract as the owner and use that contract to Mint.

If provided, it is expected that the _token_uri_ points to a JSON file following the [ERC721 Metadata JSON Schema](https://eips.ethereum.org/EIPS/eip-721).

## Extending this contract
This contract is meant to be used as a base contract for implementing custom NFT contracts conforming to the `cw721` interface. Some examples extending this contract are included in this repo, including [cw721-metadata-onchain](../cw721-metadata-onchain) and [cw2981-royalties](../cw2981-royalties).

There are four main types of extensions:
* `MintExt`: Add custom onchain metadata to NFTs, NFT Info queries will return this metadata, and you will be able to use it in custom smart contract logic.
* `CollectionMetadataExt`: Add custom onchain collection metadata, the ContractInfo query will return with the info set here.
* `ExecuteExt`: For defining custom smart contract methods for your NFT that are in addition to the cw721 spec.
* `QueryExt`: For defining custom queries.

Each extension needs to implement the [CustomMsg trait](https://docs.rs/cosmwasm-std/1.1.1/cosmwasm_std/trait.CustomMsg.html), see the example below.

Here is a complete example using all four extensions:

```rust
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CustomMsg, Empty};
pub use cw721_base::{
    ContractError, Cw721Contract, ExecuteMsg, InstantiateMsg, MintMsg, MinterResponse, QueryMsg,
};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// MintExt allows for adding custom on-chain metadata to your NFTs
#[cw_serde]
pub struct MintExt {
    creator: String,
}
impl CustomMsg for MintExt {}

// Define custom contract metadata, the ContractInfo query will return with the info set here
#[cw_serde]
pub struct CollectionMetadataExt {
    pub creator: String,
}
impl CustomMsg for CollectionMetadataExt {}

// Define a custom query ext
#[cw_serde]
pub enum QueryExt {
    AdditionalQuery {},
}
impl CustomMsg for QueryExt {}

// Define a custom query response
#[cw_serde]
pub struct AdditionalQueryResponse {
    message: String,
}

// Define a custom execute extension. Allows for creating new contract methods
#[cw_serde]
pub enum ExecuteExt {
    AdditionalExecute {},
}
impl CustomMsg for ExecuteExt {}

// Put it all together!
pub type CustomCw721<'a> =
    Cw721Contract<'a, MintExt, Empty, CollectionMetadataExt, ExecuteExt, QueryExt>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::{entry_point, to_binary};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw2::set_contract_version;

    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg<CollectionMetadataExt>,
    ) -> Result<Response, ContractError> {
        // Call the instantiate on our base cw721 with our custom extensions
        let res = CustomCw721::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<MintExt, ExecuteExt>,
    ) -> Result<Response, ContractError> {
        match msg {
            // Match extension messages
            ExecuteMsg::Extension { msg } => match msg {
                // Map them to their message handlers
                ExecuteExt::AdditionalExecute {} => execute_custom(deps, env, info),
            },
            // Handle other messages with the cw721-base default
            _ => CustomCw721::default().execute(deps, env, info, msg),
        }
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<QueryExt>) -> StdResult<Binary> {
        match msg {
            // Match extension messages
            QueryMsg::Extension { msg } => match msg {
                // Map them to their message handlers
                QueryExt::AdditionalQuery {} => to_binary(&custom_query(deps, env)?),
            },
            // Handle other queries with the cw721-base default
            _ => CustomCw721::default().query(deps, env, msg),
        }
    }

    // Custom execute handler
    pub fn execute_custom(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        Ok(Response::default())
    }

    // Custom query handler
    pub fn custom_query(deps: Deps, env: Env) -> StdResult<AdditionalQueryResponse> {
        Ok(AdditionalQueryResponse {
            message: String::from("meow"),
        })
    }
}

```

## Running this contract

You will need Rust 1.60.0+ with `wasm32-unknown-unknown` target installed.

You can run unit tests on this via: 

`cargo test`

Once you are happy with the content, you can compile it to wasm via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw721_base.wasm .
ls -l cw721_base.wasm
sha256sum cw721_base.wasm
```

For more information on building and uploading contracts see the official [CosmWasm documentation website](https://docs.cosmwasm.com).

# CosmWasm NFTS

This repo is the official repository to work on all NFT standard and examples
in the CosmWasm ecosystem. `cw721` and `cw721-base` were moved from
[`cw-plus`](https://github.com/CosmWasm/cw-plus) to start this repo, but it shall evolve
as driven by the community's needs.

Please feel free to modify `cw721-base` as you need to support these projects and add many extensions
and additional standards (like [cw-2981](https://github.com/CosmWasm/cw-plus/pull/414)) to meet
the demands of the various NFT projects springing forth.

## Maintainers

This repo is not maintained directly by Confio (although we can provide some code reviews and support),
but rather by 4 highly active community members working on NFT projects of their own:

* [alwin-peng](https://github.com/alwin-peng)
* [ben2x4](https://github.com/ben2x4)
* [Callum-A](https://github.com/Callum-A)
* [ekez](https://github.com/ezekiiel)
* [JakeHartnell](https://github.com/JakeHartnell)
* [John Y](https://github.com/yubrew)
* [orkunkl](https://github.com/orkunkl)
* [shanev](https://github.com/shanev)
* [the-frey](https://github.com/the-frey)

## Contributing

If you are working on an NFT project as well and wish to give input, please raise issues and/or PRs.
Additional maintainers can be added if they show commitment to the project.

You can also join the `#nfts` channel on [CosmWasm Discord](https://docs.cosmwasm.com/chat)
for more interactive discussion on these themes.

## Contracts and Libs

### `cw721` Package

The `cw721` package contains contains [traits.rs](./packages/cw721/src/traits.rs). For ease-of-use the traits
`Cw721Query` and `Cw721Execute` provides default implementations and may be customized by contracts. All default
queries and operations are provided in [query.rs](./packages/cw721/src/query.rs) and [execute.rs](./packages/cw721/src/execute.rs).
3rd party contracts may call query and execute messages using `Cw721Helper` in [helpers.rs](./packages/cw721/src/helpers.rs).

This package provides generics for onchain nft and collection extensions. It allows custom cw721 contracts providing
their own data for nft and collection metadata. Based on ERC721 the following structs are provided:

* [msg.rs](./packages/cw721/src/msg.rs): `CollectionExtensionMsg<TRoyaltyInfoResponse>` and `NftExtensionMsg`
* [state.rs](./packages/cw721/src/state.rs): `CollectionExtension<TRoyaltyInfo>` and `NftExtension`

These structs are optional and these types in [libs.rs](./packages/cw721/src/lib.rs) may be used in contracts:

```rust
/// Type for `Option<CollectionExtension<RoyaltyInfo>>`
pub type DefaultOptionalCollectionExtension = Option<CollectionExtension<RoyaltyInfo>>;
/// Type for `Option<Empty>`
pub type EmptyOptionalCollectionExtension = Option<Empty>;

/// Type for `Option<CollectionExtensionMsg<RoyaltyInfoResponse>>`
pub type DefaultOptionalCollectionExtensionMsg =
    Option<CollectionExtensionMsg<RoyaltyInfoResponse>>;
/// Type for `Option<Empty>`
pub type EmptyOptionalCollectionExtensionMsg = Option<Empty>;

/// Type for `Option<NftExtension>`.
pub type DefaultOptionalNftExtension = Option<NftExtension>;
/// Type for `Option<Empty>`
pub type EmptyOptionalNftExtension = Option<Empty>;

/// Type for `Option<NftExtensionMsg>`.
pub type DefaultOptionalNftExtensionMsg = Option<NftExtensionMsg>;
/// Type for `Option<Empty>`
pub type EmptyOptionalNftExtensionMsg = Option<Empty>;
```

For a developer experience there are these opionated the helpers `DefaultCw721Helper` and `EmptyCw721Helper`. Example:

```rust
// DefaultCw721Helper for handling optional onchain nft and collection extensions.
let mint_msg = Cw721ExecuteMsg::<
    DefaultOptionalNftExtensionMsg,
    DefaultOptionalCollectionExtensionMsg,
    Empty,
>::Mint {
    token_id: config.unused_token_id.to_string(),
    owner: sender,
    token_uri: config.token_uri.clone().into(),
    extension,
};
let msg = DefaultCw721Helper::new(cw721).call(mint_msg)?;
Ok(Response::new().add_message(msg))
// Alternative for custom extensions using generic Cw721Helper:
let msg = Cw721Helper::<
    CustomNftExtension,
    CustomNftExtensionMsg,
    CustomCollectionExtension,
    CustomCollectionExtensionMsg,
    Empty,
    Empty,
>(
    cw721,
    PhantomData,
    PhantomData,
    PhantomData,
    PhantomData,
    PhantomData,
    PhantomData,
)
.call(mint_msg)?;
```

`Cw721Query`:

```rust
pub trait Cw721Query<
    // NftInfo extension (onchain metadata).
    TNftExtension,
    // CollectionInfo extension (onchain attributes).
    TCollectionExtension,
    // Custom query msg for custom contract logic. Default implementation returns an empty binary.
    TExtensionQueryMsg,
> where
    TNftExtension: Cw721State + Contains,
    TCollectionExtension: Cw721State + FromAttributesState,
    TExtensionQueryMsg: Cw721CustomMsg,
{
    fn query(
        &self,
        deps: Deps,
        env: &Env,
        msg: Cw721QueryMsg<TNftExtension, TCollectionExtension, TExtensionQueryMsg>,
    ) -> Result<Binary, Cw721ContractError> {
        match msg {
            #[allow(deprecated)]
            Cw721QueryMsg::Minter {} => Ok(to_json_binary(&self.query_minter(deps.storage)?)?),
        // ...
        }
    }
}
```

`Cw721Execute`:

```rust
pub trait Cw721Execute<
    // NftInfo extension (onchain metadata).
    TNftExtension,
    // NftInfo extension msg for onchain metadata.
    TNftExtensionMsg,
    // CollectionInfo extension (onchain attributes).
    TCollectionExtension,
    // CollectionInfo extension msg for onchain collection attributes.
    TCollectionExtensionMsg,
    // Custom extension msg for custom contract logic. Default implementation is a no-op.
    TExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg + StateFactory<TNftExtension>,
    TCollectionExtension: Cw721State + ToAttributesState + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionExtension>,
    TCustomResponseMsg: CustomMsg,
{
    fn instantiate_with_version(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        msg: Cw721InstantiateMsg<TCollectionExtensionMsg>,
        contract_name: &str,
        contract_version: &str,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        // ...
    }
    // ...
    fn execute(
        &self,
        deps: DepsMut,
        env: &Env,
        info: &MessageInfo,
        msg: Cw721ExecuteMsg<TNftExtensionMsg, TCollectionExtensionMsg, TExtensionMsg>,
    ) -> Result<Response<TCustomResponseMsg>, Cw721ContractError> {
        match msg {
            Cw721ExecuteMsg::UpdateCollectionInfo { collection_info } => {
                self.update_collection_info(deps, info.into(), env.into(), collection_info)
            }
            Cw721ExecuteMsg::Mint {
                token_id,
                owner,
                token_uri,
                extension,
            } => self.mint(deps, env, info, token_id, owner, token_uri, extension),
            // ...
        }
    }
    // ...
}
```

### `cw721-base`


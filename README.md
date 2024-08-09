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

* [mr t](https://github.com/taitruong)
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

tl;dr: contracts may use `Cw721OnchainExtensions` for onchain, `Cw721BaseExtensions` for offchain nft extension and optional onchain collection extension, `Cw721EmptyExtensions` for no extensions, or `Cw721Extensions` for custom metadata. These extension implements `Cw721Query` and `Cw721Execute` traits. Both traits provide default implemtations. A contract may customize and adjust specific trait functions.

[traits.rs](./packages/cw721/src/traits.rs) in `cw721` package provides `Cw721Query` and `Cw721Execute` provides. Both traits have default implementations and may be customized by contracts. Default
queries and operations are provided in [query.rs](./packages/cw721/src/query.rs) and [execute.rs](./packages/cw721/src/execute.rs).
3rd party contracts may call query and execute messages using `Cw721Helper` in [helpers.rs](./packages/cw721/src/helpers.rs).

This package provides generics for onchain nft and collection extensions. It allows custom cw721 contracts providing
their own data for nft and collection metadata. Based on ERC721 the following structs are provided:

* [msg.rs](./packages/cw721/src/msg.rs): `CollectionExtensionMsg<TRoyaltyInfoResponse>` and `NftExtensionMsg`
* [state.rs](./packages/cw721/src/state.rs): `CollectionExtension<TRoyaltyInfo>` and `NftExtension`

These structs and default types in [libs.rs](./packages/cw721/src/lib.rs) may be used in contracts:

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

For better developer experience there are these opionated helpers: `DefaultCw721Helper` and `EmptyCw721Helper`. Example:

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

### `cw721-base`

This contracts uses `Cw721BaseExtensions` for storing metadata offchain.

[lib.rs](./contracts/cw721-base/src/lib.rs):

```rust
    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg<EmptyOptionalCollectionExtensionMsg>,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721BaseExtensions::default();
        contract.instantiate_with_version(deps, &env, &info, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<
            EmptyOptionalNftExtensionMsg,
            EmptyOptionalCollectionExtensionMsg,
            Empty,
        >,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721BaseExtensions::default();
        contract.execute(deps, &env, &info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(
        deps: Deps,
        env: Env,
        msg: Cw721QueryMsg<EmptyOptionalNftExtension, EmptyOptionalCollectionExtension, Empty>,
    ) -> Result<Binary, Cw721ContractError> {
        let contract = Cw721BaseExtensions::default();
        contract.query(deps, &env, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn migrate(
        deps: DepsMut,
        env: Env,
        msg: Cw721MigrateMsg,
    ) -> Result<Response, Cw721ContractError> {
        let contract = Cw721BaseExtensions::default();
        contract.migrate(deps, env, msg, CONTRACT_NAME, CONTRACT_VERSION)
    }
```

`Cw721BaseExtensions` in [extension.rs](./packages/cw721/src/extension.rs):

```rust
/// Opionated version of generic `Cw721Extensions` with `EmptyOptionalNftExtension` and `DefaultOptionalCollectionExtension` using:
/// - `Empty` for NftInfo extension (onchain metadata).
/// - `Empty` for NftInfo extension msg for onchain metadata.
/// - `DefaultOptionalCollectionExtension` for CollectionInfo extension (onchain attributes).
/// - `DefaultOptionalCollectionExtensionMsg` for CollectionInfo extension msg for onchain collection attributes.
/// - `Empty` for custom extension msg for custom contract logic.
/// - `Empty` for custom query msg for custom contract logic.
/// - `Empty` for custom response msg for custom contract logic.
pub struct Cw721BaseExtensions<'a> {
    pub config: Cw721Config<'a, EmptyOptionalNftExtension>,
    pub(crate) _collection_extension: PhantomData<DefaultOptionalCollectionExtension>,
    pub(crate) _nft_extension_msg: PhantomData<EmptyOptionalNftExtensionMsg>,
    pub(crate) _collection_extension_msg: PhantomData<DefaultOptionalCollectionExtensionMsg>,
    pub(crate) _extension_msg: PhantomData<Empty>,
    pub(crate) _extension_query_msg: PhantomData<Empty>,
    pub(crate) _custom_response_msg: PhantomData<Empty>,
}

impl Default for Cw721BaseExtensions<'static> {
    fn default() -> Self {
        Self {
            config: Cw721Config::<EmptyOptionalNftExtension>::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}
```

### `cw721-metadata-onchain`

This contract uses `Cw721OnchainExtensions` for storing metadata onchain.

[lib.rs](./contracts/cw721-metadata-onchain/src/lib.rs):

```rust
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg<DefaultOptionalCollectionExtensionMsg>,
    ) -> Result<Response, Cw721ContractError> {
        Cw721MetadataContract::default().instantiate_with_version(
            deps.branch(),
            &env,
            &info,
            msg,
            CONTRACT_NAME,
            CONTRACT_VERSION,
        )
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, Cw721ContractError> {
        Cw721MetadataContract::default().execute(deps, &env, &info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, Cw721ContractError> {
        Cw721MetadataContract::default().query(deps, &env, msg)
    }
```

### Custom Contracts

Custom contracts may use `Cw721Extensions` and provide their custom structs for e.g. for `TNftExtension` and `TCollectionExtension`:

[extension.rs](./packages/cw721/src/extension.rs):

```rust
impl<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Extensions<
        'static,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}
```

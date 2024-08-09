use std::marker::PhantomData;

use cosmwasm_std::Empty;

use crate::{
    state::Cw721Config,
    traits::{Cw721CustomMsg, Cw721State},
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    DefaultOptionalNftExtension, DefaultOptionalNftExtensionMsg, EmptyOptionalCollectionExtension,
    EmptyOptionalCollectionExtensionMsg, EmptyOptionalNftExtension, EmptyOptionalNftExtensionMsg,
};

/// Opionated version of generic `Cw721Extensions` with default onchain nft and collection extensions using:
/// - `DefaultOptionalNftExtension` for NftInfo extension (onchain metadata).
/// - `DefaultOptionalNftExtensionMsg` for NftInfo extension msg for onchain metadata.
/// - `DefaultOptionalCollectionExtension` for CollectionInfo extension (onchain attributes).
/// - `DefaultOptionalCollectionExtensionMsg` for CollectionInfo extension msg for onchain collection attributes.
/// - `Empty` for custom extension msg for custom contract logic.
/// - `Empty` for custom query msg for custom contract logic.
/// - `Empty` for custom response msg for custom contract logic.
pub struct Cw721OnchainExtensions<'a> {
    pub config: Cw721Config<'a, DefaultOptionalNftExtension>,
    pub(crate) _collection_extension: PhantomData<DefaultOptionalCollectionExtension>,
    pub(crate) _nft_extension_msg: PhantomData<DefaultOptionalNftExtensionMsg>,
    pub(crate) _collection_extension_msg: PhantomData<DefaultOptionalCollectionExtensionMsg>,
    pub(crate) _extension_msg: PhantomData<Empty>,
    pub(crate) _extension_query_msg: PhantomData<Empty>,
    pub(crate) _custom_response_msg: PhantomData<Empty>,
}

impl Default for Cw721OnchainExtensions<'static> {
    fn default() -> Self {
        Self {
            config: Cw721Config::<DefaultOptionalNftExtension>::default(),
            _collection_extension: PhantomData,
            _nft_extension_msg: PhantomData,
            _collection_extension_msg: PhantomData,
            _extension_msg: PhantomData,
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}

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

/// Opionated version of generic `Cw721Extensions` with empty onchain nft and collection extensions using:
/// - `Empty` for NftInfo extension (onchain metadata).
/// - `Empty` for NftInfo extension msg for onchain metadata.
/// - `Empty` for CollectionInfo extension (onchain attributes).
/// - `Empty` for CollectionInfo extension msg for onchain collection attributes.
/// - `Empty` for custom extension msg for custom contract logic.
/// - `Empty` for custom query msg for custom contract logic.
/// - `Empty` for custom response msg for custom contract logic.
pub struct Cw721EmptyExtensions<'a> {
    pub config: Cw721Config<'a, EmptyOptionalNftExtension>,
    pub(crate) _collection_extension: PhantomData<EmptyOptionalCollectionExtension>,
    pub(crate) _nft_extension_msg: PhantomData<EmptyOptionalNftExtensionMsg>,
    pub(crate) _collection_extension_msg: PhantomData<EmptyOptionalCollectionExtensionMsg>,
    pub(crate) _extension_msg: PhantomData<Empty>,
    pub(crate) _extension_query_msg: PhantomData<Empty>,
    pub(crate) _custom_response_msg: PhantomData<Empty>,
}

impl Default for Cw721EmptyExtensions<'static> {
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

/// Generic `Cw721Extensions` which may be extended by other contracts with custom onchain nft and collection extensions.
///
/// Contract with generic onchain nft and collection extensions allows handling with:
/// - no extensions: by defining all extensions as `Empty` or `Option<Empty>`.
/// - opionated `DefaultOptionalNftExtension` and `DefaultOptionalCollectionExtension`.
///   - `DefaultOptionalNftExtension`: either with nft metadata (`Some<NftExtension>`) or none `None`.
///   - `DefaultOptionalCollectionExtension`: either with collection metadata (`Some<CollectionExtension>`) or none `None`.
/// - custom extensions: by defining custom extensions.
///
/// Generics:
/// - `TNftExtension` for NftInfo extension (onchain metadata).
/// - `TNftExtensionMsg` for NftInfo extension msg for onchain metadata.
/// - `TCollectionExtension` for CollectionInfo extension (onchain attributes).
/// - `TCollectionExtensionMsg` for CollectionInfo extension msg for onchain collection attributes.
/// - `TExtensionMsg` for custom extension msg for custom contract logic.
/// - `TExtensionQueryMsg` for custom query msg for custom contract logic.
/// - `TCustomResponseMsg` for custom response msg for custom contract logic.
///
/// Example:
/// ```rust
/// // instantiate:
/// let contract = Cw721Extensions::<
///     DefaultOptionalNftExtension, // use `Option<Empty>` or `Empty` for no nft metadata
///     DefaultOptionalNftExtensionMsg, // use `Option<Empty>` or `Empty` for no nft metadata
///     DefaultOptionalCollectionExtension, // use `Option<Empty>` or `Empty` for no collection metadata
///     DefaultOptionalCollectionExtensionMsg, // use `Option<Empty>` or `Empty` for no collection metadata
///     Empty, // no custom extension msg
///     Empty, // no custom query msg
///     Empty, // no custom response msg
/// >::default();
/// let info = mock_info(CREATOR, &[]);
/// let init_msg = Cw721InstantiateMsg {
///     name: "SpaceShips".to_string(),
///     symbol: "SPACE".to_string(),
///     collection_info_extension: None,
///     minter: None,
///     creator: None,
///     withdraw_address: None,
/// };
/// //...
/// // mint:
/// let token_id = "Enterprise";
/// let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
/// let extension = Some(NftExtensionMsg {
///     description: Some("description1".into()),
///     name: Some("name1".to_string()),
///     attributes: Some(vec![Trait {
///         display_type: None,
///         trait_type: "type1".to_string(),
///         value: "value1".to_string(),
///     }]),
///     ..NftExtensionMsg::default()
/// });
/// let exec_msg = Cw721ExecuteMsg::<
///     DefaultOptionalNftExtensionMsg,
///     DefaultOptionalCollectionExtensionMsg,
///     Empty,
/// >::Mint {
///     token_id: token_id.to_string(),
///     owner: "john".to_string(),
///     token_uri: token_uri.clone(),
///     extension: extension.clone(), // use `extension: None` for no metadata
/// };
/// //...
/// ```
pub struct Cw721Extensions<
    'a,
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
    // Custom query msg for custom contract logic. Default implementation returns an empty binary.
    TExtensionQueryMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
{
    pub config: Cw721Config<'a, TNftExtension>,
    pub(crate) _collection_extension: PhantomData<TCollectionExtension>,
    pub(crate) _nft_extension_msg: PhantomData<TNftExtensionMsg>,
    pub(crate) _collection_extension_msg: PhantomData<TCollectionExtensionMsg>,
    pub(crate) _extension_msg: PhantomData<TExtensionMsg>,
    pub(crate) _extension_query_msg: PhantomData<TExtensionQueryMsg>,
    pub(crate) _custom_response_msg: PhantomData<TCustomResponseMsg>,
}

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

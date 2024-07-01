use std::marker::PhantomData;

// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use cw721::traits::{Cw721CustomMsg, Cw721State};

#[deprecated(since = "0.19.0", note = "Please use `NftInfo`")]
pub type TokenInfo<TNftExtension> = NftInfo<TNftExtension>;

/// `cw721-base` with `TNftExtension` and `TCollectionExtension` allowing contract handling with:
/// - no extensions: `TNftExtension: Empty` and `TCollectionExtension: Empty`
/// - opionated `DefaultOptionalNftExtension` and `DefaultOptionalCollectionExtension`.
///   - `DefaultOptionalNftExtension`: either with nft metadata (`Some<NftExtension>`) or none `None`.
///   - `DefaultOptionalCollectionExtension`: either with collection metadata (`Some<CollectionExtension>`) or none `None`.
///
/// Example:
/// ```rust
/// // instantiate:
/// let contract = Cw721Contract::<
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
pub struct Cw721Contract<
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
    for Cw721Contract<
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

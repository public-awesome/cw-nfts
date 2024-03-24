use std::marker::PhantomData;

// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use cw721::traits::{Cw721CustomMsg, Cw721State};

#[deprecated(since = "0.19.0", note = "Please use `NftInfo`")]
pub type TokenInfo<TNftExtension> = NftInfo<TNftExtension>;

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
    pub(crate) _extension_query_msg: PhantomData<TExtensionQueryMsg>,
    pub(crate) _custom_response_msg: PhantomData<TCustomResponseMsg>,
}

impl<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Contract<
        'static,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
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
            _extension_query_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}

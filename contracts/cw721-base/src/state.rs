use std::marker::PhantomData;

// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use cw721::traits::{Cw721CustomMsg, Cw721State};

#[deprecated(since = "0.19.0", note = "Please use `NftInfo`")]
pub type TokenInfo<TNftMetadataExtension> = NftInfo<TNftMetadataExtension>;

pub struct Cw721Contract<
    'a,
    // NftInfo extension (onchain metadata).
    TNftMetadataExtension,
    // NftInfo extension msg for onchain metadata.
    TNftMetadataExtensionMsg,
    // CollectionMetadata extension (onchain attributes).
    TCollectionMetadataExtension,
    // CollectionMetadata extension msg for onchain collection attributes.
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
{
    pub config: Cw721Config<'a, TNftMetadataExtension>,
    pub(crate) _collection_metadata_extension: PhantomData<TCollectionMetadataExtension>,
    pub(crate) _custom_metadata_extension_msg: PhantomData<TNftMetadataExtensionMsg>,
    pub(crate) _custom_collection_metadata_extension_msg:
        PhantomData<TCollectionMetadataExtensionMsg>,
    pub(crate) _custom_response_msg: PhantomData<TCustomResponseMsg>,
}

impl<
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Contract<
        'static,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
            _collection_metadata_extension: PhantomData,
            _custom_metadata_extension_msg: PhantomData,
            _custom_collection_metadata_extension_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }
}

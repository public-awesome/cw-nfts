use std::marker::PhantomData;

use cosmwasm_std::CustomMsg;

use crate::state::Cw721Config;
use crate::traits::{
    Cw721CustomMsg, Cw721Execute, Cw721Query, Cw721State, FromAttributesState, StateFactory,
    ToAttributesState,
};

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

impl<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    for Cw721Contract<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TNftMetadataExtension>,
    TCollectionMetadataExtension: Cw721State + ToAttributesState + FromAttributesState,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionMetadataExtension>,
    TCustomResponseMsg: CustomMsg,
{
}

impl<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    > Cw721Query<TNftMetadataExtension, TCollectionMetadataExtension>
    for Cw721Contract<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State + FromAttributesState,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
    TCustomResponseMsg: CustomMsg,
{
}

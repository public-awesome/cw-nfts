use std::marker::PhantomData;

use cosmwasm_std::CustomMsg;

use crate::state::Cw721Config;
use crate::traits::{
    Contains, Cw721CustomMsg, Cw721Execute, Cw721Query, Cw721State, FromAttributesState,
    StateFactory, ToAttributesState,
};

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

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TCustomResponseMsg,
    >
    for Cw721Contract<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg + StateFactory<TNftExtension>,
    TCollectionExtension: Cw721State + ToAttributesState + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionExtension>,
    TCustomResponseMsg: CustomMsg,
{
}

impl<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Cw721Query<TNftExtension, TCollectionExtension, TExtensionQueryMsg>
    for Cw721Contract<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State + Contains,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State + FromAttributesState,
    TCollectionExtensionMsg: Cw721CustomMsg,
    TExtensionQueryMsg: Cw721CustomMsg,
    TCustomResponseMsg: CustomMsg,
{
}

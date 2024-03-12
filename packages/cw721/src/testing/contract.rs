use cosmwasm_std::CustomMsg;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::execute::{Cw721Execute, Update};
use crate::query::Cw721Query;
use crate::state::{Cw721Config, Validate};

pub struct Cw721Contract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TMetadataExtension,
    // Message passed for updating metadata.
    TMetadataExtensionMsg,
    // Extension defined in CollectionInfo.
    TCollectionInfoExtension,
    // Message passed for updating collection info extension.
    TCollectionInfoExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >,
}

impl<
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Contract<
        'static,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
        }
    }
}

impl<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >
where
    TMetadataExtension:
        Serialize + DeserializeOwned + Clone + Update<TMetadataExtensionMsg> + Validate,
    TMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension:
        Serialize + DeserializeOwned + Clone + Update<TCollectionInfoExtensionMsg> + Validate,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCustomResponseMsg: CustomMsg,
{
}

impl<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    > Cw721Query<TMetadataExtension, TCollectionInfoExtension>
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCustomResponseMsg: CustomMsg,
{
}

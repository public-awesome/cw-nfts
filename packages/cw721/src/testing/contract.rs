use cosmwasm_std::CustomMsg;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::execute::Cw721Execute;
use crate::query::Cw721Query;
use crate::state::Cw721Config;

pub struct Cw721Contract<
    'a,
    TMetadataExtension,
    TCustomResponseMessage,
    TMetadataExtensionMsg,
    TCollectionInfoExtension,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >,
}

impl<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    > Default
    for Cw721Contract<
        'static,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
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
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >
    Cw721Execute<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
}

impl<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    > Cw721Query<TMetadataExtension, TCollectionInfoExtension>
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
}

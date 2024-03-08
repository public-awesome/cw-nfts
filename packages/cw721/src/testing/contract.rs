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
    TExtensionExecuteMsg,
    TCollectionInfoExtension,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >,
}

impl<
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    > Default
    for Cw721Contract<
        'static,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
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
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
    Cw721Execute<
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
}

impl<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    > Cw721Query<TMetadataExtension, TCollectionInfoExtension>
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
}

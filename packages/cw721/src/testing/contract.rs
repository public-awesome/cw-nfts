use cosmwasm_std::CustomMsg;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::execute::Cw721Execute;
use crate::query::Cw721Query;
use crate::state::Cw721Config;

pub struct Cw721Contract<
    'a,
    TMetadata,
    TCustomResponseMessage,
    TExtensionExecuteMsg,
    TMetadataResponse,
    TCollectionInfoExtension,
> where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TMetadataResponse: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >,
}

impl<
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    > Default
    for Cw721Contract<
        'static,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
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
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
    Cw721Execute<
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
    for Cw721Contract<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
}

impl<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    > Cw721Query<TMetadata, TMetadataResponse, TCollectionInfoExtension>
    for Cw721Contract<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
        TCollectionInfoExtension,
    >
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
}

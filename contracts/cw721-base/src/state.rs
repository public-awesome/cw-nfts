use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

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

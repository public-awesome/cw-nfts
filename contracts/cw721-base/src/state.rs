use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

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

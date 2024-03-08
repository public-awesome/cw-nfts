use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::execute::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Cw721Contract;

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

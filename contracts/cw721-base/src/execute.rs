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

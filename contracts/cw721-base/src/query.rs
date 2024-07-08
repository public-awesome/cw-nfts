use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::query::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Cw721Contract;

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>
    Cw721Query<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
}

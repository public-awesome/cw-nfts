use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::query::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Cw721Contract;

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

use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::query::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Cw721Contract;

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
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCustomResponseMsg: CustomMsg,
{
}

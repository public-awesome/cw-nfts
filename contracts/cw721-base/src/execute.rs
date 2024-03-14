use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::execute::*;
use cw721::StateFactory;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::Cw721Contract;

impl<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    Cw721Execute<
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    for Cw721Contract<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg:
        Serialize + DeserializeOwned + Clone + StateFactory<TNftMetadataExtension>,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg:
        Serialize + DeserializeOwned + Clone + StateFactory<TCollectionMetadataExtension>,
    TCustomResponseMsg: CustomMsg,
{
}

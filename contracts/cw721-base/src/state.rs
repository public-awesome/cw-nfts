use cosmwasm_std::CustomMsg;
// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Cw721Contract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TMetadataExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMessage,
    // Message passed for updating metadata.
    TMetadataExtensionMsg,
    // Extension defined in CollectionInfo.
    TCollectionInfoExtension,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >,
}

impl<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    > Default
    for Cw721Contract<
        'static,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
        }
    }
}

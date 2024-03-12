// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Cw721Contract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >,
}

impl<
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Contract<
        'static,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
        }
    }
}

use cosmwasm_std::{CustomMsg, Timestamp};

// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;

use cw721_base::Cw721Contract;
use cw_storage_plus::{Item, Map};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Cw721ExpirationContract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    // Message passed for updating collection info extension.
    TCollectionMetadataExtensionMsg,
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    pub expiration_days: Item<'a, u16>, // max 65535 days
    pub mint_timestamps: Map<'a, &'a str, Timestamp>,
    pub base_contract: Cw721Contract<
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
    for Cw721ExpirationContract<
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
            expiration_days: Item::new("expiration_days"),
            mint_timestamps: Map::new("mint_timestamps"),
            base_contract: Cw721Contract::default(),
        }
    }
}

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
    TMetadataExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    // Message passed for updating metadata.
    TMetadataExtensionMsg,
    // Extension defined in CollectionInfo.
    TCollectionInfoExtension,
    // Message passed for updating collection info extension.
    TCollectionInfoExtensionMsg,
    TCustomResponseMsg,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    pub expiration_days: Item<'a, u16>, // max 65535 days
    pub mint_timestamps: Map<'a, &'a str, Timestamp>,
    pub base_contract: Cw721Contract<
        'a,
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    >,
}

impl<
        TMetadataExtension,
        TMetadataExtensionMsg,
        TCollectionInfoExtension,
        TCollectionInfoExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721ExpirationContract<
        'static,
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
{
    fn default() -> Self {
        Self {
            expiration_days: Item::new("expiration_days"),
            mint_timestamps: Map::new("mint_timestamps"),
            base_contract: Cw721Contract::default(),
        }
    }
}

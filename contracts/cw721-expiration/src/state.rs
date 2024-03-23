use cosmwasm_std::Timestamp;
use cw721_base::traits::{Cw721CustomMsg, Cw721State};
use cw721_base::Cw721Contract;
use cw_storage_plus::{Item, Map};

pub struct Cw721ExpirationContract<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    // Message for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    // Message for updating collection metadata extension.
    TCollectionMetadataExtensionMsg,
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
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
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
{
    fn default() -> Self {
        Self {
            expiration_days: Item::new("expiration_days"),
            mint_timestamps: Map::new("mint_timestamps"),
            base_contract: Cw721Contract::default(),
        }
    }
}

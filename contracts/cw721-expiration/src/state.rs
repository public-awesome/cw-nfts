use cosmwasm_std::Timestamp;
use cw721_base::traits::{Cw721CustomMsg, Cw721State};
use cw721_base::Cw721Contract;
use cw_storage_plus::{Item, Map};

pub struct Cw721ExpirationContract<
    'a,
    // NftInfo extension (onchain metadata).
    TNftExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    // NftInfo extension msg for onchain metadata.
    TNftExtensionMsg,
    // CollectionInfo extension (onchain attributes).
    TCollectionExtension,
    // CollectionInfo extension msg for onchain collection attributes.
    TCollectionExtensionMsg,
    TExtensionQueryMsg,
    TCustomResponseMsg,
> where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
{
    pub expiration_days: Item<'a, u16>, // max 65535 days
    pub mint_timestamps: Map<'a, &'a str, Timestamp>,
    pub base_contract: Cw721Contract<
        'a,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >,
}

impl<
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    > Default
    for Cw721ExpirationContract<
        'static,
        TNftExtension,
        TNftExtensionMsg,
        TCollectionExtension,
        TCollectionExtensionMsg,
        TExtensionQueryMsg,
        TCustomResponseMsg,
    >
where
    TNftExtension: Cw721State,
    TNftExtensionMsg: Cw721CustomMsg,
    TCollectionExtension: Cw721State,
    TCollectionExtensionMsg: Cw721CustomMsg,
{
    fn default() -> Self {
        Self {
            expiration_days: Item::new("expiration_days"),
            mint_timestamps: Map::new("mint_timestamps"),
            base_contract: Cw721Contract::default(),
        }
    }
}

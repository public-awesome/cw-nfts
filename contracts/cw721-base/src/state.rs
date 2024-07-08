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
    // Extension query message.
    TQueryExtensionMsg,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    pub config: Cw721Config<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >,
}

impl<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg> Default
    for Cw721Contract<
        'static,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
        }
    }
}

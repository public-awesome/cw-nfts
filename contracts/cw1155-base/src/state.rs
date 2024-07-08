use cosmwasm_std::CustomMsg;
use cw1155::state::Cw1155Config;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Cw1155Contract<
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
    pub config: Cw1155Config<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >,
}

impl<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg> Default
    for Cw1155Contract<
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
            config: Cw1155Config::default(),
        }
    }
}

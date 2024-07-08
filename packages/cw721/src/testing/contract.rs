use cosmwasm_std::CustomMsg;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::execute::Cw721Execute;
use crate::query::Cw721Query;
use crate::state::Cw721Config;

pub struct Cw721Contract<
    'a,
    TMetadataExtension,
    TCustomResponseMessage,
    TMetadataExtensionMsg,
    TQueryExtensionMsg,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
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
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self {
            config: Cw721Config::default(),
        }
    }
}

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>
    Cw721Execute<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
}

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>
    Cw721Query<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
    for Cw721Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
}

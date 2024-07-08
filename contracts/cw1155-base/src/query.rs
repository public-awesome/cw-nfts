use crate::Cw1155Contract;
use cosmwasm_std::CustomMsg;
use cw1155::query::Cw1155Query;
use cw721::query::Cw721Query;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>
    Cw1155Query<
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
    for Cw1155Contract<
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
    for Cw1155Contract<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TMetadataExtensionMsg,
        TQueryExtensionMsg,
    >
where
    TCustomResponseMessage: CustomMsg,
    TMetadataExtension: Clone + DeserializeOwned + Serialize,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
}

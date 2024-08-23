use crate::Cw1155Contract;
use cosmwasm_std::CustomMsg;
use cw1155::execute::Cw1155Execute;
use serde::de::DeserializeOwned;
use serde::Serialize;

impl<'a, TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>
    Cw1155Execute<
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

use cosmwasm_std::Empty;

use cw721::{
    msg::Cw721MigrateMsg, DefaultOptionalCollectionExtension,
    DefaultOptionalCollectionExtensionMsg, DefaultOptionalNftExtension,
    DefaultOptionalNftExtensionMsg,
};

pub type InstantiateMsg = cw721::msg::Cw721InstantiateMsg<DefaultOptionalCollectionExtensionMsg>;
pub type ExecuteMsg = cw721::msg::Cw721ExecuteMsg<
    DefaultOptionalNftExtensionMsg,
    DefaultOptionalCollectionExtensionMsg,
    Empty,
>;
pub type QueryMsg = cw721::msg::Cw721QueryMsg<
    DefaultOptionalNftExtension,
    DefaultOptionalCollectionExtension,
    Empty,
>;
pub type MigrateMsg = Cw721MigrateMsg;

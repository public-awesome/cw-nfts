use cosmwasm_std::Empty;

use cw721::{
    msg::{Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721MigrateMsg, Cw721QueryMsg},
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    EmptyOptionalNftExtension, EmptyOptionalNftExtensionMsg,
};

pub type ExecuteMsg =
    Cw721ExecuteMsg<EmptyOptionalNftExtensionMsg, DefaultOptionalCollectionExtensionMsg, Empty>;
pub type InstantiateMsg = Cw721InstantiateMsg<DefaultOptionalCollectionExtensionMsg>;
pub type MigrateMsg = Cw721MigrateMsg;
pub type QueryMsg =
    Cw721QueryMsg<EmptyOptionalNftExtension, DefaultOptionalCollectionExtension, Empty>;

use cosmwasm_std::Empty;

use cw721::{
    msg::{Cw721ExecuteMsg, Cw721InstantiateMsg, Cw721MigrateMsg, Cw721QueryMsg},
    EmptyOptionalCollectionExtension, EmptyOptionalCollectionExtensionMsg,
    EmptyOptionalNftExtension, EmptyOptionalNftExtensionMsg,
};

pub type ExecuteMsg =
    Cw721ExecuteMsg<EmptyOptionalNftExtensionMsg, EmptyOptionalCollectionExtensionMsg, Empty>;
pub type InstantiateMsg = Cw721InstantiateMsg<EmptyOptionalCollectionExtensionMsg>;
pub type MigrateMsg = Cw721MigrateMsg;
pub type QueryMsg =
    Cw721QueryMsg<EmptyOptionalNftExtension, EmptyOptionalCollectionExtension, Empty>;

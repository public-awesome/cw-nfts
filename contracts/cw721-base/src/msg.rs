use cosmwasm_std::Empty;
// expose so other libs dont need to import cw721
pub use cw721::msg::*;
use cw721::{
    EmptyOptionalCollectionExtension, EmptyOptionalCollectionExtensionMsg,
    EmptyOptionalNftExtension, EmptyOptionalNftExtensionMsg,
};

pub type ExecuteMsg =
    Cw721ExecuteMsg<EmptyOptionalNftExtensionMsg, EmptyOptionalCollectionExtensionMsg, Empty>;
pub type InstantiateMsg = Cw721InstantiateMsg<EmptyOptionalCollectionExtensionMsg>;
pub type MigrateMsg = Cw721MigrateMsg;
pub type QueryMsg =
    Cw721QueryMsg<EmptyOptionalNftExtension, EmptyOptionalCollectionExtension, Empty>;

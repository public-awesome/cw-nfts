// expose to all others using contract, so others dont need to import cw721
pub use cw721::msg::{
    Cw721ExecuteMsg as ExecuteMsg, Cw721InstantiateMsg as InstantiateMsg,
    Cw721MigrateMsg as MigrateMsg, Cw721QueryMsg as QueryMsg, *,
};

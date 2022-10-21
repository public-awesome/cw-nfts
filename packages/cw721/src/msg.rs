use cosmwasm_schema::cw_serde;
use cw721_macros::cw721_execute;

#[cw721_execute]
#[cw_serde]
pub enum Cw721ExecuteMsg {}

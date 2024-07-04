use cosmwasm_schema::write_api;

use cosmwasm_std::Empty;
use cw2981_royalties::{msg::QueryMsg, ExecuteMsg};
use cw721::msg::Cw721InstantiateMsg;

fn main() {
    write_api! {
        instantiate: Cw721InstantiateMsg<Empty>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

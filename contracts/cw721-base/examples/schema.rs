use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_base::{EmptyCollectionInfoExtension, ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<EmptyCollectionInfoExtension>,
        execute: ExecuteMsg<Empty, Empty>,
        query: QueryMsg<Empty>,
    }
}

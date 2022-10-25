use cosmwasm_schema::write_api;

use cw721_base::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg<Empty>,
    }
}

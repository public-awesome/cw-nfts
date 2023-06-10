use cosmwasm_schema::write_api;

use cw721_base::InstantiateMsg;
use {{crate_name}}::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

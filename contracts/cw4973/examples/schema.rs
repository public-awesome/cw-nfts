use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw4973::msg::{ExecuteMsg, InstantiateMsg};
use cw721_base::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg<Empty>
    }
}

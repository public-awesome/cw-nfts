use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw1155::{Cw1155ExecuteMsg, Cw1155QueryMsg};

fn main() {
    write_api! {
        instantiate: Empty,
        execute: Cw1155ExecuteMsg,
        query: Cw1155QueryMsg
    }
}

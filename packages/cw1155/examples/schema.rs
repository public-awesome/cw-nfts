use std::fs::create_dir_all;

use cosmwasm_schema::{remove_schemas, write_api};
use cosmwasm_std::Empty;

use cw1155::{Cw1155ExecuteMsg, Cw1155QueryMsg};

type Extension = Empty;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: Cw1155ExecuteMsg,
        query: Cw1155QueryMsg,
    }
}

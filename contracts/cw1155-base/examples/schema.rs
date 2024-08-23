use cosmwasm_schema::write_api;
use cw1155::msg::Cw1155InstantiateMsg;

use cw1155_base::{Cw1155BaseExecuteMsg, Cw1155BaseQueryMsg};

fn main() {
    write_api! {
        instantiate: Cw1155InstantiateMsg,
        execute: Cw1155BaseExecuteMsg,
        query: Cw1155BaseQueryMsg,
    }
}

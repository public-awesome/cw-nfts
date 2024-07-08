use cosmwasm_schema::write_api;
use cw1155::msg::Cw1155InstantiateMsg;
use cw1155_royalties::{Cw1155RoyaltiesExecuteMsg, Cw1155RoyaltiesQueryMsg};

fn main() {
    write_api! {
        instantiate: Cw1155InstantiateMsg,
        execute: Cw1155RoyaltiesExecuteMsg,
        query: Cw1155RoyaltiesQueryMsg,
    }
}

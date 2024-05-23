use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw1155::{Cw1155InstantiateMsg, Cw1155QueryMsg};
use cw1155_metadata_onchain::Cw1155MetadataExecuteMsg;

fn main() {
    write_api! {
        instantiate: Cw1155InstantiateMsg,
        execute: Cw1155MetadataExecuteMsg,
        query: Cw1155QueryMsg<Empty>,
    }
}

use cosmwasm_schema::write_api;

use cw721::EmptyCollectionInfoExtension;
use cw721_metadata_onchain::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<EmptyCollectionInfoExtension>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

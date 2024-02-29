use cosmwasm_schema::write_api;

use cw2981_royalties::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw721::EmptyCollectionInfoExtension;

fn main() {
    write_api! {
        instantiate: InstantiateMsg<EmptyCollectionInfoExtension>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

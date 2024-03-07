use cosmwasm_schema::write_api;

use cw2981_royalties::{ExecuteMsg, InstantiateMsg, QueryMsg};
use cw721::state::DefaultOptionCollectionInfoExtension;

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionCollectionInfoExtension>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

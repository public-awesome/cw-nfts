use cosmwasm_schema::write_api;

use cw721::state::DefaultOptionCollectionMetadataExtension;
use cw721_fixed_price::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionCollectionMetadataExtension>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

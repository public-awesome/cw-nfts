use cosmwasm_schema::write_api;

use cw721::state::DefaultOptionCollectionInfoExtension;
use cw721_fixed_price::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionCollectionInfoExtension>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

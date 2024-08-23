use cosmwasm_schema::write_api;

use cw721::DefaultOptionalCollectionExtension;
use cw721_fixed_price::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionalCollectionExtension>,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

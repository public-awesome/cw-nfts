use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;
use cw721::state::DefaultOptionCollectionInfoExtension;
use cw721_expiration::msg::{InstantiateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg<DefaultOptionCollectionInfoExtension>,
        query: QueryMsg<Empty, DefaultOptionCollectionInfoExtension>,
    }
}

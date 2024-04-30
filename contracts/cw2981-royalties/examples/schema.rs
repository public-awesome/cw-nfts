use cosmwasm_schema::write_api;

use cw2981_royalties::{msg::QueryMsg, ExecuteMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}

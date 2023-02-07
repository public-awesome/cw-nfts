use cosmwasm_schema::write_api;
use cosmwasm_std::Empty;

use cw721_base::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg<Empty, Empty>,
        query: QueryMsg<Empty>,
        migrate: MigrateMsg,
    }
}

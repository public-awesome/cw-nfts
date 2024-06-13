use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// expose to all others using contract, so others dont need to import cw721
pub use cw721::state::*;

#[cw_serde]
pub struct Config {
    pub admin: Option<Addr>,
}

pub const CONFIG: Item<Config> = Item::new("config");

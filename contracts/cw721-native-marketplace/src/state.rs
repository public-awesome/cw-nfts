use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub nft_contract_addr: Addr,
    pub allowed_native: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub owner: Addr,
    pub id: String,
    pub price: Uint128,
    pub on_sale: bool,
}

pub const TOKENS: Map<String, Token> = Map::new("tokens");

use cosmwasm_std::{Empty, Timestamp};
use cw721::Cw721;
use cw_storage_plus::{Item, Map};

use crate::Extension;

pub struct Cw721ExpirationContract<'a> {
    pub expiration_days: Item<'a, u64>,
    pub mint_timestamps: Map<'a, &'a str, Timestamp>,
    pub base_contract: cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>,
}

// This is a signal, the implementations are in other files
impl<'a> Cw721<Extension, Empty> for Cw721ExpirationContract<'a> {}

impl Default for Cw721ExpirationContract<'static> {
    fn default() -> Self {
        Self {
            expiration_days: Item::new("expiration_days"),
            mint_timestamps: Map::new("mint_timestamps"),
            base_contract: cw721_base::Cw721Contract::default(),
        }
    }
}

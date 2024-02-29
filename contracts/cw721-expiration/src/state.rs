use cosmwasm_std::{Empty, Timestamp};
use cw_storage_plus::{Item, Map};

use crate::{EmptyCollectionInfoExtension, EmptyExtension};

pub struct Cw721ExpirationContract<'a> {
    pub expiration_days: Item<'a, u16>, // max 65535 days
    pub mint_timestamps: Map<'a, &'a str, Timestamp>,
    pub base_contract: cw721_base::Cw721Contract<
        'a,
        EmptyExtension,
        Empty,
        Empty,
        Empty,
        EmptyCollectionInfoExtension,
    >,
}

impl Default for Cw721ExpirationContract<'static> {
    fn default() -> Self {
        Self {
            expiration_days: Item::new("expiration_days"),
            mint_timestamps: Map::new("mint_timestamps"),
            base_contract: cw721_base::Cw721Contract::default(),
        }
    }
}

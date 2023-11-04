use cosmwasm_std::Empty;
use cw721::Cw721;

use crate::Extension;

#[derive(Default)]
pub struct Cw721ExpirationContract<'a> {
    pub base_contract: cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>,
}

// This is a signal, the implementations are in other files
impl<'a> Cw721<Extension, Empty> for Cw721ExpirationContract<'a> {}

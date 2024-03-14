pub mod error;
pub mod execute;
pub mod helpers;
pub mod msg;
pub mod query;
pub mod receiver;
pub mod state;

use cosmwasm_std::{Deps, Empty, Env, MessageInfo};
pub use cw_utils::Expiration;
use error::Cw721ContractError;
pub use state::{Approval, CollectionMetadataExtension, RoyaltyInfo};

#[cfg(test)]
pub mod testing;

pub trait StateFactory<S> {
    fn create(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        current: Option<&S>,
    ) -> Result<S, Cw721ContractError>;
    fn validate(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        current: Option<&S>,
    ) -> Result<(), Cw721ContractError>;
}

impl StateFactory<Empty> for Empty {
    fn create(
        &self,
        _deps: Deps,
        _env: &Env,
        _info: &MessageInfo,
        _current: Option<&Empty>,
    ) -> Result<Empty, Cw721ContractError> {
        Ok(Empty {})
    }

    fn validate(
        &self,
        _deps: Deps,
        _env: &Env,
        _info: &MessageInfo,
        _current: Option<&Empty>,
    ) -> Result<(), Cw721ContractError> {
        Ok(())
    }
}

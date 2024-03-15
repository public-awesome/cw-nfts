use std::fmt::Debug;

use cosmwasm_std::{Deps, Empty, Env, MessageInfo};
use schemars::JsonSchema;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Cw721ContractError;

/// This is an exact copy of `CustomMsg`, since implementing a trait for a type from another crate is not possible.
///
/// Possible:
/// `impl<T> Cw721CustomMsg for Option<T> where T: Cw721CustomMsg {}`
///
/// Not possible:
/// `impl<T> CustomMsg for Option<T> where T: CustomMsg {}`
pub trait Cw721CustomMsg: Serialize + Clone + Debug + PartialEq + JsonSchema {}

pub trait Cw721State: Serialize + DeserializeOwned + Clone {}

impl Cw721State for Empty {}
impl<T> Cw721State for Option<T> where T: Cw721State {}
impl Cw721CustomMsg for Empty {}
impl<T> Cw721CustomMsg for Option<T> where T: Cw721CustomMsg {}

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

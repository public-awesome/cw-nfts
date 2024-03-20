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
///
/// This will be removed once the `CustomMsg` trait is moved to the `cosmwasm_std` crate: https://github.com/CosmWasm/cosmwasm/issues/2056
pub trait Cw721CustomMsg: Serialize + Clone + Debug + PartialEq + JsonSchema {}

pub trait Cw721State: Serialize + DeserializeOwned + Clone + Debug {}

impl Cw721State for Empty {}
impl<T> Cw721State for Option<T> where T: Cw721State {}
impl Cw721CustomMsg for Empty {}
impl<T> Cw721CustomMsg for Option<T> where T: Cw721CustomMsg {}

pub trait StateFactory<TState> {
    fn create(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&TState>,
    ) -> Result<TState, Cw721ContractError>;
    fn validate(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&TState>,
    ) -> Result<(), Cw721ContractError>;
}

impl StateFactory<Empty> for Empty {
    fn create(
        &self,
        _deps: Option<Deps>,
        _env: Option<&Env>,
        _info: Option<&MessageInfo>,
        _current: Option<&Empty>,
    ) -> Result<Empty, Cw721ContractError> {
        Ok(Empty {})
    }

    fn validate(
        &self,
        _deps: Option<Deps>,
        _env: Option<&Env>,
        _info: Option<&MessageInfo>,
        _current: Option<&Empty>,
    ) -> Result<(), Cw721ContractError> {
        Ok(())
    }
}

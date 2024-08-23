use cosmwasm_std::{OverflowError, StdError, Uint128};
use cw2::VersionError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Cw1155ContractError {
    #[error("StdError: {0}")]
    Std(#[from] StdError),

    #[error("OverflowError: {0}")]
    Overflow(#[from] OverflowError),

    #[error("VersionError: {0}")]
    Version(#[from] VersionError),

    #[error("OwnershipError: {0}")]
    Ownership(#[from] OwnershipError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Expired")]
    Expired {},

    #[error("Zero amount provided")]
    InvalidZeroAmount {},

    #[error("Not enough tokens available for this action. Available: {available}, Requested: {requested}.")]
    NotEnoughTokens {
        available: Uint128,
        requested: Uint128,
    },

    #[error("Must provide either 'token_uri' or 'extension' to update.")]
    NoUpdatesRequested {},
}

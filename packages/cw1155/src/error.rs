use cosmwasm_std::{OverflowError, StdError};
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
}

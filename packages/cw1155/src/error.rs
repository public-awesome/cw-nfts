use cosmwasm_std::{OverflowError, StdError};
use cw2::VersionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Cw1155ContractError {
    #[error("StdError: {0}")]
    Std(#[from] StdError),

    #[error("OverflowError: {0}")]
    Overflow(#[from] OverflowError),

    #[error("Version: {0}")]
    Version(#[from] VersionError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Expired")]
    Expired {},
}

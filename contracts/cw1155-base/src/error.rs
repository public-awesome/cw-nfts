use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Invalid state found: Error: {msg}")]
    InvalidState { msg: String },

    #[error("Invalid state found: Owner({owner}) does not have enough balance for {token_id}")]
    InsufficientBalance { token_id: String, owner: Addr },

    #[error("There was an addition overflow")]
    OverflowError {},
}

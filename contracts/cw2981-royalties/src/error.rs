use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Base(#[from] cw721::error::Cw721ContractError),

    #[error("Royalty percentage must be between 0 and 100")]
    InvalidRoyaltyPercentage,
}

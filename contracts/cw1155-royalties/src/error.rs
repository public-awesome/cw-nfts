use cosmwasm_std::StdError;
use cw1155::error::Cw1155ContractError;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum Cw1155RoyaltiesContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Base(#[from] Cw1155ContractError),

    #[error("Royalty percentage must be between 0 and 100")]
    InvalidRoyaltyPercentage,

    #[error("Invalid royalty payment address")]
    InvalidRoyaltyPaymentAddress,
}

use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, PartialEq)]
pub enum Cw721ContractError {
    #[error(transparent)]
    ParseError(#[from] ParseError),

    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error(transparent)]
    Version(#[from] cw2::VersionError),

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("No withdraw address set")]
    NoWithdrawAddress {},

    #[error("Collection description must not be empty")]
    CollectionDescriptionEmpty {},

    #[error("Collection description too long. Max length is 512 characters.")]
    CollectionDescriptionTooLong {},

    #[error("InvalidRoyalties: {0}")]
    InvalidRoyalties(String),
}

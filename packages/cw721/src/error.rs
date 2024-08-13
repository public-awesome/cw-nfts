use std::{num::ParseIntError, str::ParseBoolError};

use cosmwasm_std::StdError;
use cw_ownable::OwnershipError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug, PartialEq)]
pub enum Cw721ContractError {
    #[error(transparent)]
    ParseError(#[from] ParseError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error(transparent)]
    ParseBoolError(#[from] ParseBoolError),

    #[error("Unsupoorted custom attribute type: {key}={value}")]
    UnsupportedCustomAttributeType { key: String, value: String },

    #[error("Missing {0} attribute")]
    AttributeMissing(String),

    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error(transparent)]
    Version(#[from] cw2::VersionError),

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Caller is not collection creator")]
    NotCreator {},

    #[error("Caller is not minter")]
    NotMinter {},

    #[error("Caller is neither minter nor collection creator")]
    NotMinterOrCreator {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("No withdraw address set")]
    NoWithdrawAddress {},

    #[error("Collection name must not be empty")]
    CollectionNameEmpty {},

    #[error("Collection symbol must not be empty")]
    CollectionSymbolEmpty {},

    #[error("Collection description must not be empty")]
    CollectionDescriptionEmpty {},

    #[error("Collection description too long. Max length is {max_length} characters.")]
    CollectionDescriptionTooLong { max_length: u32 },

    #[error("InvalidRoyalties: {0}")]
    InvalidRoyalties(String),

    #[error("Trait type in metadata must not be empty")]
    TraitTypeEmpty {},

    #[error("Trait value in metadata must not be empty")]
    TraitValueEmpty {},

    #[error("Trait display type in metadata must not be empty")]
    TraitDisplayTypeEmpty {},

    #[error("Internal error. Missing argument: Info")]
    NoInfo,
}

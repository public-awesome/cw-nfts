use cosmwasm_std::Timestamp;

// expose so other libs dont need to import cw721-base
#[allow(unused_imports)]
pub use cw721_base::error::*;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    Cw721(#[from] Cw721ContractError),

    #[error("A minimum expiration day of 1 must be set")]
    MinExpiration {},

    #[error("Token {token_id} minted at {mint_date} expired at {expiration}")]
    NftExpired {
        token_id: String,
        mint_date: Timestamp,
        expiration: Timestamp,
    },
}

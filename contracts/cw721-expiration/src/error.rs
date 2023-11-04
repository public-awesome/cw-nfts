use cw721_base::error::ContractError as Cw721ContractError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] cosmwasm_std::StdError),

    #[error(transparent)]
    Cw721(#[from] Cw721ContractError),

    #[error("A minimum expiration day of 1 must be set")]
    MinExpiration {},
}

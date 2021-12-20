use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("NotFound")]
    NotFound {},

    #[error("Native token not in allowed list: {denom}")]
    NativeDenomNotAllowed { denom: String },

    #[error("CW20 token not in allowed list: {addr}")]
    CW20TokenNotAllowed { addr: String },

    #[error("Send single native token type")]
    SendSingleNativeToken {},

    #[error("Insufficient balance, need: {need} sent: {sent}")]
    InsufficientBalance { need: Uint128, sent: Uint128 },

    #[error("NFT not on sale")]
    NftNotOnSale {},

    #[error("Marketplace contract is not approved as operator")]
    NotApproved {},

    #[error("Approval expired")]
    ApprovalExpired {},

    #[error("Wrong input")]
    WrongInput {},
}

use cosmwasm_std::StdError;
pub use cw721_base::ContractError as Cw721ContractError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Cw721ContractError(Cw721ContractError),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Cannot give to yourself")]
    CannotGiveToSelf,

    #[error("Cannot take from yourself")]
    CannotTakeFromSelf,

    #[error("Caller is not minter")]
    NotMinter,

    #[error("NFT is already unequipped")]
    NftAlreadyUnequipped,

    #[error("NFT is already equipped")]
    NftAlreadyEquipped,

    #[error("NFT is already unadmitted")]
    NftAlreadyUnadmitted,

    #[error("Invalid human readable path {0}")]
    Hrp(String),

    #[error("Invalid receiver address")]
    To,

    #[error("Invalid sender address")]
    From,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Invalid signer")]
    InvalidSigner,

    #[error("Cannot verify signature")]
    CannotVerifySignature,

    #[error("Cannot unequip NFT")]
    CannotUnequipNFT,
}

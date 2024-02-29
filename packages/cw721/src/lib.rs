mod msg;
mod query;
mod receiver;
mod state;
mod traits;

use cosmwasm_std::Empty;
pub use cw_utils::Expiration;

pub use crate::msg::Cw721ExecuteMsg;
#[allow(deprecated)]
pub use crate::query::{
    AllNftInfoResponse, Approval, ApprovalResponse, ApprovalsResponse, ContractInfoResponse,
    Cw721QueryMsg, NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse,
    OwnerOfResponse, TokensResponse,
};
pub use crate::receiver::Cw721ReceiveMsg;
pub use crate::state::{CollectionInfo, Metadata, MetadataExtension, Trait};
pub use crate::traits::{Cw721, Cw721Execute, Cw721Query};

// These are simple types to let us handle empty extensions
pub type EmptyExtension = Option<Empty>;
pub type EmptyCollectionInfoExtension = Option<Empty>;

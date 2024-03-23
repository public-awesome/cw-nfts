pub mod error;
pub mod execute;
pub mod helpers;
pub mod msg;
pub mod query;
pub mod receiver;
pub mod state;
pub mod traits;

pub use cw_utils::Expiration;
use msg::{
    CollectionExtensionMsg, CollectionExtensionResponse, CollectionInfoAndExtensionResponse,
    RoyaltyInfoResponse,
};
pub use state::{Approval, Attribute, NftExtension, RoyaltyInfo};

/// Default CollectionExtension using `Option<CollectionExtension<RoyaltyInfo>>`
pub type DefaultOptionalCollectionExtension = Option<CollectionExtensionResponse<RoyaltyInfo>>;
pub type DefaultOptionalCollectionExtensionMsg =
    Option<CollectionExtensionMsg<RoyaltyInfoResponse>>;
/// Default NftExtension using `Option<NftExtension>`.
pub type DefaultOptionalNftExtension = Option<NftExtension>;
pub type DefaultOptionalNftExtensionMsg = Option<NftExtensionMsg>;

// explicit type for better distinction.
pub type NftExtensionMsg = NftExtension;
#[deprecated(since = "0.19.0", note = "Please use `NftExtension` instead")]
pub type MetaData = NftExtension;
#[deprecated(
    since = "0.19.0",
    note = "Please use `CollectionInfo<DefaultOptionalCollectionExtension>` instead"
)]
pub type ContractInfoResponse =
    CollectionInfoAndExtensionResponse<DefaultOptionalCollectionExtension>;
#[cfg(test)]
pub mod testing;

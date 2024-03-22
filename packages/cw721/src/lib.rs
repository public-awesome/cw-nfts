pub mod error;
pub mod execute;
pub mod helpers;
pub mod msg;
pub mod query;
pub mod receiver;
pub mod state;
pub mod traits;

pub use cw_utils::Expiration;
use msg::{CollectionMetadataExtensionMsg, RoyaltyInfoResponse};
pub use state::{
    Approval, Attribute, CollectionMetadataAndExtension, CollectionMetadataExtensionWrapper,
    NftMetadata, RoyaltyInfo,
};

/// Default CollectionMetadataExtension using `Option<CollectionMetadataExtension<RoyaltyInfo>>`
pub type DefaultOptionCollectionMetadataExtension =
    Option<CollectionMetadataExtensionWrapper<RoyaltyInfo>>;
pub type DefaultOptionCollectionMetadataExtensionMsg =
    Option<CollectionMetadataExtensionMsg<RoyaltyInfoResponse>>;
/// Default NftMetadataExtension using `Option<NftMetadata>`.
pub type DefaultOptionNftMetadataExtension = Option<NftMetadata>;
pub type DefaultOptionNftMetadataExtensionMsg = Option<NftMetadataMsg>;

// explicit type for better distinction.
pub type NftMetadataMsg = NftMetadata;
#[deprecated(since = "0.19.0", note = "Please use `NftMetadata` instead")]
pub type MetaData = NftMetadata;
#[deprecated(
    since = "0.19.0",
    note = "Please use `CollectionMetadata<DefaultOptionCollectionMetadataExtension>` instead"
)]
pub type ContractInfoResponse =
    CollectionMetadataAndExtension<DefaultOptionCollectionMetadataExtension>;
#[cfg(test)]
pub mod testing;

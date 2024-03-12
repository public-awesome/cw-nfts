use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, Decimal, Empty, StdResult, Storage, Timestamp};
use cw_ownable::{OwnershipStore, OWNERSHIP_KEY};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use cw_utils::Expiration;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::error::Cw721ContractError;
use crate::execute::{Update, Validate};
use crate::msg::CollectionMetadataExtensionMsg;

/// Creator owns this contract and can update collection info!
/// !!! Important note here: !!!
/// - creator is stored using using cw-ownable's OWNERSHIP singleton, so it is not stored here
/// - in release v0.18.0 it was used for minter (which is confusing), but now it is used for creator
pub const CREATOR: OwnershipStore = OwnershipStore::new(OWNERSHIP_KEY);
/// - minter is stored in the contract storage using cw_ownable::OwnershipStore (same as for OWNERSHIP but with different key)
pub const MINTER: OwnershipStore = OwnershipStore::new("collection_minter");

/// Default CollectionMetadataExtension using `Option<CollectionMetadataExtension<RoyaltyInfo>>`
pub type DefaultOptionCollectionMetadataExtension =
    Option<CollectionMetadataExtension<RoyaltyInfo>>;
/// Default NftMetadataExtension using `Option<NftMetadata>`.
pub type DefaultOptionNftMetadataExtension = Option<NftMetadata>;

// explicit type for better distinction.
pub type EmptyMsg = Empty;
pub type NftMetadataMsg = NftMetadata;

// ----------------------
// NOTE: below are max restrictions for default CollectionMetadataExtension
// This may be quite restrictive and may be increased in the future.
// Custom contracts may also provide a custom CollectionMetadataExtension.

/// Maximum length of the description field in the collection info.
pub const MAX_COLLECTION_DESCRIPTION_LENGTH: u32 = 512;
/// Max increase/decrease of of royalty share percentage.
pub const MAX_ROYALTY_SHARE_DELTA_PCT: u64 = 2;
/// Max royalty share percentage.
pub const MAX_ROYALTY_SHARE_PCT: u64 = 10;
// ----------------------

pub struct Cw721Config<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    // Message passed for updating collection info extension.
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    /// Note: replaces deprecated/legacy key "nft_info"!
    pub collection_metadata: Item<'a, CollectionMetadata<TCollectionMetadataExtension>>,
    pub token_count: Item<'a, u64>,
    /// Stored as (granter, operator) giving operator full control over granter's account.
    /// NOTE: granter is the owner, so operator has only control for NFTs owned by granter!
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub nft_info: IndexedMap<
        'a,
        &'a str,
        NftInfo<TNftMetadataExtension>,
        TokenIndexes<'a, TNftMetadataExtension>,
    >,
    pub withdraw_address: Item<'a, String>,

    pub(crate) _custom_metadata_extension_msg: PhantomData<TNftMetadataExtensionMsg>,
    pub(crate) _custom_collection_metadata_extension_msg:
        PhantomData<TCollectionMetadataExtensionMsg>,
    pub(crate) _custom_response_msg: PhantomData<TCustomResponseMsg>,
}

impl<
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Config<
        'static,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self::new(
            "collection_metadata", // Note: replaces deprecated/legacy key "nft_info"
            "num_tokens",
            "operators",
            "tokens",
            "tokens__owner",
            "withdraw_address",
        )
    }
}

impl<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    Cw721Config<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtension,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
    TNftMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionMetadataExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn new(
        collection_metadata_key: &'a str,
        token_count_key: &'a str,
        operator_key: &'a str,
        nft_info_key: &'a str,
        nft_info_owner_key: &'a str,
        withdraw_address_key: &'a str,
    ) -> Self {
        let indexes = TokenIndexes {
            owner: MultiIndex::new(token_owner_idx, nft_info_key, nft_info_owner_key),
        };
        Self {
            collection_metadata: Item::new(collection_metadata_key),
            token_count: Item::new(token_count_key),
            operators: Map::new(operator_key),
            nft_info: IndexedMap::new(nft_info_key, indexes),
            withdraw_address: Item::new(withdraw_address_key),
            _custom_metadata_extension_msg: PhantomData,
            _custom_collection_metadata_extension_msg: PhantomData,
            _custom_response_msg: PhantomData,
        }
    }

    pub fn token_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.token_count.may_load(storage)?.unwrap_or_default())
    }

    pub fn increment_tokens(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.token_count(storage)? + 1;
        self.token_count.save(storage, &val)?;
        Ok(val)
    }

    pub fn decrement_tokens(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.token_count(storage)? - 1;
        self.token_count.save(storage, &val)?;
        Ok(val)
    }
}

pub fn token_owner_idx<TNftMetadataExtension>(
    _pk: &[u8],
    d: &NftInfo<TNftMetadataExtension>,
) -> Addr {
    d.owner.clone()
}

#[cw_serde]
pub struct NftInfo<TNftMetadataExtension> {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: TNftMetadataExtension,
}

impl Update<NftInfo<NftMetadata>> for NftInfo<NftMetadata> {
    fn update(&self, msg: &NftInfo<NftMetadata>) -> Result<Self, crate::error::Cw721ContractError> {
        msg.validate()?;
        Ok(msg.clone())
    }
}

impl<TNftMetadataExtension> Validate for NftInfo<TNftMetadataExtension>
where
    TNftMetadataExtension: Validate,
{
    fn validate(&self) -> Result<(), Cw721ContractError> {
        // validate token_uri is a URL
        if let Some(token_uri) = &self.token_uri {
            Url::parse(token_uri)?;
        }
        // validate extension
        self.extension.validate()
    }
}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

pub struct TokenIndexes<'a, TNftMetadataExtension>
where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
{
    pub owner: MultiIndex<'a, Addr, NftInfo<TNftMetadataExtension>, String>,
}

impl<'a, TNftMetadataExtension> IndexList<NftInfo<TNftMetadataExtension>>
    for TokenIndexes<'a, TNftMetadataExtension>
where
    TNftMetadataExtension: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<NftInfo<TNftMetadataExtension>>> + '_> {
        let v: Vec<&dyn Index<NftInfo<TNftMetadataExtension>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

#[cw_serde]
pub struct CollectionMetadata<TCollectionMetadataExtension> {
    pub name: String,
    pub symbol: String,
    pub extension: TCollectionMetadataExtension,
    pub updated_at: Timestamp,
}

#[cw_serde]
pub struct CollectionMetadataExtension<TRoyaltyInfo> {
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub explicit_content: Option<bool>,
    pub start_trading_time: Option<Timestamp>,
    pub royalty_info: Option<TRoyaltyInfo>,
}

impl From<CollectionMetadataExtensionMsg<RoyaltyInfo>>
    for CollectionMetadataExtension<RoyaltyInfo>
{
    fn from(ext: CollectionMetadataExtensionMsg<RoyaltyInfo>) -> Self {
        Self {
            description: ext.description.unwrap_or_default(),
            image: ext.image.unwrap_or_default(),
            external_link: ext.external_link,
            explicit_content: ext.explicit_content,
            start_trading_time: ext.start_trading_time,
            royalty_info: ext.royalty_info,
        }
    }
}

impl Validate for CollectionMetadataExtension<RoyaltyInfo> {
    /// Validates only extension, not royalty info!
    fn validate(&self) -> Result<(), Cw721ContractError> {
        // check description length, must not be empty and max 512 chars
        if self.description.is_empty() {
            return Err(Cw721ContractError::CollectionDescriptionEmpty {});
        }
        if self.description.len() > MAX_COLLECTION_DESCRIPTION_LENGTH as usize {
            return Err(Cw721ContractError::CollectionDescriptionTooLong {
                max_length: MAX_COLLECTION_DESCRIPTION_LENGTH,
            });
        }

        // check images are URLs
        Url::parse(&self.image)?;
        if self.external_link.as_ref().is_some() {
            Url::parse(self.external_link.as_ref().unwrap())?;
        }
        // validate royalty info
        self.royalty_info
            .as_ref()
            .map_or(Ok(()), |r| r.validate(None))?;

        Ok(())
    }
}

impl<T> Validate for Option<T>
where
    T: Validate,
{
    fn validate(&self) -> Result<(), Cw721ContractError> {
        match self {
            Some(ext) => ext.validate(),
            // no extension, nothing to validate
            None => Ok(()),
        }
    }
}

impl Update<CollectionMetadataExtensionMsg<RoyaltyInfo>>
    for CollectionMetadataExtension<RoyaltyInfo>
{
    fn update(
        &self,
        msg: &CollectionMetadataExtensionMsg<RoyaltyInfo>,
    ) -> Result<Self, crate::error::Cw721ContractError> {
        let mut extension = self.clone();
        // validate royalty before updating
        if let Some(royalty_info) = &extension.royalty_info {
            royalty_info.validate(msg.royalty_info.clone())?;
        }
        extension.description = msg.description.clone().unwrap_or(self.description.clone());
        extension.image = msg.image.clone().unwrap_or(self.image.clone());
        extension.external_link = msg.external_link.clone().or(self.external_link.clone());
        extension.explicit_content = msg.explicit_content.or(self.explicit_content);
        extension.start_trading_time = msg.start_trading_time.or(self.start_trading_time);
        extension.royalty_info = msg.royalty_info.clone().or(self.royalty_info.clone());
        extension.validate()?;

        Ok(extension)
    }
}

impl Update<CollectionMetadataExtensionMsg<RoyaltyInfo>>
    for DefaultOptionCollectionMetadataExtension
{
    fn update(
        &self,
        msg: &CollectionMetadataExtensionMsg<RoyaltyInfo>,
    ) -> Result<Self, crate::error::Cw721ContractError> {
        match self {
            // update extension
            Some(ext) => {
                let updated = ext.update(msg)?;
                Ok(Some(updated))
            }
            // create extension
            None => {
                let extension: CollectionMetadataExtension<RoyaltyInfo> = From::from(msg.clone());
                extension.validate()?;
                Ok(Some(extension))
            }
        }
    }
}

#[cw_serde]
pub struct RoyaltyInfo {
    pub payment_address: Addr,
    pub share: Decimal,
}

impl RoyaltyInfo {
    // Compares the new royalty info with the current one and checks if the share is valid.
    pub fn validate(
        &self,
        new_royalty_info: Option<RoyaltyInfo>,
    ) -> Result<(), Cw721ContractError> {
        match new_royalty_info {
            Some(new_royalty_info) => {
                // check max share delta
                if self.share < new_royalty_info.share {
                    let share_delta = new_royalty_info.share.abs_diff(self.share);

                    if share_delta > Decimal::percent(MAX_ROYALTY_SHARE_DELTA_PCT) {
                        return Err(Cw721ContractError::InvalidRoyalties(format!(
                            "Share increase cannot be greater than {MAX_ROYALTY_SHARE_DELTA_PCT}%"
                        )));
                    }
                }
                // check max share
                if new_royalty_info.share > Decimal::percent(MAX_ROYALTY_SHARE_PCT) {
                    return Err(Cw721ContractError::InvalidRoyalties(format!(
                        "Share cannot be greater than {MAX_ROYALTY_SHARE_PCT}%"
                    )));
                }
                Ok(())
            }
            // There is no new royalty info, so we only need to check if the share is valid.
            None => {
                if self.share > Decimal::percent(MAX_ROYALTY_SHARE_PCT) {
                    return Err(Cw721ContractError::InvalidRoyalties(format!(
                        "Share cannot be greater than {MAX_ROYALTY_SHARE_PCT}%"
                    )));
                }
                Ok(())
            }
        }
    }
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct NftMetadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

impl Validate for NftMetadata {
    fn validate(&self) -> Result<(), Cw721ContractError> {
        // check URLs
        if let Some(image) = &self.image {
            Url::parse(image)?;
        }
        if let Some(url) = &self.external_url {
            Url::parse(url)?;
        }
        if let Some(animation_url) = &self.animation_url {
            Url::parse(animation_url)?;
        }
        if let Some(youtube_url) = &self.youtube_url {
            Url::parse(youtube_url)?;
        }
        // Strings must not be empty
        if let Some(image_data) = &self.image_data {
            if image_data.is_empty() {
                return Err(Cw721ContractError::MetadataImageDataEmpty {});
            }
        }
        if let Some(desc) = &self.description {
            if desc.is_empty() {
                return Err(Cw721ContractError::MetadataDescriptionEmpty {});
            }
        }
        if let Some(name) = &self.name {
            if name.is_empty() {
                return Err(Cw721ContractError::MetadataNameEmpty {});
            }
        }
        if let Some(background_color) = &self.background_color {
            if background_color.is_empty() {
                return Err(Cw721ContractError::MetadataBackgroundColorEmpty {});
            }
        }
        // check traits
        if let Some(attributes) = &self.attributes {
            for attribute in attributes {
                if attribute.trait_type.is_empty() {
                    return Err(Cw721ContractError::TraitTypeEmpty {});
                }
                if attribute.value.is_empty() {
                    return Err(Cw721ContractError::TraitValueEmpty {});
                }
                if let Some(display_type) = &attribute.display_type {
                    if display_type.is_empty() {
                        return Err(Cw721ContractError::TraitDisplayTypeEmpty {});
                    }
                }
            }
        }
        Ok(())
    }
}

impl Update<NftMetadataMsg> for NftMetadata {
    fn update(&self, msg: &NftMetadataMsg) -> Result<Self, Cw721ContractError> {
        msg.validate()?;
        let mut metadata = self.clone();
        metadata.image = msg.image.clone().or(self.image.clone());
        metadata.image_data = msg.image_data.clone().or(self.image_data.clone());
        metadata.external_url = msg.external_url.clone().or(self.external_url.clone());
        metadata.description = msg.description.clone().or(self.description.clone());
        metadata.name = msg.name.clone().or(self.name.clone());
        metadata.attributes = msg.attributes.clone().or(self.attributes.clone());
        metadata.background_color = msg
            .background_color
            .clone()
            .or(self.background_color.clone());
        metadata.animation_url = msg.animation_url.clone().or(self.animation_url.clone());
        metadata.youtube_url = msg.youtube_url.clone().or(self.youtube_url.clone());
        Ok(metadata)
    }
}

impl Update<NftMetadataMsg> for DefaultOptionNftMetadataExtension {
    fn update(&self, msg: &NftMetadataMsg) -> Result<Self, crate::error::Cw721ContractError> {
        match self {
            // update metadata
            Some(ext) => {
                let updated = ext.update(msg)?;
                Ok(Some(updated))
            }
            // create metadata
            None => {
                msg.validate()?;
                Ok(Some(msg.clone()))
            }
        }
    }
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

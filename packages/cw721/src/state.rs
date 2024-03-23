use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    from_json, to_json_binary, Addr, Binary, BlockInfo, Decimal, Deps, Env, MessageInfo, StdResult,
    Storage, Timestamp,
};
use cw_ownable::{OwnershipStore, OWNERSHIP_KEY};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use cw_utils::Expiration;
use serde::de::DeserializeOwned;
use serde::Serialize;
use url::Url;

use crate::error::Cw721ContractError;
use crate::execute::{assert_creator, assert_minter};
use crate::msg::CollectionMetadataMsg;
use crate::traits::{Cw721CustomMsg, Cw721State, FromAttributesState, ToAttributesState};
use crate::{traits::StateFactory, NftMetadataMsg};

/// Creator owns this contract and can update collection metadata!
/// !!! Important note here: !!!
/// - creator is stored using using cw-ownable's OWNERSHIP singleton, so it is not stored here
/// - in release v0.18.0 it was used for minter (which is confusing), but now it is used for creator
pub const CREATOR: OwnershipStore = OwnershipStore::new(OWNERSHIP_KEY);
/// - minter is stored in the contract storage using cw_ownable::OwnershipStore (same as for OWNERSHIP but with different key)
pub const MINTER: OwnershipStore = OwnershipStore::new("collection_minter");

// ----------------------
// NOTE: below are max restrictions for default CollectionMetadataExtension
// This may be quite restrictive and may be increased in the future.
// Custom contracts may also provide a custom CollectionMetadataExtension.

/// Maximum length of the description field in the collection metadata.
pub const MAX_COLLECTION_DESCRIPTION_LENGTH: u32 = 512;
/// Max increase/decrease of of royalty share percentage.
pub const MAX_ROYALTY_SHARE_DELTA_PCT: u64 = 2;
/// Max royalty share percentage.
pub const MAX_ROYALTY_SHARE_PCT: u64 = 10;
// ----------------------
pub const ATTRIBUTE_DESCRIPTION: &str = "description";
pub const ATTRIBUTE_IMAGE: &str = "image";
pub const ATTRIBUTE_EXTERNAL_LINK: &str = "external_link";
pub const ATTRIBUTE_EXPLICIT_CONTENT: &str = "explicit_content";
pub const ATTRIBUTE_START_TRADING_TIME: &str = "start_trading_time";
pub const ATTRIBUTE_ROYALTY_INFO: &str = "royalty_info";
// ----------------------

pub struct Cw721Config<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Message passed for updating collection metadata extension.
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
{
    /// Note: replaces deprecated/legacy key "nft_info"!
    pub collection_metadata: Item<'a, CollectionMetadata>,
    pub collection_metadata_extension: Map<'a, String, Attribute>,
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
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    > Default
    for Cw721Config<
        'static,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
{
    fn default() -> Self {
        Self::new(
            "collection_metadata", // Note: replaces deprecated/legacy key "nft_info"
            "collection_metadata_extension",
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
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
    Cw721Config<
        'a,
        TNftMetadataExtension,
        TNftMetadataExtensionMsg,
        TCollectionMetadataExtensionMsg,
        TCustomResponseMsg,
    >
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
{
    fn new(
        collection_metadata_key: &'a str,
        collection_metadata_extension_key: &'a str,
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
            collection_metadata_extension: Map::new(collection_metadata_extension_key),
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
    TNftMetadataExtension: Cw721State,
{
    pub owner: MultiIndex<'a, Addr, NftInfo<TNftMetadataExtension>, String>,
}

impl<'a, TNftMetadataExtension> IndexList<NftInfo<TNftMetadataExtension>>
    for TokenIndexes<'a, TNftMetadataExtension>
where
    TNftMetadataExtension: Cw721State,
{
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<NftInfo<TNftMetadataExtension>>> + '_> {
        let v: Vec<&dyn Index<NftInfo<TNftMetadataExtension>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

#[cw_serde]
pub struct CollectionMetadata {
    pub name: String,
    pub symbol: String,
    pub updated_at: Timestamp,
}

/// This is a wrapper around CollectionMetadata that includes the extension.
#[cw_serde]
pub struct CollectionMetadataAndExtension<TCollectionMetadataExtension> {
    pub name: String,
    pub symbol: String,
    pub extension: TCollectionMetadataExtension,
    pub updated_at: Timestamp,
}

impl<T> From<CollectionMetadataAndExtension<T>> for CollectionMetadata {
    fn from(wrapper: CollectionMetadataAndExtension<T>) -> Self {
        CollectionMetadata {
            name: wrapper.name,
            symbol: wrapper.symbol,
            updated_at: wrapper.updated_at,
        }
    }
}

impl<TCollectionMetadataExtension, TCollectionMetadataExtensionMsg>
    StateFactory<CollectionMetadataAndExtension<TCollectionMetadataExtension>>
    for CollectionMetadataMsg<TCollectionMetadataExtensionMsg>
where
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionMetadataExtension>,
{
    fn create(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&CollectionMetadataAndExtension<TCollectionMetadataExtension>>,
    ) -> Result<CollectionMetadataAndExtension<TCollectionMetadataExtension>, Cw721ContractError>
    {
        self.validate(deps, env, info, current)?;
        match current {
            // Some: update existing metadata
            Some(current) => {
                let mut updated = current.clone();
                if let Some(name) = &self.name {
                    updated.name = name.clone();
                }
                if let Some(symbol) = &self.symbol {
                    updated.symbol = symbol.clone();
                }
                let current_extension = current.extension.clone();
                let updated_extension =
                    self.extension
                        .create(deps, env, info, Some(&current_extension))?;
                updated.extension = updated_extension;
                Ok(updated)
            }
            // None: create new metadata
            None => {
                let extension = self.extension.create(deps, env, info, None)?;
                let env = env.ok_or(Cw721ContractError::NoEnv)?;
                let new = CollectionMetadataAndExtension {
                    name: self.name.clone().unwrap(),
                    symbol: self.symbol.clone().unwrap(),
                    extension,
                    updated_at: env.block.time,
                };
                Ok(new)
            }
        }
    }

    fn validate(
        &self,
        deps: Option<Deps>,
        _env: Option<&Env>,
        info: Option<&MessageInfo>,
        _current: Option<&CollectionMetadataAndExtension<TCollectionMetadataExtension>>,
    ) -> Result<(), Cw721ContractError> {
        // make sure the name and symbol are not empty
        if self.name.is_some() && self.name.clone().unwrap().is_empty() {
            return Err(Cw721ContractError::CollectionNameEmpty {});
        }
        if self.symbol.is_some() && self.symbol.clone().unwrap().is_empty() {
            return Err(Cw721ContractError::CollectionSymbolEmpty {});
        }
        let deps = deps.ok_or(Cw721ContractError::NoDeps)?;
        // collection metadata can only be updated by the creator. creator assertion is skipped for these cases:
        // - CREATOR store is empty/not initioized (like in instantiation)
        // - info is none (like in migration)
        let creator_initialized = CREATOR.item.may_load(deps.storage)?;
        if (self.name.is_some() || self.symbol.is_some())
            && creator_initialized.is_some()
            && info.is_some()
            && CREATOR
                .assert_owner(deps.storage, &info.unwrap().sender)
                .is_err()
        {
            return Err(Cw721ContractError::NotCreator {});
        }
        Ok(())
    }
}

#[cw_serde]
pub struct CollectionMetadataExtensionWrapper<TRoyaltyInfo> {
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub explicit_content: Option<bool>,
    pub start_trading_time: Option<Timestamp>,
    pub royalty_info: Option<TRoyaltyInfo>,
}

impl<TRoyaltyInfo> ToAttributesState for CollectionMetadataExtensionWrapper<TRoyaltyInfo>
where
    TRoyaltyInfo: Serialize,
{
    fn to_attributes_states(&self) -> Result<Vec<Attribute>, Cw721ContractError> {
        let attributes = vec![
            Attribute {
                key: ATTRIBUTE_DESCRIPTION.to_string(),
                value: to_json_binary(&self.description)?,
            },
            Attribute {
                key: ATTRIBUTE_IMAGE.to_string(),
                value: to_json_binary(&self.image)?,
            },
            Attribute {
                key: ATTRIBUTE_EXTERNAL_LINK.to_string(),
                value: to_json_binary(&self.external_link.clone())?,
            },
            Attribute {
                key: ATTRIBUTE_EXPLICIT_CONTENT.to_string(),
                value: to_json_binary(&self.explicit_content)?,
            },
            Attribute {
                key: ATTRIBUTE_START_TRADING_TIME.to_string(),
                value: to_json_binary(&self.start_trading_time)?,
            },
            Attribute {
                key: ATTRIBUTE_ROYALTY_INFO.to_string(),
                value: to_json_binary(&self.royalty_info)?,
            },
        ];
        // if let Some(royalty_info) = &self.royalty_info {
        //     attributes.extend(royalty_info.to_attributes_states()?);
        // } else {
        //     attributes.push(Attribute {
        //         key: ATTRIBUTE_ROYALTY_INFO.to_string(),
        //         value: to_json_binary(&None::<Option<RoyaltyInfo>>)?,
        //     });
        // }
        Ok(attributes)
    }
}

impl<TRoyaltyInfo> FromAttributesState for CollectionMetadataExtensionWrapper<TRoyaltyInfo>
where
    TRoyaltyInfo: ToAttributesState + FromAttributesState,
{
    fn from_attributes_state(attributes: &[Attribute]) -> Result<Self, Cw721ContractError> {
        let description = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_DESCRIPTION)
            .ok_or(Cw721ContractError::AttributeMissing(
                "description".to_string(),
            ))?
            .value::<String>()?;
        let image = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_IMAGE)
            .ok_or(Cw721ContractError::AttributeMissing("image".to_string()))?
            .value::<String>()?;
        let external_link = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_EXTERNAL_LINK)
            .ok_or(Cw721ContractError::AttributeMissing(
                "external link".to_string(),
            ))?
            .value::<Option<String>>()?;
        let explicit_content = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_EXPLICIT_CONTENT)
            .ok_or(Cw721ContractError::AttributeMissing(
                "explicit content".to_string(),
            ))?
            .value::<Option<bool>>()?;
        let start_trading_time = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_START_TRADING_TIME)
            .ok_or(Cw721ContractError::AttributeMissing(
                "start trading time".to_string(),
            ))?
            .value::<Option<Timestamp>>()?;

        let royalty_info = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_ROYALTY_INFO)
            .ok_or(Cw721ContractError::AttributeMissing(
                "royalty info".to_string(),
            ))?
            .value::<Option<RoyaltyInfo>>()?;

        let royalty_info = if royalty_info.is_some() {
            Some(FromAttributesState::from_attributes_state(attributes)?)
        } else {
            None
        };
        Ok(CollectionMetadataExtensionWrapper {
            description,
            image,
            external_link,
            explicit_content,
            start_trading_time,
            royalty_info,
        })
    }
}

#[cw_serde]
pub struct Attribute {
    pub key: String,
    pub value: Binary,
}

impl Attribute {
    pub fn value<T>(&self) -> Result<T, Cw721ContractError>
    where
        T: DeserializeOwned,
    {
        Ok(from_json(&self.value)?)
    }
}

impl Cw721State for CollectionMetadataExtensionWrapper<RoyaltyInfo> {}

#[cw_serde]
pub struct RoyaltyInfo {
    pub payment_address: Addr,
    pub share: Decimal,
}

impl Cw721State for RoyaltyInfo {}
impl Cw721CustomMsg for RoyaltyInfo {}

impl ToAttributesState for RoyaltyInfo {
    fn to_attributes_states(&self) -> Result<Vec<Attribute>, Cw721ContractError> {
        Ok(vec![Attribute {
            key: ATTRIBUTE_ROYALTY_INFO.to_string(),
            value: to_json_binary(&self.clone()).unwrap(),
        }])
    }
}

impl FromAttributesState for RoyaltyInfo {
    fn from_attributes_state(attributes: &[Attribute]) -> Result<Self, Cw721ContractError> {
        let royalty_info = attributes
            .iter()
            .find(|attr| attr.key == ATTRIBUTE_ROYALTY_INFO)
            .ok_or(Cw721ContractError::AttributeMissing(
                "royalty payment address".to_string(),
            ))?
            .value::<RoyaltyInfo>()?;
        Ok(royalty_info)
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

impl Cw721State for NftMetadata {}
impl Cw721CustomMsg for NftMetadata {}

impl StateFactory<NftMetadata> for NftMetadataMsg {
    fn create(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&NftMetadata>,
    ) -> Result<NftMetadata, Cw721ContractError> {
        self.validate(deps, env, info, current)?;
        match current {
            // Some: update existing metadata
            Some(current) => {
                let mut updated = current.clone();
                if let Some(image) = &self.image {
                    updated.image = Some(image.clone());
                }
                if let Some(image_data) = &self.image_data {
                    updated.image_data = Some(image_data.clone());
                }
                if let Some(external_url) = &self.external_url {
                    updated.external_url = Some(external_url.clone());
                }
                if let Some(description) = &self.description {
                    updated.description = Some(description.clone());
                }
                if let Some(name) = &self.name {
                    updated.name = Some(name.clone());
                }
                if let Some(attributes) = &self.attributes {
                    updated.attributes = Some(attributes.create(deps, env, info, None)?);
                }
                if let Some(background_color) = &self.background_color {
                    updated.background_color = Some(background_color.clone());
                }
                if let Some(animation_url) = &self.animation_url {
                    updated.animation_url = Some(animation_url.clone());
                }
                if let Some(youtube_url) = &self.youtube_url {
                    updated.youtube_url = Some(youtube_url.clone());
                }
                Ok(updated)
            }
            // None: create new metadata, note: msg is of same type as metadata, so we can clone it
            None => {
                let mut new_metadata = self.clone();
                if let Some(attributes) = &self.attributes {
                    new_metadata.attributes = Some(attributes.create(deps, env, info, None)?);
                }
                Ok(new_metadata)
            }
        }
    }

    fn validate(
        &self,
        deps: Option<Deps>,
        _env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&NftMetadata>,
    ) -> Result<(), Cw721ContractError> {
        // assert here is different to NFT Info:
        // - creator and minter can create NFT metadata
        // - only creator can update NFT metadata
        if current.is_none() {
            let deps = deps.ok_or(Cw721ContractError::NoDeps)?;
            let info = info.ok_or(Cw721ContractError::NoInfo)?;
            // current is none: minter and creator can create new NFT metadata
            let minter_check = assert_minter(deps.storage, &info.sender);
            let creator_check = assert_creator(deps.storage, &info.sender);
            if minter_check.is_err() && creator_check.is_err() {
                return Err(Cw721ContractError::NotMinterOrCreator {});
            }
        } else {
            let deps = deps.ok_or(Cw721ContractError::NoDeps)?;
            let info = info.ok_or(Cw721ContractError::NoInfo)?;
            // current is some: only creator can update NFT metadata
            assert_creator(deps.storage, &info.sender)?;
        }
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
        Ok(())
    }
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

impl StateFactory<Trait> for Trait {
    fn create(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&Trait>,
    ) -> Result<Trait, Cw721ContractError> {
        self.validate(deps, env, info, current)?;
        Ok(self.clone())
    }

    fn validate(
        &self,
        _deps: Option<Deps>,
        _env: Option<&Env>,
        _info: Option<&MessageInfo>,
        _current: Option<&Trait>,
    ) -> Result<(), Cw721ContractError> {
        if self.trait_type.is_empty() {
            return Err(Cw721ContractError::TraitTypeEmpty {});
        }
        if self.value.is_empty() {
            return Err(Cw721ContractError::TraitValueEmpty {});
        }
        if let Some(display_type) = &self.display_type {
            if display_type.is_empty() {
                return Err(Cw721ContractError::TraitDisplayTypeEmpty {});
            }
        }
        Ok(())
    }
}

impl StateFactory<Vec<Trait>> for Vec<Trait> {
    fn create(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        current: Option<&Vec<Trait>>,
    ) -> Result<Vec<Trait>, Cw721ContractError> {
        self.validate(deps, env, info, current)?;
        Ok(self.clone())
    }

    fn validate(
        &self,
        deps: Option<Deps>,
        env: Option<&Env>,
        info: Option<&MessageInfo>,
        _current: Option<&Vec<Trait>>,
    ) -> Result<(), Cw721ContractError> {
        for attribute in self {
            attribute.validate(deps, env, info, None)?;
        }
        Ok(())
    }
}

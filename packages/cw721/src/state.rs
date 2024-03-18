use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, Decimal, Deps, StdResult, Storage, Timestamp};
use cw_ownable::{OwnershipStore, OWNERSHIP_KEY};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use cw_utils::Expiration;
use url::Url;

use crate::error::Cw721ContractError;
use crate::msg::CollectionMetadataMsg;
use crate::traits::{Cw721CustomMsg, Cw721State};
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

pub struct Cw721Config<
    'a,
    // Metadata defined in NftInfo (used for mint).
    TNftMetadataExtension,
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Extension defined in CollectionMetadata.
    TCollectionMetadataExtension,
    // Message passed for updating collection metadata extension.
    TCollectionMetadataExtensionMsg,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMsg,
> where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
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
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
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
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg,
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg,
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
pub struct CollectionMetadata<TCollectionMetadataExtension> {
    pub name: String,
    pub symbol: String,
    pub extension: TCollectionMetadataExtension,
    pub updated_at: Timestamp,
}

impl<TCollectionMetadataExtension, TCollectionMetadataExtensionMsg>
    StateFactory<CollectionMetadata<TCollectionMetadataExtension>>
    for CollectionMetadataMsg<TCollectionMetadataExtensionMsg>
where
    TCollectionMetadataExtension: Cw721State,
    TCollectionMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TCollectionMetadataExtension>,
{
    fn create(
        &self,
        deps: Deps,
        env: &cosmwasm_std::Env,
        info: &cosmwasm_std::MessageInfo,
        current: Option<&CollectionMetadata<TCollectionMetadataExtension>>,
    ) -> Result<CollectionMetadata<TCollectionMetadataExtension>, Cw721ContractError> {
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
                let new = CollectionMetadata {
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
        deps: Deps,
        _env: &cosmwasm_std::Env,
        info: &cosmwasm_std::MessageInfo,
        _current: Option<&CollectionMetadata<TCollectionMetadataExtension>>,
    ) -> Result<(), Cw721ContractError> {
        // collection metadata can only be updated by the creator
        // - case 1: skip in case of init, since there is no creator yet
        let creator_initialized = CREATOR.item.may_load(deps.storage)?;
        // - case 2: check if sender is creator
        if (self.name.is_some() || self.symbol.is_some())
            && creator_initialized.is_some()
            && CREATOR.assert_owner(deps.storage, &info.sender).is_err()
        {
            return Err(Cw721ContractError::NotCollectionCreator {});
        }
        // make sure the name and symbol are not empty
        if self.name.is_some() && self.name.clone().unwrap().is_empty() {
            return Err(Cw721ContractError::CollectionNameEmpty {});
        }
        if self.symbol.is_some() && self.symbol.clone().unwrap().is_empty() {
            return Err(Cw721ContractError::CollectionSymbolEmpty {});
        }
        Ok(())
    }
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

impl Cw721State for CollectionMetadataExtension<RoyaltyInfo> {}

#[cw_serde]
pub struct RoyaltyInfo {
    pub payment_address: Addr,
    pub share: Decimal,
}

impl Cw721State for RoyaltyInfo {}
impl Cw721CustomMsg for RoyaltyInfo {}

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
        deps: Deps,
        env: &cosmwasm_std::Env,
        info: &cosmwasm_std::MessageInfo,
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
                    updated.attributes = Some(attributes.clone());
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
            None => Ok(self.clone()),
        }
    }

    fn validate(
        &self,
        _deps: Deps,
        _env: &cosmwasm_std::Env,
        _info: &cosmwasm_std::MessageInfo,
        _current: Option<&NftMetadata>,
    ) -> Result<(), Cw721ContractError> {
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

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

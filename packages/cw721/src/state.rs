use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, CustomMsg, Decimal, StdResult, Storage, Timestamp};
use cw_ownable::{OwnershipStore, OWNERSHIP_KEY};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use cw_utils::Expiration;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Creator owns this contract and can update collection info!
/// !!! Important note here: !!!
/// - creator is stored using using cw-ownable's OWNERSHIP singleton, so it is not stored here
/// - in release v0.18.0 it was used for minter (which is confusing), but now it is used for creator
pub const CREATOR: OwnershipStore = OwnershipStore::new(OWNERSHIP_KEY);
/// - minter is stored in the contract storage using cw_ownable::OwnershipStore (same as for OWNERSHIP but with different key)
pub const MINTER: OwnershipStore = OwnershipStore::new("collection_minter");

/// Default CollectionInfoExtension with RoyaltyInfo
pub type DefaultOptionCollectionInfoExtension = Option<CollectionInfoExtension<RoyaltyInfo>>;
pub type DefaultOptionMetadataExtension = Option<Metadata>;

pub struct Cw721Config<
    'a,
    TMetadataExtension,
    TCustomResponseMessage,
    TExtensionExecuteMsg,
    TCollectionInfoExtension,
> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    /// Note: replaces deprecated/legacy key "nft_info"!
    pub collection_info: Item<'a, CollectionInfo<TCollectionInfoExtension>>,
    pub token_count: Item<'a, u64>,
    /// Stored as (granter, operator) giving operator full control over granter's account.
    /// NOTE: granter is the owner, so operator has only control for NFTs owned by granter!
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub nft_info: IndexedMap<'a, &'a str, NftInfo<TMetadataExtension>, TokenIndexes<'a, TMetadataExtension>>,
    pub withdraw_address: Item<'a, String>,

    pub(crate) _custom_response: PhantomData<TCustomResponseMessage>,
    pub(crate) _custom_execute: PhantomData<TExtensionExecuteMsg>,
}

impl<TMetadataExtension, TCustomResponseMessage, TExtensionExecuteMsg, TCollectionInfoExtension> Default
    for Cw721Config<
        'static,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self::new(
            "collection_info", // Note: replaces deprecated/legacy key "nft_info"
            "num_tokens",
            "operators",
            "tokens",
            "tokens__owner",
            "withdraw_address",
        )
    }
}

impl<'a, TMetadataExtension, TCustomResponseMessage, TExtensionExecuteMsg, TCollectionInfoExtension>
    Cw721Config<
        'a,
        TMetadataExtension,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TCollectionInfoExtension,
    >
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    fn new(
        collection_info_key: &'a str,
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
            collection_info: Item::new(collection_info_key),
            token_count: Item::new(token_count_key),
            operators: Map::new(operator_key),
            nft_info: IndexedMap::new(nft_info_key, indexes),
            withdraw_address: Item::new(withdraw_address_key),
            _custom_response: PhantomData,
            _custom_execute: PhantomData,
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

pub fn token_owner_idx<TMetadataExtension>(_pk: &[u8], d: &NftInfo<TMetadataExtension>) -> Addr {
    d.owner.clone()
}

#[cw_serde]
pub struct NftInfo<TMetadataExtension> {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: TMetadataExtension,
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

pub struct TokenIndexes<'a, TMetadataExtension>
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
{
    pub owner: MultiIndex<'a, Addr, NftInfo<TMetadataExtension>, String>,
}

impl<'a, TMetadataExtension> IndexList<NftInfo<TMetadataExtension>> for TokenIndexes<'a, TMetadataExtension>
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<NftInfo<TMetadataExtension>>> + '_> {
        let v: Vec<&dyn Index<NftInfo<TMetadataExtension>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

#[cw_serde]
pub struct CollectionInfo<TCollectionInfoExtension> {
    pub name: String,
    pub symbol: String,
    pub extension: TCollectionInfoExtension,
    pub updated_at: Timestamp,
}

#[cw_serde]
pub struct CollectionInfoExtension<TRoyaltyInfo> {
    pub description: String,
    pub image: String,
    pub external_link: Option<String>,
    pub explicit_content: Option<bool>,
    pub start_trading_time: Option<Timestamp>,
    pub royalty_info: Option<TRoyaltyInfo>,
}

#[cw_serde]
pub struct RoyaltyInfo {
    pub payment_address: Addr,
    pub share: Decimal,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
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

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

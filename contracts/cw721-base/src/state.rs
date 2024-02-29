use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use cosmwasm_std::{Addr, BlockInfo, CustomMsg, StdResult, Storage};

use cw721::{CollectionInfoResponse, Cw721, Expiration};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

pub struct Cw721Contract<
    'a,
    TMetadata,
    TCustomResponseMessage,
    TExtensionExecuteMsg,
    TMetadataResponse,
> where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TMetadataResponse: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
{
    /// Note: do not use deprecated/legacy key "nft_info"!
    pub collection_info: Item<'a, CollectionInfoResponse>,
    pub token_count: Item<'a, u64>,
    /// Stored as (granter, operator) giving operator full control over granter's account
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    /// Note: do not use deprecated/legacy keys "tokens" and "tokens__owner"!
    pub nft_info: IndexedMap<'a, &'a str, NftInfo<TMetadata>, TokenIndexes<'a, TMetadata>>,
    pub withdraw_address: Item<'a, String>,

    pub(crate) _custom_response: PhantomData<TCustomResponseMessage>,
    pub(crate) _custom_query: PhantomData<TMetadataResponse>,
    pub(crate) _custom_execute: PhantomData<TExtensionExecuteMsg>,
}

// This is a signal, the implementations are in other files
impl<'a, TMetadata, TCustomResponseMessage, TExtensionExecuteMsg, TMetadataResponse>
    Cw721<TMetadata, TCustomResponseMessage>
    for Cw721Contract<
        'a,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
    >
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
{
}

impl<TMetadata, TCustomResponseMessage, TExtensionExecuteMsg, TMetadataResponse> Default
    for Cw721Contract<
        'static,
        TMetadata,
        TCustomResponseMessage,
        TExtensionExecuteMsg,
        TMetadataResponse,
    >
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
{
    fn default() -> Self {
        Self::new(
            "collection_info", // Note: do not use deprecated/legacy key "nft_info"
            "num_tokens",
            "operators",
            "nft",        // Note: do not use deprecated/legacy key "tokens"
            "nft__owner", // Note: do not use deprecated/legacy key "tokens__owner"
            "withdraw_address",
        )
    }
}

impl<'a, TMetadata, TCustomResponseMessage, TExtensionExecuteMsg, TMetadataResponse>
    Cw721Contract<'a, TMetadata, TCustomResponseMessage, TExtensionExecuteMsg, TMetadataResponse>
where
    TMetadata: Serialize + DeserializeOwned + Clone,
    TExtensionExecuteMsg: CustomMsg,
    TMetadataResponse: CustomMsg,
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
            _custom_query: PhantomData,
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

#[deprecated(since = "0.19.0", note = "Please use NftInfo")]
pub type TokenInfo<TMetadata> = NftInfo<TMetadata>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NftInfo<TMetadata> {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: TMetadata,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
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

pub struct TokenIndexes<'a, TMetadata>
where
    TMetadata: Serialize + DeserializeOwned + Clone,
{
    pub owner: MultiIndex<'a, Addr, NftInfo<TMetadata>, String>,
}

impl<'a, TMetadata> IndexList<NftInfo<TMetadata>> for TokenIndexes<'a, TMetadata>
where
    TMetadata: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<NftInfo<TMetadata>>> + '_> {
        let v: Vec<&dyn Index<NftInfo<TMetadata>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx<TMetadata>(_pk: &[u8], d: &NftInfo<TMetadata>) -> Addr {
    d.owner.clone()
}

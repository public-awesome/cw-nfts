use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use cosmwasm_std::{Addr, BlockInfo, StdResult, Storage, Uint64};

use cw1155::{ContractInfoResponse, CustomMsg, Cw1155, Expiration};
use cw_storage_plus::{Item, Map};

pub struct Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub total_token_count: Item<'a, u64>,
    /// Stored as (granter, operator) giving operator full control over granter's account
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    /// Stores the tokeninfo for each token_id
    pub tokens: Map<'a, &'a str, TokenInfo<T>>,
    /// Stores the spender info in a mapping of token_id -> owner -> OwnerInfo
    pub token_owned_info: Map<'a, (&'a str, &'a Addr), OwnerInfo>,
    pub(crate) _custom_response: PhantomData<C>,
}

// This is a signal, the implementations are in other files
impl<'a, T, C> Cw1155<T, C> for Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

impl<T, C> Default for Cw1155Contract<'static, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self::new(
            "nft_info",
            "minter",
            "total_tokens",
            "operators",
            "tokens",
            "tokens__owner",
            "tokens_owner_info",
        )
    }
}

impl<'a, T, C> Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn new(
        contract_key: &'a str,
        minter_key: &'a str,
        token_count_key: &'a str,
        total_token_count_key: &'a str,
        operator_key: &'a str,
        tokens_key: &'a str,
        tokens_owner_key: &'a str,
        tokens_owner_info_key: &'a str,
    ) -> Self {
        Self {
            contract_info: Item::new(contract_key),
            minter: Item::new(minter_key),
            token_counts: Map::new(token_count_key),
            total_token_count: Item::new(total_token_count_key),
            operators: Map::new(operator_key),
            tokens: Map::new(tokens_key),
            token_owned_info: Map::new(tokens_owner_info_key),
            _custom_response: PhantomData,
        }
    }

    pub fn get_total_token_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self
            .total_token_count
            .may_load(storage)?
            .unwrap_or_default())
    }

    pub fn increment_total_tokens(
        &self,
        storage: &mut dyn Storage,
        amount: Uint64,
    ) -> StdResult<u64> {
        let val = self.total_token_count(storage)? + amount;
        self.total_token_count.save(storage, &val)?;
        Ok(val)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo<T> {
    /// All owners that own this token
    // dev: Storing owners in token info allows us to easily check which accounts
    // have access to the token
    pub owners: Vec<Addr>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC1155
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// Total supply of the token
    pub supply: Uint64,

    /// You can add any custom metadata here when you extend cw1155-base
    pub extension: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerInfo {
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Amount of tokens owned by this owner
    pub balance: Uint64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
    /// The amount of tokens allowed to be spent by the spender
    pub allowance: Uint64,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

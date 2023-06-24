use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use cosmwasm_std::{CustomMsg, StdResult, Storage};

use cw5144::{ContractInfoResponse, Cw5144, Soul};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

pub struct Cw5144Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomMsg,
    E: CustomMsg,
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub token_count: Item<'a, u64>,
    pub tokens: IndexedMap<'a, &'a str, TokenInfo<T>, TokenIndexes<'a, T>>,

    pub(crate) _custom_response: PhantomData<C>,
    pub(crate) _custom_query: PhantomData<Q>,
    pub(crate) _custom_execute: PhantomData<E>,
}

// This is a signal, the implementations are in other files
impl<'a, T, C, E, Q> Cw5144<T, C> for Cw5144Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
}

impl<T, C, E, Q> Default for Cw5144Contract<'static, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn default() -> Self {
        Self::new(
            "nft_info",
            "num_tokens",
            "tokens",
            "tokens__owner",
        )
    }
}

impl<'a, T, C, E, Q> Cw5144Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn new(
        contract_key: &'a str,
        token_count_key: &'a str,
        tokens_key: &'a str,
        tokens_owner_key: &'a str,
    ) -> Self {
        let indexes = TokenIndexes {
            owner: MultiIndex::new(token_owner_idx, tokens_key, tokens_owner_key),
        };
        Self {
            contract_info: Item::new(contract_key),
            token_count: Item::new(token_count_key),
            tokens: IndexedMap::new(tokens_key, indexes),
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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo<T> {
    /// The owner of the newly minted SBT is now a Soul rather than `Addr`
    pub owner: Soul,

    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend cw5144-base
    pub extension: T,
}



pub struct TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub owner: MultiIndex<'a, String, TokenInfo<T>, String>,
}

impl<'a, T> IndexList<TokenInfo<T>> for TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenInfo<T>>> + '_> {
        let v: Vec<&dyn Index<TokenInfo<T>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx<T>(_pk: &[u8], d: &TokenInfo<T>) -> String {
    d.owner.to_key().clone()
}

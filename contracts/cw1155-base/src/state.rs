use cosmwasm_schema::cw_serde;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;

use cosmwasm_std::{Addr, CustomMsg, StdError, StdResult, Storage, Uint128};

use cw1155::{Balance, Expiration, TokenApproval};
use cw721::ContractInfoResponse;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

pub struct Cw1155Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub supply: Item<'a, Uint128>, // total supply of all tokens
    // key: token id
    pub token_count: Map<'a, &'a str, Uint128>, // total supply of a specific token
    // key: (owner, token id)
    pub balances: IndexedMap<'a, (Addr, String), Balance, BalanceIndexes<'a>>,
    // key: (owner, spender)
    pub approves: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    // key: (token id, owner, spender)
    pub token_approves: Map<'a, (&'a str, &'a Addr, &'a Addr), TokenApproval>,
    // key: token id
    pub tokens: Map<'a, &'a str, TokenInfo<T>>,

    pub(crate) _custom_response: PhantomData<C>,
    pub(crate) _custom_query: PhantomData<Q>,
    pub(crate) _custom_execute: PhantomData<E>,
}

impl<'a, T, C, E, Q> Default for Cw1155Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn default() -> Self {
        Self::new(
            "cw1155_contract_info",
            "tokens",
            "token_count",
            "supply",
            "balances",
            "balances__token_id",
            "approves",
            "token_approves",
        )
    }
}

impl<'a, T, C, E, Q> Cw1155Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    #[allow(clippy::too_many_arguments)]
    fn new(
        contract_info_key: &'a str,
        tokens_key: &'a str,
        token_count_key: &'a str,
        supply_key: &'a str,
        balances_key: &'a str,
        balances_token_id_key: &'a str,
        approves_key: &'a str,
        token_approves_key: &'a str,
    ) -> Self {
        let balances_indexes = BalanceIndexes {
            token_id: MultiIndex::new(
                |_, b| b.token_id.to_string(),
                balances_key,
                balances_token_id_key,
            ),
        };
        Self {
            contract_info: Item::new(contract_info_key),
            tokens: Map::new(tokens_key),
            token_count: Map::new(token_count_key),
            supply: Item::new(supply_key),
            balances: IndexedMap::new(balances_key, balances_indexes),
            approves: Map::new(approves_key),
            token_approves: Map::new(token_approves_key),
            _custom_execute: PhantomData,
            _custom_query: PhantomData,
            _custom_response: PhantomData,
        }
    }

    pub fn token_count(&self, storage: &dyn Storage, token_id: &'a str) -> StdResult<Uint128> {
        Ok(self
            .token_count
            .may_load(storage, token_id)?
            .unwrap_or_default())
    }

    pub fn increment_tokens(
        &self,
        storage: &mut dyn Storage,
        token_id: &'a str,
        amount: &Uint128,
    ) -> StdResult<Uint128> {
        // increment token count
        let val = self.token_count(storage, token_id)? + amount;
        self.token_count.save(storage, token_id, &val)?;

        // increment total supply
        self.supply.update(storage, |prev| {
            Ok::<Uint128, StdError>(prev.checked_add(*amount)?)
        })?;

        Ok(val)
    }

    pub fn decrement_tokens(
        &self,
        storage: &mut dyn Storage,
        token_id: &'a str,
        amount: &Uint128,
    ) -> StdResult<Uint128> {
        // decrement token count
        let val = self.token_count(storage, token_id)?.checked_sub(*amount)?;
        self.token_count.save(storage, token_id, &val)?;

        // decrement total supply
        self.supply.update(storage, |prev| {
            Ok::<Uint128, StdError>(prev.checked_sub(*amount)?)
        })?;

        Ok(val)
    }
}

#[cw_serde]
pub struct TokenInfo<T> {
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw1155-base
    pub extension: T,
}

pub struct BalanceIndexes<'a> {
    pub token_id: MultiIndex<'a, String, Balance, (Addr, String)>,
}

impl<'a> IndexList<Balance> for BalanceIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Balance>> + '_> {
        let v: Vec<&dyn Index<Balance>> = vec![&self.token_id];
        Box::new(v.into_iter())
    }
}

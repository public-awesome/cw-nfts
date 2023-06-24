use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomMsg, QuerierWrapper, StdResult, WasmMsg, WasmQuery,
};
use cw5144::{
    AllNftInfoResponse, ContractInfoResponse,
    NftInfoResponse, NumTokensResponse, OwnerOfResponse, TokensResponse, Soul
};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{ExecuteMsg, QueryMsg};

#[cw_serde]
pub struct Cw5144Contract<Q: CustomMsg, E: CustomMsg>(
    pub Addr,
    pub PhantomData<Q>,
    pub PhantomData<E>,
);

#[allow(dead_code)]
impl<Q: CustomMsg, E: CustomMsg> Cw5144Contract<Q, E> {
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Serialize>(&self, msg: ExecuteMsg<T, E>) -> StdResult<CosmosMsg> {
        let msg = to_binary(&msg)?;
        Ok(WasmMsg::Execute {
            contract_addr: self.addr().into(),
            msg,
            funds: vec![],
        }
        .into())
    }

    pub fn query<T: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        req: QueryMsg<Q>,
    ) -> StdResult<T> {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_binary(&req)?,
        }
        .into();
        querier.query(&query)
    }

    /*** queries ***/

    pub fn owner_of<T: Into<String>>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse> {
        let req = QueryMsg::OwnerOf {
            token_id: token_id.into(),
            include_expired: Some(include_expired),
        };
        self.query(querier, req)
    }




    pub fn num_tokens(&self, querier: &QuerierWrapper) -> StdResult<u64> {
        let req = QueryMsg::NumTokens {};
        let res: NumTokensResponse = self.query(querier, req)?;
        Ok(res.count)
    }

    /// With metadata extension
    pub fn contract_info(&self, querier: &QuerierWrapper) -> StdResult<ContractInfoResponse> {
        let req = QueryMsg::ContractInfo {};
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn sbt_info<T: Into<String>, U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
    ) -> StdResult<NftInfoResponse<U>> {
        let req = QueryMsg::SbtInfo {
            token_id: token_id.into(),
        };
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn all_sbt_info<T: Into<String>, U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<U>> {
        let req = QueryMsg::AllSbtInfo {
            token_id: token_id.into(),
            include_expired: Some(include_expired),
        };
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn tokens<T: Into<Soul>>(
        &self,
        querier: &QuerierWrapper,
        owner: T,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = QueryMsg::Tokens {
            owner: owner.into(),
            start_after,
            limit,
        };
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn all_tokens(
        &self,
        querier: &QuerierWrapper,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = QueryMsg::AllTokens { start_after, limit };
        self.query(querier, req)
    }

    /// returns true if the contract supports the metadata extension
    pub fn has_metadata(&self, querier: &QuerierWrapper) -> bool {
        self.contract_info(querier).is_ok()
    }

    /// returns true if the contract supports the enumerable extension
    pub fn has_enumerable(&self, querier: &QuerierWrapper, owner: Soul) -> bool {
        self.tokens(querier, owner, None, Some(1)).is_ok()
    }
}

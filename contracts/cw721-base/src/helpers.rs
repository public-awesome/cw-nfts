use std::marker::PhantomData;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, CosmosMsg, CustomMsg, CustomQuery, Empty, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};
use cw721::{
    AllNftInfoResponse, Approval, ApprovalResponse, ApprovalsResponse, ContractInfoResponse,
    NftInfoResponse, NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{ExecuteMsg, QueryMsg};

#[cw_serde]
pub struct Cw721Contract<
    ExtendCw721Msg: Serialize + DeserializeOwned = Empty,
    ExtendCw721Query: Serialize + DeserializeOwned = Empty,
    ModuleMsg: CustomMsg = Empty,
    ModuleQuery: CustomQuery = Empty,
>(
    pub Addr,
    pub PhantomData<ExtendCw721Msg>,
    pub PhantomData<ExtendCw721Query>,
    pub PhantomData<ModuleMsg>,
    pub PhantomData<ModuleQuery>,
);

#[allow(dead_code)]
impl<ExtendCw721Msg, ExtendCw721Query, ModuleMsg, ModuleQuery>
    Cw721Contract<ExtendCw721Msg, ExtendCw721Query, ModuleMsg, ModuleQuery>
where
    ExtendCw721Msg: Serialize + DeserializeOwned,
    ExtendCw721Query: Serialize + DeserializeOwned + schemars::JsonSchema,
    ModuleMsg: CustomMsg,
    ModuleQuery: CustomQuery,
{
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call<T: Serialize>(
        &self,
        msg: ExecuteMsg<T, ExtendCw721Msg>,
    ) -> StdResult<CosmosMsg<ModuleMsg>> {
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
        querier: &QuerierWrapper<ModuleQuery>,
        req: QueryMsg<ExtendCw721Query>,
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
        querier: &QuerierWrapper<ModuleQuery>,
        token_id: T,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse> {
        let req = QueryMsg::OwnerOf {
            token_id: token_id.into(),
            include_expired: Some(include_expired),
        };
        self.query(querier, req)
    }

    pub fn approval<T: Into<String>>(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
        token_id: T,
        spender: T,
        include_expired: Option<bool>,
    ) -> StdResult<ApprovalResponse> {
        let req = QueryMsg::Approval {
            token_id: token_id.into(),
            spender: spender.into(),
            include_expired,
        };
        let res: ApprovalResponse = self.query(querier, req)?;
        Ok(res)
    }

    pub fn approvals<T: Into<String>>(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
        token_id: T,
        include_expired: Option<bool>,
    ) -> StdResult<ApprovalsResponse> {
        let req = QueryMsg::Approvals {
            token_id: token_id.into(),
            include_expired,
        };
        let res: ApprovalsResponse = self.query(querier, req)?;
        Ok(res)
    }

    pub fn all_operators<T: Into<String>>(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
        owner: T,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Approval>> {
        let req = QueryMsg::AllOperators {
            owner: owner.into(),
            include_expired: Some(include_expired),
            start_after,
            limit,
        };
        let res: OperatorsResponse = self.query(querier, req)?;
        Ok(res.operators)
    }

    pub fn num_tokens(&self, querier: &QuerierWrapper<ModuleQuery>) -> StdResult<u64> {
        let req = QueryMsg::NumTokens {};
        let res: NumTokensResponse = self.query(querier, req)?;
        Ok(res.count)
    }

    /// With metadata extension
    pub fn contract_info(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
    ) -> StdResult<ContractInfoResponse> {
        let req = QueryMsg::ContractInfo {};
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn nft_info<T: Into<String>, U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
        token_id: T,
    ) -> StdResult<NftInfoResponse<U>> {
        let req = QueryMsg::NftInfo {
            token_id: token_id.into(),
        };
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn all_nft_info<T: Into<String>, U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
        token_id: T,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<U>> {
        let req = QueryMsg::AllNftInfo {
            token_id: token_id.into(),
            include_expired: Some(include_expired),
        };
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn tokens<T: Into<String>>(
        &self,
        querier: &QuerierWrapper<ModuleQuery>,
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
        querier: &QuerierWrapper<ModuleQuery>,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = QueryMsg::AllTokens { start_after, limit };
        self.query(querier, req)
    }

    /// returns true if the contract supports the metadata extension
    pub fn has_metadata(&self, querier: &QuerierWrapper<ModuleQuery>) -> bool {
        self.contract_info(querier).is_ok()
    }

    /// returns true if the contract supports the enumerable extension
    pub fn has_enumerable(&self, querier: &QuerierWrapper<ModuleQuery>) -> bool {
        self.tokens(querier, self.addr(), None, Some(1)).is_ok()
    }
}

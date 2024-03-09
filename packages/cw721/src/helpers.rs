use std::marker::PhantomData;

use crate::msg::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, NftInfoResponse, NumTokensResponse,
    OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use crate::msg::{Cw721ExecuteMsg, Cw721QueryMsg};
use crate::state::CollectionInfo;
use crate::Approval;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_json_binary, Addr, CosmosMsg, CustomMsg, Empty, QuerierWrapper, StdResult, WasmMsg,
    WasmQuery,
};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[cw_serde]
pub struct Cw721Contract<
    TMetadataExtension,
    TMetadataExtensionMsg: CustomMsg,
    TCollectionInfoExtension,
>(
    pub Addr,
    pub PhantomData<TMetadataExtension>,
    pub PhantomData<TMetadataExtensionMsg>,
    pub PhantomData<TCollectionInfoExtension>,
);

#[allow(dead_code)]
impl<TMetadataExtension, TMetadataExtensionMsg: CustomMsg, TCollectionInfoExtension>
    Cw721Contract<TMetadataExtension, TMetadataExtensionMsg, TCollectionInfoExtension>
where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCollectionInfoExtension: Serialize + DeserializeOwned + Clone,
{
    pub fn addr(&self) -> Addr {
        self.0.clone()
    }

    pub fn call(
        &self,
        msg: Cw721ExecuteMsg<TMetadataExtension, TMetadataExtensionMsg, TCollectionInfoExtension>,
    ) -> StdResult<CosmosMsg> {
        let msg = to_json_binary(&msg)?;
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
        req: Cw721QueryMsg<TMetadataExtension, TCollectionInfoExtension>,
    ) -> StdResult<T> {
        let query = WasmQuery::Smart {
            contract_addr: self.addr().into(),
            msg: to_json_binary(&req)?,
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
        let req = Cw721QueryMsg::OwnerOf {
            token_id: token_id.into(),
            include_expired: Some(include_expired),
        };
        self.query(querier, req)
    }

    pub fn approval<T: Into<String>>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
        spender: T,
        include_expired: Option<bool>,
    ) -> StdResult<ApprovalResponse> {
        let req = Cw721QueryMsg::Approval {
            token_id: token_id.into(),
            spender: spender.into(),
            include_expired,
        };
        let res: ApprovalResponse = self.query(querier, req)?;
        Ok(res)
    }

    pub fn approvals<T: Into<String>>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
        include_expired: Option<bool>,
    ) -> StdResult<ApprovalsResponse> {
        let req = Cw721QueryMsg::Approvals {
            token_id: token_id.into(),
            include_expired,
        };
        let res: ApprovalsResponse = self.query(querier, req)?;
        Ok(res)
    }

    pub fn all_operators<T: Into<String>>(
        &self,
        querier: &QuerierWrapper,
        owner: T,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<Approval>> {
        let req = Cw721QueryMsg::AllOperators {
            owner: owner.into(),
            include_expired: Some(include_expired),
            start_after,
            limit,
        };
        let res: OperatorsResponse = self.query(querier, req)?;
        Ok(res.operators)
    }

    pub fn num_tokens(&self, querier: &QuerierWrapper) -> StdResult<u64> {
        let req = Cw721QueryMsg::NumTokens {};
        let res: NumTokensResponse = self.query(querier, req)?;
        Ok(res.count)
    }

    /// With metadata extension
    pub fn collection_info<U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
    ) -> StdResult<CollectionInfo<U>> {
        let req = Cw721QueryMsg::GetCollectionInfo {};
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn nft_info<T: Into<String>, U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
    ) -> StdResult<NftInfoResponse<U>> {
        let req = Cw721QueryMsg::NftInfo {
            token_id: token_id.into(),
        };
        self.query(querier, req)
    }

    /// With metadata extension
    pub fn all_nft_info<T: Into<String>, U: DeserializeOwned>(
        &self,
        querier: &QuerierWrapper,
        token_id: T,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<U>> {
        let req = Cw721QueryMsg::AllNftInfo {
            token_id: token_id.into(),
            include_expired: Some(include_expired),
        };
        self.query(querier, req)
    }

    /// With enumerable extension
    pub fn tokens<T: Into<String>>(
        &self,
        querier: &QuerierWrapper,
        owner: T,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let req = Cw721QueryMsg::Tokens {
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
        let req = Cw721QueryMsg::AllTokens { start_after, limit };
        self.query(querier, req)
    }

    /// returns true if the contract supports the metadata extension
    pub fn has_metadata(&self, querier: &QuerierWrapper) -> bool {
        self.collection_info::<Empty>(querier).is_ok()
    }

    /// returns true if the contract supports the enumerable extension
    pub fn has_enumerable(&self, querier: &QuerierWrapper) -> bool {
        self.tokens(querier, self.addr(), None, Some(1)).is_ok()
    }
}

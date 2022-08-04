use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::ApprovalResponse;
use crate::{
    AllNftInfoResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CustomMsg, CustomQuery};
use cw_utils::Expiration;

pub trait Cw721<T, C, Q>: Cw721Execute<T, C, Q> + Cw721Query<T, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    Q: CustomQuery,
{
}

pub trait Cw721Execute<T, C, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    Q: CustomQuery,
{
    type Err: ToString;

    fn transfer_nft(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn send_nft(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<C>, Self::Err>;

    fn approve(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;

    fn approve_all(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<C>, Self::Err>;

    fn revoke_all(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<C>, Self::Err>;

    fn burn(
        &self,
        deps: DepsMut<Q>,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<C>, Self::Err>;
}

pub trait Cw721Query<T, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    Q: CustomQuery,
{
    // TODO: use custom error?
    // How to handle the two derived error types?

    fn contract_info(&self,deps: Deps<Q>) -> StdResult<ContractInfoResponse>;

    fn num_tokens(&self,deps: Deps<Q>) -> StdResult<NumTokensResponse>;

    fn nft_info(&self, deps: Deps<Q>, token_id: String) -> StdResult<NftInfoResponse<T>>;

    fn owner_of(
        &self,
        deps: Deps<Q>,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse>;

    fn operators(
        &self,
        deps: Deps<Q>,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse>;

    fn approval(
        &self,
        deps: Deps<Q>,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse>;

    fn approvals(
        &self,
        deps: Deps<Q>,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse>;

    fn tokens(
        &self,
        deps: Deps<Q>,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps<Q>,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_nft_info(
        &self,
        deps: Deps<Q>,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>>;
}

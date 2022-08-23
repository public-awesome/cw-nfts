use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::query::ApprovalResponse;
use crate::{
    AllNftInfoResponse, ApprovalsResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cosmwasm_std::{
    Binary, CustomMsg, CustomQuery, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw_utils::Expiration;

pub trait Cw721<T, ModuleMsg, ModuleQuery>: Cw721Execute<T, ModuleMsg, ModuleQuery> + Cw721Query<T, ModuleQuery>
where
    T: Serialize + DeserializeOwned + Clone,
    ModuleMsg: CustomMsg,
    ModuleQuery: CustomQuery,
{
}

pub trait Cw721Execute<T, ModuleMsg, ModuleQuery>
where
    T: Serialize + DeserializeOwned + Clone,
    ModuleMsg: CustomMsg,
    ModuleQuery: CustomQuery,
{
    type Err: ToString;

    fn transfer_nft(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        recipient: String,
        token_id: String,
    ) -> Result<Response<ModuleMsg>, Self::Err>;

    fn send_nft(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        contract: String,
        token_id: String,
        msg: Binary,
    ) -> Result<Response<ModuleMsg>, Self::Err>;

    fn approve(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    ) -> Result<Response<ModuleMsg>, Self::Err>;

    fn revoke(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        spender: String,
        token_id: String,
    ) -> Result<Response<ModuleMsg>, Self::Err>;

    fn approve_all(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        operator: String,
        expires: Option<Expiration>,
    ) -> Result<Response<ModuleMsg>, Self::Err>;

    fn revoke_all(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        operator: String,
    ) -> Result<Response<ModuleMsg>, Self::Err>;

    fn burn(
        &self,
        deps: DepsMut<ModuleQuery>,
        env: Env,
        info: MessageInfo,
        token_id: String,
    ) -> Result<Response<ModuleMsg>, Self::Err>;
}

pub trait Cw721Query<T, ModuleQuery>
where
    T: Serialize + DeserializeOwned + Clone,
    ModuleQuery: CustomQuery,
{
    // TODO: use custom error?
    // How to handle the two derived error types?

    fn contract_info(&self, deps: Deps<ModuleQuery>) -> StdResult<ContractInfoResponse>;

    fn num_tokens(&self, deps: Deps<ModuleQuery>) -> StdResult<NumTokensResponse>;

    fn nft_info(&self, deps: Deps<ModuleQuery>, token_id: String) -> StdResult<NftInfoResponse<T>>;

    fn owner_of(
        &self,
        deps: Deps<ModuleQuery>,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse>;

    fn operators(
        &self,
        deps: Deps<ModuleQuery>,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<OperatorsResponse>;

    fn approval(
        &self,
        deps: Deps<ModuleQuery>,
        env: Env,
        token_id: String,
        spender: String,
        include_expired: bool,
    ) -> StdResult<ApprovalResponse>;

    fn approvals(
        &self,
        deps: Deps<ModuleQuery>,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<ApprovalsResponse>;

    fn tokens(
        &self,
        deps: Deps<ModuleQuery>,
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps<ModuleQuery>,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_nft_info(
        &self,
        deps: Deps<ModuleQuery>,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>>;
}

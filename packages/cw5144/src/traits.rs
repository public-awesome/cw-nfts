use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::{
    AllNftInfoResponse, ContractInfoResponse, NftInfoResponse,
    NumTokensResponse, OwnerOfResponse, TokensResponse, query::Soul,
};
use cosmwasm_std::{CustomMsg, Deps, Env, StdResult};



pub trait Cw5144<T, C>: Cw5144Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

pub trait Cw5144Query<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    // TODO: use custom error?
    // How to handle the two derived error types?

    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse>;

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse>;

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<T>>;

    fn owner_of(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<OwnerOfResponse>;

    fn tokens(
        &self,
        deps: Deps,
        owner: Soul,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse>;

    fn all_nft_info(
        &self,
        deps: Deps,
        env: Env,
        token_id: String,
        include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>>;
}

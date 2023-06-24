use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    to_binary, Addr, Binary, CustomMsg, Deps, Env, Order, StdResult,
};

use cw5144::{
    AllNftInfoResponse, ContractInfoResponse, Cw5144Query,
    NftInfoResponse, NumTokensResponse,
    OwnerOfResponse, TokensResponse, Soul
};
use cw_storage_plus::Bound;

use crate::msg::{MinterResponse, QueryMsg};
use crate::state::{Cw5144Contract};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

impl<'a, T, C, E, Q> Cw5144Query<T> for Cw5144Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.contract_info.load(deps.storage)
    }

    fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse> {
        let count = self.token_count(deps.storage)?;
        Ok(NumTokensResponse { count })
    }

    fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(NftInfoResponse {
            token_uri: info.token_uri,
            extension: info.extension,
        })
    }

    fn owner_of(
        &self,
        deps: Deps,
        _env: Env,
        token_id: String,
        _include_expired: bool,
    ) -> StdResult<OwnerOfResponse> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(OwnerOfResponse {
            nft_address: info.owner.nft_address,
            soul_token_id: info.owner.token_id
        })
    }

    


    fn tokens(
        &self,
        deps: Deps,
        owner: Soul,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let tokens: Vec<String> = self
            .tokens
            .idx
            .owner
            .prefix(owner.to_key())
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<Vec<_>>>()?;

        Ok(TokensResponse { tokens })
    }

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

        let tokens: StdResult<Vec<String>> = self
            .tokens
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| item.map(|(k, _)| k))
            .collect();

        Ok(TokensResponse { tokens: tokens? })
    }

    fn all_nft_info(
        &self,
        deps: Deps,
        _env: Env,
        token_id: String,
        _include_expired: bool,
    ) -> StdResult<AllNftInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(AllNftInfoResponse {
            access: OwnerOfResponse {
                nft_address: info.owner.nft_address,
                soul_token_id: info.owner.token_id
            },
            info: NftInfoResponse {
                token_uri: info.token_uri,
                extension: info.extension,
            },
        })
    }
}

impl<'a, T, C, E, Q> Cw5144Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            QueryMsg::Minter {} => to_binary(&self.minter(deps)?),
            QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),
            QueryMsg::SbtInfo { token_id } => to_binary(&self.nft_info(deps, token_id)?),
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => {
                to_binary(&self.owner_of(deps, env, token_id, include_expired.unwrap_or(false))?)
            }
            QueryMsg::AllSbtInfo {
                token_id,
                include_expired,
            } => to_binary(&self.all_nft_info(
                deps,
                env,
                token_id,
                include_expired.unwrap_or(false),
            )?),
            

            QueryMsg::NumTokens {} => to_binary(&self.num_tokens(deps)?),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => to_binary(&self.tokens(deps, owner, start_after, limit)?),
            QueryMsg::AllTokens { start_after, limit } => {
                to_binary(&self.all_tokens(deps, start_after, limit)?)
            }
            QueryMsg::Ownership {} => to_binary(&Self::ownership(deps)?),
            QueryMsg::Extension { msg: _ } => Ok(Binary::default()),
        }
    }

    pub fn minter(&self, deps: Deps) -> StdResult<MinterResponse> {
        let minter = cw_ownable::get_ownership(deps.storage)?
            .owner
            .map(|a| a.into_string());

        Ok(MinterResponse { minter })
    }

    pub fn ownership(deps: Deps) -> StdResult<cw_ownable::Ownership<Addr>> {
        cw_ownable::get_ownership(deps.storage)
    }
}

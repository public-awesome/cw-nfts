use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{to_binary, Binary, Deps, Env, Order, Record, StdResult, Uint64};

use cw0::maybe_addr;
use cw1155::{
    ApprovedForAllResponse, BalanceOfResponse, ContractInfoResponse, CustomMsg, Cw1155Query,
    Expiration, NumTokensResponse, TokenInfoResponse, TokensResponse,
};
use cw_storage_plus::Bound;

use crate::msg::{MinterResponse, QueryMsg};
use crate::state::Cw1155Contract;

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 30;

impl<'a, T, C> Cw1155Query<T> for Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
        self.contract_info.load(deps.storage)
    }

    fn num_tokens(&self, token_id: String, deps: Deps) -> StdResult<NumTokensResponse> {
        let info = self.tokens.load(deps.storage, &token_id);
        match info {
            Ok(info) => Ok(NumTokensResponse {
                count: info.supply.u64(),
            }),
            Err(_) => Ok(NumTokensResponse { count: 0 }),
        }
    }

    fn token_info(&self, deps: Deps, token_id: String) -> StdResult<TokenInfoResponse<T>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(TokenInfoResponse {
            token_uri: info.token_uri,
            extension: info.extension,
        })
    }

    fn balance_of(
        &self,
        deps: Deps,
        _env: Env,
        token_id: String,
        owner: String,
    ) -> StdResult<BalanceOfResponse> {
        let owner = deps.api.addr_validate(&owner)?;
        match self
            .token_owned_info
            .load(deps.storage, (&token_id, &owner))
        {
            Ok(info) => Ok(BalanceOfResponse {
                balance: info.balance,
            }),
            Err(_) => Ok(BalanceOfResponse {
                balance: Uint64::zero(),
            }),
        }
    }

    fn all_approvals(
        &self,
        deps: Deps,
        env: Env,
        owner: String,
        include_expired: bool,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<ApprovedForAllResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start_addr = maybe_addr(deps.api, start_after)?;
        let start = start_addr.map(|addr| Bound::exclusive(addr.as_ref()));

        let owner_addr = deps.api.addr_validate(&owner)?;
        let res: StdResult<Vec<_>> = self
            .operators
            .prefix(&owner_addr)
            .range(deps.storage, start, None, Order::Ascending)
            .filter(|r| {
                include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block)
            })
            .take(limit)
            .map(parse_approval)
            .collect();
        Ok(ApprovedForAllResponse { operators: res? })
    }

    fn all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(Bound::exclusive);

        let tokens: StdResult<Vec<String>> = self
            .tokens
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| item.map(|(k, _)| String::from_utf8_lossy(&k).to_string()))
            .collect();
        Ok(TokensResponse { tokens: tokens? })
    }
}

impl<'a, T, C> Cw1155Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
    pub fn minter(&self, deps: Deps) -> StdResult<MinterResponse> {
        let minter_addr = self.minter.load(deps.storage)?;
        Ok(MinterResponse {
            minter: minter_addr.to_string(),
        })
    }

    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Minter {} => to_binary(&self.minter(deps)?),
            QueryMsg::ContractInfo {} => to_binary(&self.contract_info(deps)?),
            QueryMsg::TokenInfo { token_id } => to_binary(&self.token_info(deps, token_id)?),
            QueryMsg::BalanceOf { token_id, owner } => {
                to_binary(&self.balance_of(deps, env, token_id, owner)?)
            }
            QueryMsg::ApprovedForAll {
                owner,
                include_expired,
                start_after,
                limit,
            } => to_binary(&self.all_approvals(
                deps,
                env,
                owner,
                include_expired.unwrap_or(false),
                start_after,
                limit,
            )?),
            QueryMsg::NumTokens { token_id } => to_binary(&self.num_tokens(token_id, deps)?),
            QueryMsg::AllTokens { start_after, limit } => {
                to_binary(&self.all_tokens(deps, start_after, limit)?)
            }
        }
    }
}

fn parse_approval(item: StdResult<Record<Expiration>>) -> StdResult<cw1155::Approval> {
    item.and_then(|(k, expires)| {
        let spender = String::from_utf8(k)?;
        Ok(cw1155::Approval { spender, expires })
    })
}

use serde::de::DeserializeOwned;
use serde::Serialize;

use cosmwasm_std::{
    to_json_binary, Addr, Binary, CustomMsg, Deps, Empty, Env, Order, StdResult, Uint128,
};

use cw1155::{
    AllBalancesResponse, AllTokenInfoResponse, Approval, ApprovedForAllResponse, Balance,
    BalanceResponse, BatchBalanceResponse, Cw1155QueryMsg, Expiration, IsApprovedForAllResponse,
    NumTokensResponse, TokenInfoResponse, TokensResponse,
};
use cw721_base::Cw721Contract;
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;

use crate::state::{Cw1155Contract, TokenInfo};

const DEFAULT_LIMIT: u32 = 10;
const MAX_LIMIT: u32 = 100;

impl<'a, T, C, E, Q> Cw1155Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    pub fn query(&self, deps: Deps, env: Env, msg: Cw1155QueryMsg<Q>) -> StdResult<Binary> {
        match msg {
            Cw1155QueryMsg::Minter {} => {
                let tract = Cw721Contract::<T, Empty, Empty, Q>::default();
                to_json_binary(&tract.minter(deps)?)
            }
            Cw1155QueryMsg::BalanceOf { owner, token_id } => {
                let owner_addr = deps.api.addr_validate(&owner)?;
                let balance = self
                    .balances
                    .may_load(deps.storage, (owner_addr.clone(), token_id.clone()))?
                    .unwrap_or(Balance {
                        owner: owner_addr,
                        token_id,
                        amount: Uint128::new(0),
                    });
                to_json_binary(&BalanceResponse {
                    balance: balance.amount,
                })
            }
            Cw1155QueryMsg::AllBalances {
                token_id,
                start_after,
                limit,
            } => to_json_binary(&self.query_all_balances(deps, token_id, start_after, limit)?),
            Cw1155QueryMsg::BalanceOfBatch { owner, token_ids } => {
                let owner_addr = deps.api.addr_validate(&owner)?;
                let balances = token_ids
                    .into_iter()
                    .map(|token_id| -> StdResult<_> {
                        Ok(self
                            .balances
                            .may_load(deps.storage, (owner_addr.clone(), token_id.clone()))?
                            .unwrap_or(Balance {
                                owner: owner_addr.clone(),
                                token_id,
                                amount: Uint128::new(0),
                            })
                            .amount)
                    })
                    .collect::<StdResult<_>>()?;
                to_json_binary(&BatchBalanceResponse { balances })
            }
            Cw1155QueryMsg::NumTokens { token_id } => {
                let count = self.token_count(deps.storage, &token_id)?;
                to_json_binary(&NumTokensResponse { count })
            }
            Cw1155QueryMsg::TokenApprovals {
                owner,
                token_id,
                include_expired,
            } => {
                let owner = deps.api.addr_validate(&owner)?;
                let approvals = self
                    .token_approves
                    .prefix((&token_id, &owner))
                    .range(deps.storage, None, None, Order::Ascending)
                    .filter_map(|approval| {
                        let (_, approval) = approval.unwrap();
                        if include_expired.unwrap_or(false) || !approval.is_expired(&env) {
                            Some(approval)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                to_json_binary(&approvals)
            }
            Cw1155QueryMsg::ApprovalsForAll {
                owner,
                include_expired,
                start_after,
                limit,
            } => {
                let owner_addr = deps.api.addr_validate(&owner)?;
                let start_addr = maybe_addr(deps.api, start_after)?;
                to_json_binary(&self.query_all_approvals(
                    deps,
                    env,
                    owner_addr,
                    include_expired.unwrap_or(false),
                    start_addr,
                    limit,
                )?)
            }
            Cw1155QueryMsg::IsApprovedForAll { owner, operator } => {
                let owner_addr = deps.api.addr_validate(&owner)?;
                let operator_addr = deps.api.addr_validate(&operator)?;
                let approved =
                    self.verify_all_approval(deps.storage, &env, &owner_addr, &operator_addr);
                to_json_binary(&IsApprovedForAllResponse { approved })
            }
            Cw1155QueryMsg::TokenInfo { token_id } => {
                let token_info = self.tokens.load(deps.storage, &token_id)?;
                to_json_binary(&TokenInfoResponse::<T> {
                    token_uri: token_info.token_uri,
                    extension: token_info.extension,
                })
            }
            Cw1155QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => {
                let owner_addr = deps.api.addr_validate(&owner)?;
                to_json_binary(&self.query_owner_tokens(deps, owner_addr, start_after, limit)?)
            }
            Cw1155QueryMsg::AllTokenInfo { start_after, limit } => {
                to_json_binary(&self.query_all_token_infos(deps, start_after, limit)?)
            }
            Cw1155QueryMsg::ContractInfo {} => {
                to_json_binary(&self.contract_info.load(deps.storage)?)
            }
            Cw1155QueryMsg::Supply {} => {
                let supply = self.supply.load(deps.storage)?;
                to_json_binary(&NumTokensResponse { count: supply })
            }
            Cw1155QueryMsg::AllTokens { start_after, limit } => {
                to_json_binary(&self.query_all_tokens(deps, start_after, limit)?)
            }
            Cw1155QueryMsg::Ownership {} => {
                to_json_binary(&cw_ownable::get_ownership(deps.storage)?)
            }

            Cw1155QueryMsg::Extension { .. } => {
                unimplemented!()
            }
        }
    }
}

impl<'a, T, C, E, Q> Cw1155Contract<'a, T, C, E, Q>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
    E: CustomMsg,
    Q: CustomMsg,
{
    fn query_all_approvals(
        &self,
        deps: Deps,
        env: Env,
        owner: Addr,
        include_expired: bool,
        start_after: Option<Addr>,
        limit: Option<u32>,
    ) -> StdResult<ApprovedForAllResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(Bound::exclusive);

        let operators = self
            .approves
            .prefix(&owner)
            .range(deps.storage, start, None, Order::Ascending)
            .filter(|r| {
                include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block)
            })
            .take(limit)
            .map(build_approval)
            .collect::<StdResult<_>>()?;
        Ok(ApprovedForAllResponse { operators })
    }

    fn query_owner_tokens(
        &self,
        deps: Deps,
        owner: Addr,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

        let tokens = self
            .balances
            .prefix(owner)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<_>>()?;
        Ok(TokensResponse { tokens })
    }

    fn query_all_tokens(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));
        let tokens = self
            .tokens
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<_>>()?;
        Ok(TokensResponse { tokens })
    }

    fn query_all_token_infos(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<Vec<AllTokenInfoResponse<T>>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));
        let tokens = self
            .tokens
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                let (
                    token_id,
                    TokenInfo {
                        token_uri,
                        extension,
                    },
                ) = item.unwrap();
                AllTokenInfoResponse {
                    token_id,
                    info: TokenInfoResponse {
                        token_uri,
                        extension,
                    },
                }
            })
            .collect::<Vec<_>>();
        Ok(tokens)
    }

    fn query_all_balances(
        &self,
        deps: Deps,
        token_id: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<AllBalancesResponse> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

        let start = if let Some(start_after) = start_after {
            let start_key = (Addr::unchecked(start_after), token_id.clone());
            Some(Bound::exclusive::<(Addr, String)>(start_key))
        } else {
            None
        };

        let balances: Vec<Balance> = self
            .balances
            .idx
            .token_id
            .prefix(token_id)
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                let (_, v) = item.unwrap();
                v
            })
            .collect();

        Ok(AllBalancesResponse { balances })
    }
}

fn build_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<Approval> {
    item.map(|(addr, expires)| Approval {
        spender: addr.into(),
        expires,
    })
}

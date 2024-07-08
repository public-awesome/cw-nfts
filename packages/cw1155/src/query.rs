use cosmwasm_std::{to_json_binary, Addr, Binary, CustomMsg, Deps, Env, Order, StdResult, Uint128};
use cw721::msg::TokensResponse;
use cw721::query::Cw721Query;
use cw721::Approval;
use cw_storage_plus::Bound;
use cw_utils::{maybe_addr, Expiration};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::msg::{
    ApprovedForAllResponse, Balance, BalanceResponse, BalancesResponse, Cw1155QueryMsg,
    IsApprovedForAllResponse, OwnerToken,
};
use crate::msg::{NumTokensResponse, TokenInfoResponse};
use crate::state::Cw1155Config;

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 1000;

pub trait Cw1155Query<
    // Metadata defined in NftInfo.
    TMetadataExtension,
    // Defines for `CosmosMsg::Custom<T>` in response. Barely used, so `Empty` can be used.
    TCustomResponseMessage,
    // Message passed for updating metadata.
    TMetadataExtensionMsg,
    // Extension query message.
    TQueryExtensionMsg
>: Cw721Query<TMetadataExtension,TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg> where
    TMetadataExtension: Serialize + DeserializeOwned + Clone,
    TCustomResponseMessage: CustomMsg,
    TMetadataExtensionMsg: CustomMsg,
    TQueryExtensionMsg: Serialize + DeserializeOwned + Clone,
{
    fn query(
        &self,
        deps: Deps,
        env: Env,
        msg: Cw1155QueryMsg<TMetadataExtension, TQueryExtensionMsg>,
    ) -> StdResult<Binary> {
        match msg {
            Cw1155QueryMsg::Minter {} => {
                to_json_binary(&self.query_minter(deps.storage)?)
            }
            Cw1155QueryMsg::BalanceOf(OwnerToken { owner, token_id }) => {
                let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
                let owner_addr = deps.api.addr_validate(&owner)?;
                let balance = config
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
            Cw1155QueryMsg::BalanceOfBatch(batch) => {
                let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
                let balances = batch
                    .into_iter()
                    .map(|OwnerToken { owner, token_id }| {
                        let owner = Addr::unchecked(owner);
                        config.balances
                            .load(deps.storage, (owner.clone(), token_id.to_string()))
                            .unwrap_or(Balance {
                                owner,
                                token_id,
                                amount: Uint128::zero(),
                            })
                    })
                    .collect::<Vec<_>>();
                to_json_binary(&BalancesResponse { balances })
            }
            Cw1155QueryMsg::TokenApprovals {
                owner,
                token_id,
                include_expired,
            } => {
                let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
                let owner = deps.api.addr_validate(&owner)?;
                let approvals = config
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
                let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
                let owner_addr = deps.api.addr_validate(&owner)?;
                let operator_addr = deps.api.addr_validate(&operator)?;
                let approved =
                    config.verify_all_approval(deps.storage, &env, &owner_addr, &operator_addr);
                to_json_binary(&IsApprovedForAllResponse { approved })
            }
            Cw1155QueryMsg::TokenInfo { token_id } => {
                let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
                let token_info = config.tokens.load(deps.storage, &token_id)?;
                to_json_binary(&TokenInfoResponse::<TMetadataExtension> {
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
            Cw1155QueryMsg::ContractInfo {} => {
                to_json_binary(&self.query_collection_info(deps, env)?)
            }
            Cw1155QueryMsg::NumTokens { token_id } => {
                let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
                let count = if let Some(token_id) = token_id {
                    config.token_count(deps.storage, &token_id)?
                } else {
                    config.supply.load(deps.storage)?
                };
                to_json_binary(&NumTokensResponse { count })
            }
            Cw1155QueryMsg::AllTokens { start_after, limit } => {
                to_json_binary(&self.query_all_tokens_cw1155(deps, start_after, limit)?)
            }
            Cw1155QueryMsg::Ownership {} => {
                to_json_binary(&cw_ownable::get_ownership(deps.storage)?)
            }

            Cw1155QueryMsg::Extension { msg: ext_msg, .. } => {
                self.query_extension(deps, env, ext_msg)
            }
        }
    }

    fn query_all_approvals(
        &self,
        deps: Deps,
        env: Env,
        owner: Addr,
        include_expired: bool,
        start_after: Option<Addr>,
        limit: Option<u32>,
    ) -> StdResult<ApprovedForAllResponse> {
        let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(Bound::exclusive);

        let operators = config
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
        let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));

        let tokens = config
            .balances
            .prefix(owner)
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<_>>()?;
        Ok(TokensResponse { tokens })
    }

    fn query_all_tokens_cw1155(
        &self,
        deps: Deps,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<TokensResponse> {
        let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));
        let tokens = config
            .tokens
            .keys(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .collect::<StdResult<_>>()?;
        Ok(TokensResponse { tokens })
    }

    fn query_all_balances(
        &self,
        deps: Deps,
        token_id: String,
        start_after: Option<String>,
        limit: Option<u32>,
    ) -> StdResult<BalancesResponse> {
        let config = Cw1155Config::<TMetadataExtension, TCustomResponseMessage, TMetadataExtensionMsg, TQueryExtensionMsg>::default();
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

        let start = if let Some(start_after) = start_after {
            let start_key = (Addr::unchecked(start_after), token_id.clone());
            Some(Bound::exclusive::<(Addr, String)>(start_key))
        } else {
            None
        };

        let balances: Vec<Balance> = config
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

        Ok(BalancesResponse { balances })
    }
}

fn build_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<Approval> {
    item.map(|(addr, expires)| Approval {
        spender: addr.into(),
        expires,
    })
}

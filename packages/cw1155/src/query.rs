use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;

use cosmwasm_std::{Addr, Uint128};
use cw_ownable::cw_ownable_query;
use cw_utils::Expiration;

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw1155QueryMsg<Q: JsonSchema> {
    // cw1155
    /// Returns the current balance of the given address, 0 if unset.
    #[returns(BalanceResponse)]
    BalanceOf { owner: String, token_id: String },
    /// Returns the current balance of the given address for a batch of tokens, 0 if unset.
    #[returns(BatchBalanceResponse)]
    BalanceOfBatch {
        owner: String,
        token_ids: Vec<String>,
    },
    /// Query approved status `owner` granted to `operator`.
    #[returns(IsApprovedForAllResponse)]
    IsApprovedForAll { owner: String, operator: String },
    /// Return approvals that a token owner has
    #[returns(Vec<crate::TokenApproval>)]
    TokenApprovals {
        owner: String,
        token_id: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens.
    #[returns(ApprovedForAllResponse)]
    ApprovalsForAll {
        owner: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Returns all current balances of the given token id. Supports pagination
    #[returns(AllBalancesResponse)]
    AllBalances {
        token_id: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    Supply {},
    /// Total number of tokens issued for the token id
    #[returns(cw721::NumTokensResponse)]
    NumTokens { token_id: String },

    // cw721
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract.
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// Query Minter.
    #[returns(cw721::MinterResponse)]
    Minter {},
    /// With MetaData Extension.
    /// Query metadata of token
    #[returns(TokenInfoResponse<Q>)]
    TokenInfo { token_id: String },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(TokenInfoResponse<Q>)]
    AllTokenInfo {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Extension query
    #[returns(())]
    Extension { msg: Q },
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct AllBalancesResponse {
    pub balances: Vec<Balance>,
}

#[cw_serde]
pub struct Balance {
    pub token_id: String,
    pub owner: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct BatchBalanceResponse {
    pub balances: Vec<Uint128>,
}

#[cw_serde]
pub struct NumTokensResponse {
    pub count: Uint128,
}

#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: String,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

#[cw_serde]
pub struct ApprovedForAllResponse {
    pub operators: Vec<Approval>,
}

#[cw_serde]
pub struct IsApprovedForAllResponse {
    pub approved: bool,
}

#[cw_serde]
pub struct AllTokenInfoResponse<T> {
    pub token_id: String,
    pub info: TokenInfoResponse<T>,
}

#[cw_serde]
pub struct TokenInfoResponse<T> {
    /// Should be a url point to a json file
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw1155-base
    pub extension: T,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}

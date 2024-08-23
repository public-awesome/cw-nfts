use cosmwasm_schema::{cw_serde, QueryResponses};
use std::fmt::{Display, Formatter};

use cosmwasm_std::{Addr, Binary, Env, Uint128};
use cw721::Approval;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use cw_utils::Expiration;

#[cw_serde]
pub struct Cw1155InstantiateMsg {
    /// Name of the token contract
    pub name: String,
    /// Symbol of the token contract
    pub symbol: String,

    /// The minter is the only one who can create new tokens.
    /// This is designed for a base token platform that is controlled by an external program or
    /// contract.
    /// If None, sender is the minter.
    pub minter: Option<String>,
}

/// This is like Cw1155ExecuteMsg but we add a Mint command for a minter
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_ownable_execute]
#[cw_serde]
pub enum Cw1155ExecuteMsg<TMetadataExtension, TMetadataExtensionMsg> {
    // cw1155
    /// BatchSendFrom is a base message to move multiple types of tokens in batch,
    /// if `env.sender` is the owner or has sufficient pre-approval.
    SendBatch {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        /// if `to` is not contract, `msg` should be `None`
        to: String,
        batch: Vec<TokenAmount>,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// Mint a batch of tokens, can only be called by the contract minter
    MintBatch {
        recipient: String,
        msgs: Vec<Cw1155MintMsg<TMetadataExtension>>,
    },
    /// BatchBurn is a base message to burn multiple types of tokens in batch.
    BurnBatch {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        batch: Vec<TokenAmount>,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    // cw721
    /// SendFrom is a base message to move tokens,
    /// if `env.sender` is the owner or has sufficient pre-approval.
    Send {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        /// If `to` is not contract, `msg` should be `None`
        to: String,
        token_id: String,
        amount: Uint128,
        /// `None` means don't call the receiver interface
        msg: Option<Binary>,
    },
    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        recipient: String,
        msg: Cw1155MintMsg<TMetadataExtension>,
    },
    /// Burn is a base message to burn tokens.
    Burn {
        /// check approval if from is Some, otherwise assume sender is owner
        from: Option<String>,
        token_id: String,
        amount: Uint128,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        /// Optional amount to approve. If None, approve entire balance.
        amount: Option<Uint128>,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
        /// Optional amount to revoke. If None, revoke entire amount.
        amount: Option<Uint128>,
    },

    /// Extension msg
    Extension { msg: TMetadataExtensionMsg },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw1155QueryMsg<TMetadataExtension, TQueryExtensionMsg> {
    // cw1155
    /// Returns the current balance of the given account, 0 if unset.
    #[returns(BalanceResponse)]
    BalanceOf(OwnerToken),
    #[returns(OwnersOfResponse)]
    OwnersOf {
        token_id: String,
        limit: Option<u32>,
        start_after: Option<String>,
    },
    /// Returns the current balance of the given batch of accounts/tokens, 0 if unset.
    #[returns(BalancesResponse)]
    BalanceOfBatch(Vec<OwnerToken>),
    /// Query approved status `owner` granted to `operator`.
    #[returns(IsApprovedForAllResponse)]
    IsApprovedForAll { owner: String, operator: String },
    /// Return approvals that a token owner has
    #[returns(Vec<crate::msg::TokenApproval>)]
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
    #[returns(BalancesResponse)]
    AllBalances {
        token_id: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(NumTokensResponse)]
    NumTokens {
        token_id: Option<String>, // optional token id to get supply of, otherwise total supply
    },

    // cw721
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract.
    #[returns(cw721::state::CollectionInfo)]
    ContractInfo {},
    /// Query Minter.
    #[returns(cw721::msg::MinterResponse)]
    Minter {},
    /// With MetaData Extension.
    /// Query metadata of token
    #[returns(TokenInfoResponse<TMetadataExtension>)]
    TokenInfo { token_id: String },
    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::msg::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::msg::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Extension query
    #[returns(())]
    Extension {
        msg: TQueryExtensionMsg,
        phantom: Option<TMetadataExtension>, // dummy field to infer type
    },
}

#[cw_serde]
pub struct BalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct BalancesResponse {
    pub balances: Vec<Balance>,
}

#[cw_serde]
pub struct NumTokensResponse {
    pub count: Uint128,
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
pub struct Cw1155MintMsg<T> {
    pub token_id: String,
    /// The amount of the newly minted tokens
    pub amount: Uint128,

    /// Only first mint can set `token_uri` and `extension`
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// Any custom extension used by this contract
    pub extension: T,
}

#[cw_serde]
#[derive(Eq)]
pub struct TokenAmount {
    pub token_id: String,
    pub amount: Uint128,
}

impl Display for TokenAmount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.token_id, self.amount)
    }
}

#[cw_serde]
pub struct TokenApproval {
    pub amount: Uint128,
    pub expiration: Expiration,
}

impl TokenApproval {
    pub fn is_expired(&self, env: &Env) -> bool {
        self.expiration.is_expired(&env.block)
    }
}

#[cw_serde]
pub struct OwnerToken {
    pub owner: String,
    pub token_id: String,
}

#[cw_serde]
pub struct Balance {
    pub token_id: String,
    pub owner: Addr,
    pub amount: Uint128,
}

#[cw_serde]
pub struct OwnersOfResponse {
    pub balances: Vec<Balance>,
    pub count: u64,
}

#[cw_serde]
pub struct CollectionInfo {
    pub name: String,
    pub symbol: String,
}

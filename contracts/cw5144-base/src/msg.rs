use cosmwasm_schema::{cw_serde, QueryResponses};
use cw5144::{Soul};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use schemars::JsonSchema;

#[cw_serde]
pub struct InstantiateMsg {
    /// Name of the SBT contract
    pub name: String,
    /// Symbol of the SBT contract
    pub symbol: String,

    /// The minter is the only one who can create new SBTs.
    /// This is designed for a base SBT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

/// This is like Cw5144ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg<T, E> {

    /// Mint a new SBT, can only be called by the contract minter 
    Mint {
        /// Unique ID of the NFT
        token_id: String,
        owner: Soul,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC5144
        /// Metadata JSON Schema
        token_uri: Option<String>,
        /// Any custom extension used by this contract
        extension: T,
    },

    /// Extension msg
    Extension { msg: E },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<Q: JsonSchema> {
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw5144::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Total number of tokens issued
    #[returns(cw5144::NumTokensResponse)]
    NumTokens {},
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw5144::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC5144 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw5144::NftInfoResponse<Q>)]
    SbtInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw5144::AllNftInfoResponse<Q>)]
    AllSbtInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw5144::TokensResponse)]
    Tokens {
        owner: Soul,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw5144::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Return the minter
    #[returns(MinterResponse)]
    Minter {},

    /// Extension query
    #[returns(())]
    Extension { msg: Q },
}

/// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

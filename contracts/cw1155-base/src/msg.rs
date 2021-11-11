use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, Uint64};
use cw1155::Expiration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Name of the contract
    pub name: String,
    /// Symbol of the contract
    pub symbol: String,

    /// The minter is the only one who can create new tokens.
    /// This is designed for a base 1155 contract that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom implementations.
    pub minter: String,
}

/// This is like Cw1155ExecuteMsg but we add a Mint command for an owner
/// to make this stand-alone. You will likely want to remove mint and
/// use other control logic in any contract that inherits this.
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg<T> {
    /// Transfer is a base message to move a token to another account without triggering actions
    /// owner is optional, if not specified its assumed that the sender is the owner.
    Transfer {
        recipient: String,
        token_id: String,
        amount: Uint64,
        owner: Option<String>,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    /// /// owner is optional, if not specified its assumed that the sender is the owner.
    Send {
        contract: String,
        token_id: String,
        amount: Uint64,
        owner: Option<String>,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    IncreaseAllowance {
        spender: String,
        token_id: String,
        amount: Uint64,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    DecreaseAllowance {
        spender: String,
        token_id: String,
        amount: Uint64,
        expires: Option<Expiration>,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll { operator: String },

    /// Mint a new token, can only be called by the contract minter
    Mint(MintMsg<T>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg<T> {
    /// Unique ID of the token
    pub token_id: String,
    /// The owner of the newly minter token
    pub owner: String,
    /// Universal resource identifier for this token
    /// Should point to a JSON file that conforms to the ERC1155
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// The number of tokens to mint
    pub amount: Uint64,
    /// Any custom extension used by this contract
    pub extension: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the current balance of the given address, 0 if unset and
    /// error if token does not exist
    /// Return type: BalanceOfResponse
    BalanceOf {
        token_id: String,
        // Address of the owner.
        owner: String,
    },
    /// List all operators that can access all of the owner's tokens
    /// Return type: `ApprovedForAllResponse`
    ApprovedForAll {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    NumTokens {
        token_id: String,
    },
    /// With MetaData Extension.
    /// Returns top-level metadata about the contract: `ContractInfoResponse`
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC1155 Metadata JSON Schema*
    /// but directly from the contract: `TokenInfoResponse`
    TokenInfo {
        token_id: String,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    /// Return type: TokensResponse.
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    // Return the minter
    Minter {},
}

/// Shows who can mint these tokens
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterResponse {
    pub minter: String,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};


#[cw_serde]
pub struct Soul {
    pub nft_address: Addr,
    pub token_id: Uint128,
}
impl Soul {
    pub fn to_key(&self) -> String {
        format!("{}_{}", self.nft_address, self.token_id)  // for example
    }
}

#[cw_serde]
pub enum Cw5144QueryMsg {
    /// Return the owner of the given token, error if token does not exist
    /// Return type: OwnerOfResponse
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Total number of tokens issued
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract: `ContractInfoResponse`
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract: `NftInfoResponse`
    SbtInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients: `AllNftInfo`
    AllSbtInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    /// Return type: TokensResponse.
    Tokens {
        owner: Soul,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    /// Return type: TokensResponse.
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub nft_address: Addr,
    pub soul_token_id: Uint128
}



#[cw_serde]
pub struct NumTokensResponse {
    pub count: u64,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub name: String,
    pub symbol: String,
}

#[cw_serde]
pub struct NftInfoResponse<T> {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    pub extension: T,
}

#[cw_serde]
pub struct AllNftInfoResponse<T> {
    /// Who can transfer the token
    pub access: OwnerOfResponse,
    /// Data on the token itself,
    pub info: NftInfoResponse<T>,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_after` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}

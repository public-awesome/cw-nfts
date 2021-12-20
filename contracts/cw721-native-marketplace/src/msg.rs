use crate::state::{Config, Token};

use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub admin: String,
    pub nft_addr: String,
    pub allowed_native: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Buy buys nft using native token
    Buy {
        /// recipient if None, tx sender is used
        recipient: Option<String>,
        token_id: String,
    },
    /// ListTokens registers or relists tokens
    ListTokens {
        tokens: Vec<Token>,
    },
    /// Delist tokens removes tokens from marketplace
    DelistTokens {
        tokens: Vec<String>,
    },
    UpdatePrice {
        token: String,
        price: Uint128,
    },
    UpdateConfig {
        admin: Option<String>,
        nft_addr: Option<String>,
        allowed_native: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    Token {
        id: String,
    },
    RangeTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    ListTokens {
        ids: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokenResponse {
    pub token: Token,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TokensResponse {
    pub tokens: Vec<Token>,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, BlockInfo, Empty, Response, StdResult};
use cw721::{cw721_interface, ContractInfoResponse, Expiration};
use cw_ownable::Ownership;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use sylvia::contract;
use sylvia::types::{ExecCtx, InstantiateCtx, QueryCtx};

use crate::ContractError;

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:cw721-sylvia-base";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DEFAULT_LIMIT: u32 = 10;
pub const MAX_LIMIT: u32 = 100;

// TODO should this just be in cw721_interface along with the other responses?
#[cw_serde]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

/// TODO move to a responses file?
/// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

// TODO what do we want to do with extensions? They seem to be unnessary now?
// TODO kill extensions
#[cw_serde]
pub struct TokenInfo {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    pub extension: Empty,
}

pub fn token_owner_idx(_pk: &[u8], d: &TokenInfo) -> Addr {
    d.owner.clone()
}

/// Indexed map for NFT tokens by owner
pub struct TokenIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, TokenInfo, String>,
}
impl<'a> IndexList<TokenInfo> for TokenIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ (dyn Index<TokenInfo> + '_)> + '_> {
        let v: Vec<&dyn Index<TokenInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub struct Cw721Contract<'a> {
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub token_count: Item<'a, u64>,
    /// Stored as (granter, operator) giving operator full control over granter's account
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub tokens: IndexedMap<'a, &'a str, TokenInfo, TokenIndexes<'a>>,
}

#[cw_serde]
pub struct InstantiateMsgData {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: String,
}

#[cfg_attr(not(feature = "library"), sylvia::entry_points)]
#[contract]
#[error(ContractError)]
#[messages(cw721_interface as Cw721Interface)]
impl Cw721Contract<'_> {
    pub fn new() -> Self {
        let indexes = TokenIndexes {
            owner: MultiIndex::new(token_owner_idx, "tokens", "tokens__owner"),
        };
        Self {
            contract_info: Item::new("contract_info"),
            token_count: Item::new("token_count"),
            operators: Map::new("operators"),
            tokens: IndexedMap::new("tokens", indexes),
        }
    }

    #[msg(instantiate)]
    pub fn instantiate(
        &self,
        ctx: InstantiateCtx,
        data: InstantiateMsgData,
    ) -> StdResult<Response> {
        cw2::set_contract_version(ctx.deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let info = ContractInfoResponse {
            name: data.name,
            symbol: data.symbol,
        };
        self.contract_info.save(ctx.deps.storage, &info)?;

        cw_ownable::initialize_owner(ctx.deps.storage, ctx.deps.api, Some(&data.minter))?;

        Ok(Response::new())
    }

    #[msg(exec)]
    pub fn mint(
        &self,
        ctx: ExecCtx,
        token_id: String,
        owner: String,
        token_uri: Option<String>,
    ) -> Result<Response, ContractError> {
        cw_ownable::assert_owner(ctx.deps.storage, &ctx.info.sender)?;

        // create the token
        let token = TokenInfo {
            owner: ctx.deps.api.addr_validate(&owner)?,
            approvals: vec![],
            token_uri,
            extension: Empty {},
        };
        self.tokens
            .update(ctx.deps.storage, &token_id, |old| match old {
                Some(_) => Err(ContractError::Claimed {}),
                None => Ok(token),
            })?;

        self.increment_tokens(ctx.deps.storage)?;

        Ok(Response::new()
            .add_attribute("action", "mint")
            .add_attribute("minter", ctx.info.sender)
            .add_attribute("owner", owner)
            .add_attribute("token_id", token_id))
    }

    #[msg(exec)]
    pub fn update_ownership(
        &self,
        ctx: ExecCtx,
        action: cw_ownable::Action,
    ) -> Result<Response, ContractError> {
        let ownership =
            cw_ownable::update_ownership(ctx.deps, &ctx.env.block, &ctx.info.sender, action)?;
        Ok(Response::new().add_attributes(ownership.into_attributes()))
    }

    #[msg(query)]
    pub fn minter(&self, ctx: QueryCtx) -> StdResult<MinterResponse> {
        let minter = cw_ownable::get_ownership(ctx.deps.storage)?
            .owner
            .map(|a| a.into_string());

        Ok(MinterResponse { minter })
    }

    #[msg(query)]
    pub fn ownership(&self, ctx: QueryCtx) -> StdResult<Ownership<Addr>> {
        cw_ownable::get_ownership(ctx.deps.storage)
    }
}

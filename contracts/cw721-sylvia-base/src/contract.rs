use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    to_binary, Addr, Binary, BlockInfo, CustomMsg, Deps, Env, Order, Response, StdError, StdResult,
};
use cw721::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, ContractInfoResponse, Cw721Query,
    Expiration, NftInfoResponse, NumTokensResponse, OperatorResponse, OperatorsResponse,
    OwnerOfResponse, TokensResponse,
};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use cw_utils::maybe_addr;
use sylvia::types::InstantiateCtx;
use sylvia::{contract, entry_points};

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

/// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

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

#[entry_points]
#[contract]
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
        let info = ContractInfoResponse {
            name: data.name,
            symbol: data.symbol,
        };
        self.contract_info.save(ctx.deps.storage, &info)?;

        cw_ownable::initialize_owner(ctx.deps.storage, ctx.deps.api, Some(&data.minter))?;

        Ok(Response::new())
    }

    // Execute msgs
    // // TODO convert to ExecCtx
    // // TODO remove extension
    // pub fn mint(
    //     &self,
    //     deps: DepsMut,
    //     info: MessageInfo,
    //     token_id: String,
    //     owner: String,
    //     token_uri: Option<String>,
    //     extension: T,
    // ) -> Result<Response<C>, ContractError> {
    //     cw_ownable::assert_owner(deps.storage, &info.sender)?;

    //     // create the token
    //     let token = TokenInfo {
    //         owner: deps.api.addr_validate(&owner)?,
    //         approvals: vec![],
    //         token_uri,
    //         extension,
    //     };
    //     self.tokens
    //         .update(deps.storage, &token_id, |old| match old {
    //             Some(_) => Err(ContractError::Claimed {}),
    //             None => Ok(token),
    //         })?;

    //     self.increment_tokens(deps.storage)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "mint")
    //         .add_attribute("minter", info.sender)
    //         .add_attribute("owner", owner)
    //         .add_attribute("token_id", token_id))
    // }

    // pub fn update_ownership(
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     action: cw_ownable::Action,
    // ) -> Result<Response<C>, ContractError> {
    //     let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    //     Ok(Response::new().add_attributes(ownership.into_attributes()))
    // }

    // fn transfer_nft(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     recipient: String,
    //     token_id: String,
    // ) -> Result<Response<C>, ContractError> {
    //     self._transfer_nft(deps, &env, &info, &recipient, &token_id)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "transfer_nft")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("recipient", recipient)
    //         .add_attribute("token_id", token_id))
    // }

    // fn send_nft(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     contract: String,
    //     token_id: String,
    //     msg: Binary,
    // ) -> Result<Response<C>, ContractError> {
    //     // Transfer token
    //     self._transfer_nft(deps, &env, &info, &contract, &token_id)?;

    //     let send = Cw721ReceiveMsg {
    //         sender: info.sender.to_string(),
    //         token_id: token_id.clone(),
    //         msg,
    //     };

    //     // Send message
    //     Ok(Response::new()
    //         .add_message(send.into_cosmos_msg(contract.clone())?)
    //         .add_attribute("action", "send_nft")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("recipient", contract)
    //         .add_attribute("token_id", token_id))
    // }

    // fn approve(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     spender: String,
    //     token_id: String,
    //     expires: Option<Expiration>,
    // ) -> Result<Response<C>, ContractError> {
    //     self._update_approvals(deps, &env, &info, &spender, &token_id, true, expires)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "approve")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("spender", spender)
    //         .add_attribute("token_id", token_id))
    // }

    // fn revoke(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     spender: String,
    //     token_id: String,
    // ) -> Result<Response<C>, ContractError> {
    //     self._update_approvals(deps, &env, &info, &spender, &token_id, false, None)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "revoke")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("spender", spender)
    //         .add_attribute("token_id", token_id))
    // }

    // fn approve_all(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     operator: String,
    //     expires: Option<Expiration>,
    // ) -> Result<Response<C>, ContractError> {
    //     // reject expired data as invalid
    //     let expires = expires.unwrap_or_default();
    //     if expires.is_expired(&env.block) {
    //         return Err(ContractError::Expired {});
    //     }

    //     // set the operator for us
    //     let operator_addr = deps.api.addr_validate(&operator)?;
    //     self.operators
    //         .save(deps.storage, (&info.sender, &operator_addr), &expires)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "approve_all")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("operator", operator))
    // }

    // fn revoke_all(
    //     &self,
    //     deps: DepsMut,
    //     _env: Env,
    //     info: MessageInfo,
    //     operator: String,
    // ) -> Result<Response<C>, ContractError> {
    //     let operator_addr = deps.api.addr_validate(&operator)?;
    //     self.operators
    //         .remove(deps.storage, (&info.sender, &operator_addr));

    //     Ok(Response::new()
    //         .add_attribute("action", "revoke_all")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("operator", operator))
    // }

    // fn burn(
    //     &self,
    //     deps: DepsMut,
    //     env: Env,
    //     info: MessageInfo,
    //     token_id: String,
    // ) -> Result<Response<C>, ContractError> {
    //     let token = self.tokens.load(deps.storage, &token_id)?;
    //     self.check_can_send(deps.as_ref(), &env, &info, &token)?;

    //     self.tokens.remove(deps.storage, &token_id)?;
    //     self.decrement_tokens(deps.storage)?;

    //     Ok(Response::new()
    //         .add_attribute("action", "burn")
    //         .add_attribute("sender", info.sender)
    //         .add_attribute("token_id", token_id))
    // }

    // pub fn _transfer_nft(
    //     &self,
    //     deps: DepsMut,
    //     env: &Env,
    //     info: &MessageInfo,
    //     recipient: &str,
    //     token_id: &str,
    // ) -> Result<TokenInfo<T>, ContractError> {
    //     let mut token = self.tokens.load(deps.storage, token_id)?;
    //     // ensure we have permissions
    //     self.check_can_send(deps.as_ref(), env, info, &token)?;
    //     // set owner and remove existing approvals
    //     token.owner = deps.api.addr_validate(recipient)?;
    //     token.approvals = vec![];
    //     self.tokens.save(deps.storage, token_id, &token)?;
    //     Ok(token)
    // }

    // #[allow(clippy::too_many_arguments)]
    // pub fn _update_approvals(
    //     &self,
    //     deps: DepsMut,
    //     env: &Env,
    //     info: &MessageInfo,
    //     spender: &str,
    //     token_id: &str,
    //     // if add == false, remove. if add == true, remove then set with this expiration
    //     add: bool,
    //     expires: Option<Expiration>,
    // ) -> Result<TokenInfo<T>, ContractError> {
    //     let mut token = self.tokens.load(deps.storage, token_id)?;
    //     // ensure we have permissions
    //     self.check_can_approve(deps.as_ref(), env, info, &token)?;

    //     // update the approval list (remove any for the same spender before adding)
    //     let spender_addr = deps.api.addr_validate(spender)?;
    //     token.approvals.retain(|apr| apr.spender != spender_addr);

    //     // only difference between approve and revoke
    //     if add {
    //         // reject expired data as invalid
    //         let expires = expires.unwrap_or_default();
    //         if expires.is_expired(&env.block) {
    //             return Err(ContractError::Expired {});
    //         }
    //         let approval = Approval {
    //             spender: spender_addr,
    //             expires,
    //         };
    //         token.approvals.push(approval);
    //     }

    //     self.tokens.save(deps.storage, token_id, &token)?;

    //     Ok(token)
    // }

    // /// returns true iff the sender can execute approve or reject on the contract
    // pub fn check_can_approve(
    //     &self,
    //     deps: Deps,
    //     env: &Env,
    //     info: &MessageInfo,
    //     token: &TokenInfo<T>,
    // ) -> Result<(), ContractError> {
    //     // owner can approve
    //     if token.owner == info.sender {
    //         return Ok(());
    //     }
    //     // operator can approve
    //     let op = self
    //         .operators
    //         .may_load(deps.storage, (&token.owner, &info.sender))?;
    //     match op {
    //         Some(ex) => {
    //             if ex.is_expired(&env.block) {
    //                 Err(ContractError::Ownership(OwnershipError::NotOwner))
    //             } else {
    //                 Ok(())
    //             }
    //         }
    //         None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
    //     }
    // }

    // /// returns true iff the sender can transfer ownership of the token
    // pub fn check_can_send(
    //     &self,
    //     deps: Deps,
    //     env: &Env,
    //     info: &MessageInfo,
    //     token: &TokenInfo<T>,
    // ) -> Result<(), ContractError> {
    //     // owner can send
    //     if token.owner == info.sender {
    //         return Ok(());
    //     }

    //     // any non-expired token approval can send
    //     if token
    //         .approvals
    //         .iter()
    //         .any(|apr| apr.spender == info.sender && !apr.is_expired(&env.block))
    //     {
    //         return Ok(());
    //     }

    //     // operator can send
    //     let op = self
    //         .operators
    //         .may_load(deps.storage, (&token.owner, &info.sender))?;
    //     match op {
    //         Some(ex) => {
    //             if ex.is_expired(&env.block) {
    //                 Err(ContractError::Ownership(OwnershipError::NotOwner))
    //             } else {
    //                 Ok(())
    //             }
    //         }
    //         None => Err(ContractError::Ownership(OwnershipError::NotOwner)),
    //     }
    // }

    // // QUERIES
    // // TODO move me to a separate file
    // // TODO use QueryCtx
    // // TODO add interfaces to cw721 package
    // #[msg(query)]
    // pub fn contract_info(&self, deps: Deps) -> StdResult<ContractInfoResponse> {
    //     self.contract_info.load(deps.storage)
    // }

    // #[msg(query)]
    // pub fn num_tokens(&self, deps: Deps) -> StdResult<NumTokensResponse> {
    //     let count = self.token_count(deps.storage)?;
    //     Ok(NumTokensResponse { count })
    // }

    // #[msg(query)]
    // pub fn nft_info(&self, deps: Deps, token_id: String) -> StdResult<NftInfoResponse {
    //     let info = self.tokens.load(deps.storage, &token_id)?;
    //     Ok(NftInfoResponse {
    //         token_uri: info.token_uri,
    //         extension: info.extension,
    //     })
    // }

    // #[msg(query)]
    // pub fn owner_of(
    //     &self,
    //     deps: Deps,
    //     env: Env,
    //     token_id: String,
    //     include_expired: bool,
    // ) -> StdResult<OwnerOfResponse> {
    //     let info = self.tokens.load(deps.storage, &token_id)?;
    //     Ok(OwnerOfResponse {
    //         owner: info.owner.to_string(),
    //         approvals: humanize_approvals(&env.block, &info, include_expired),
    //     })
    // }

    // /// operator returns the approval status of an operator for a given owner if exists
    // #[msg(query)]
    // pub fn operator(
    //     &self,
    //     deps: Deps,
    //     env: Env,
    //     owner: String,
    //     operator: String,
    //     include_expired: bool,
    // ) -> StdResult<OperatorResponse> {
    //     let owner_addr = deps.api.addr_validate(&owner)?;
    //     let operator_addr = deps.api.addr_validate(&operator)?;

    //     let info = self
    //         .operators
    //         .may_load(deps.storage, (&owner_addr, &operator_addr))?;

    //     if let Some(expires) = info {
    //         if !include_expired && expires.is_expired(&env.block) {
    //             return Err(StdError::not_found("Approval not found"));
    //         }

    //         return Ok(OperatorResponse {
    //             approval: cw721::Approval {
    //                 spender: operator,
    //                 expires,
    //             },
    //         });
    //     }

    //     Err(StdError::not_found("Approval not found"))
    // }

    // /// operators returns all operators owner given access to
    // #[msg(query)]
    // pub fn operators(
    //     &self,
    //     deps: Deps,
    //     env: Env,
    //     owner: String,
    //     include_expired: bool,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // ) -> StdResult<OperatorsResponse> {
    //     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //     let start_addr = maybe_addr(deps.api, start_after)?;
    //     let start = start_addr.as_ref().map(Bound::exclusive);

    //     let owner_addr = deps.api.addr_validate(&owner)?;
    //     let res: StdResult<Vec<_>> = self
    //         .operators
    //         .prefix(&owner_addr)
    //         .range(deps.storage, start, None, Order::Ascending)
    //         .filter(|r| {
    //             include_expired || r.is_err() || !r.as_ref().unwrap().1.is_expired(&env.block)
    //         })
    //         .take(limit)
    //         .map(self.parse_approval)
    //         .collect();
    //     Ok(OperatorsResponse { operators: res? })
    // }

    // #[msg(query)]
    // pub fn approval(
    //     &self,
    //     deps: Deps,
    //     env: Env,
    //     token_id: String,
    //     spender: String,
    //     include_expired: bool,
    // ) -> StdResult<ApprovalResponse> {
    //     let token = self.tokens.load(deps.storage, &token_id)?;

    //     // token owner has absolute approval
    //     if token.owner == spender {
    //         let approval = cw721::Approval {
    //             spender: token.owner.to_string(),
    //             expires: Expiration::Never {},
    //         };
    //         return Ok(ApprovalResponse { approval });
    //     }

    //     let filtered: Vec<_> = token
    //         .approvals
    //         .into_iter()
    //         .filter(|t| t.spender == spender)
    //         .filter(|t| include_expired || !t.is_expired(&env.block))
    //         .map(|a| cw721::Approval {
    //             spender: a.spender.into_string(),
    //             expires: a.expires,
    //         })
    //         .collect();

    //     if filtered.is_empty() {
    //         return Err(StdError::not_found("Approval not found"));
    //     }
    //     // we expect only one item
    //     let approval = filtered[0].clone();

    //     Ok(ApprovalResponse { approval })
    // }

    // /// approvals returns all approvals owner given access to
    // #[msg(query)]
    // pub fn approvals(
    //     &self,
    //     deps: Deps,
    //     env: Env,
    //     token_id: String,
    //     include_expired: bool,
    // ) -> StdResult<ApprovalsResponse> {
    //     let token = self.tokens.load(deps.storage, &token_id)?;
    //     let approvals: Vec<_> = token
    //         .approvals
    //         .into_iter()
    //         .filter(|t| include_expired || !t.is_expired(&env.block))
    //         .map(|a| cw721::Approval {
    //             spender: a.spender.into_string(),
    //             expires: a.expires,
    //         })
    //         .collect();

    //     Ok(ApprovalsResponse { approvals })
    // }

    // #[msg(query)]
    // pub fn tokens(
    //     &self,
    //     deps: Deps,
    //     owner: String,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // ) -> StdResult<TokensResponse> {
    //     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //     let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    //     let owner_addr = deps.api.addr_validate(&owner)?;
    //     let tokens: Vec<String> = self
    //         .tokens
    //         .idx
    //         .owner
    //         .prefix(owner_addr)
    //         .keys(deps.storage, start, None, Order::Ascending)
    //         .take(limit)
    //         .collect::<StdResult<Vec<_>>>()?;

    //     Ok(TokensResponse { tokens })
    // }

    // #[msg(query)]
    // pub fn all_tokens(
    //     &self,
    //     deps: Deps,
    //     start_after: Option<String>,
    //     limit: Option<u32>,
    // ) -> StdResult<TokensResponse> {
    //     let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //     let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    //     let tokens: StdResult<Vec<String>> = self
    //         .tokens
    //         .range(deps.storage, start, None, Order::Ascending)
    //         .take(limit)
    //         .map(|item| item.map(|(k, _)| k))
    //         .collect();

    //     Ok(TokensResponse { tokens: tokens? })
    // }

    // #[msg(query)]
    // pub fn all_nft_info(
    //     &self,
    //     deps: Deps,
    //     env: Env,
    //     token_id: String,
    //     include_expired: bool,
    // ) -> StdResult<AllNftInfoResponse> {
    //     let info = self.tokens.load(deps.storage, &token_id)?;
    //     Ok(AllNftInfoResponse {
    //         access: OwnerOfResponse {
    //             owner: info.owner.to_string(),
    //             approvals: self.humanize_approvals(&env.block, &info, include_expired),
    //         },
    //         info: NftInfoResponse {
    //             token_uri: info.token_uri,
    //             extension: info.extension,
    //         },
    //     })
    // }

    // #[msg(query)]
    // pub fn minter(&self, deps: Deps) -> StdResult<MinterResponse> {
    //     let minter = cw_ownable::get_ownership(deps.storage)?
    //         .owner
    //         .map(|a| a.into_string());

    //     Ok(MinterResponse { minter })
    // }

    // #[msg(query)]
    // pub fn ownership(deps: Deps) -> StdResult<cw_ownable::Ownership<Addr>> {
    //     cw_ownable::get_ownership(deps.storage)
    // }

    // fn parse_approval(item: StdResult<(Addr, Expiration)>) -> StdResult<cw721::Approval> {
    //     item.map(|(spender, expires)| cw721::Approval {
    //         spender: spender.to_string(),
    //         expires,
    //     })
    // }

    // fn humanize_approvals(
    //     block: &BlockInfo,
    //     info: &TokenInfo,
    //     include_expired: bool,
    // ) -> Vec<cw721::Approval> {
    //     info.approvals
    //         .iter()
    //         .filter(|apr| include_expired || !apr.is_expired(block))
    //         .map(humanize_approval)
    //         .collect()
    // }

    // fn humanize_approval(approval: &Approval) -> cw721::Approval {
    //     cw721::Approval {
    //         spender: approval.spender.to_string(),
    //         expires: approval.expires,
    //     }
    // }
}

pub fn token_owner_idx(_pk: &[u8], d: &TokenInfo) -> Addr {
    d.owner.clone()
}

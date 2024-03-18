use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Coin, Decimal, Deps, Env, MessageInfo, Timestamp};
use cw_ownable::{Action, Ownership};
use cw_utils::Expiration;
use url::Url;

use crate::error::Cw721ContractError;
use crate::state::{
    CollectionMetadata, NftInfo, CREATOR, MAX_COLLECTION_DESCRIPTION_LENGTH,
    MAX_ROYALTY_SHARE_DELTA_PCT, MAX_ROYALTY_SHARE_PCT, MINTER,
};
use crate::traits::{Cw721CustomMsg, Cw721State};
use crate::{traits::StateFactory, Approval, CollectionMetadataExtension, RoyaltyInfo};

#[cw_serde]
pub enum Cw721ExecuteMsg<
    // Message passed for updating metadata.
    TNftMetadataExtensionMsg,
    // Message passed for updating collection info extension.
    TCollectionMetadataExtensionMsg,
> {
    #[deprecated(since = "0.19.0", note = "Please use UpdateMinterOwnership instead")]
    /// Deprecated: use UpdateMinterOwnership instead! Will be removed in next release!
    UpdateOwnership(Action),
    UpdateMinterOwnership(Action),
    UpdateCreatorOwnership(Action),

    /// The creator is the only one eligible to update `CollectionMetadata`.
    UpdateCollectionMetadata {
        collection_metadata: CollectionMetadataMsg<TCollectionMetadataExtensionMsg>,
    },
    /// Transfer is a base message to move a token to another account without triggering actions
    TransferNft {
        recipient: String,
        token_id: String,
    },
    /// Send is a base message to transfer a token to a contract and trigger an action
    /// on the receiving contract.
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    /// Allows operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted Approval
    Revoke {
        spender: String,
        token_id: String,
    },
    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    /// Remove previously granted ApproveAll permission
    RevokeAll {
        operator: String,
    },

    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        /// Unique ID of the NFT
        token_id: String,
        /// The owner of the newly minter NFT
        owner: String,
        /// Universal resource identifier for this NFT
        /// Should point to a JSON file that conforms to the ERC721
        /// Metadata JSON Schema
        token_uri: Option<String>,
        /// Any custom extension used by this contract
        extension: TNftMetadataExtensionMsg,
    },

    /// Burn an NFT the sender has access to
    Burn {
        token_id: String,
    },

    /// Metadata msg
    #[deprecated(since = "0.19.0", note = "Please use UpdateNftMetadata instead")]
    /// Deprecated: use UpdateNftMetadata instead! In previous release it was a no-op for customization in other contracts. Will be removed in next release!
    Extension {
        msg: TNftMetadataExtensionMsg,
    },
    /// The creator is the only one eligible to update NFT's token uri and onchain metadata (`NftInfo.extension`).
    /// NOTE: approvals and owner are not affected by this call, since they belong to the NFT owner.
    UpdateNftInfo {
        token_id: String,
        token_uri: Option<String>,
        extension: TNftMetadataExtensionMsg,
    },

    /// Sets address to send withdrawn fees to. Only owner can call this.
    SetWithdrawAddress {
        address: String,
    },
    /// Removes the withdraw address, so fees are sent to the contract. Only owner can call this.
    RemoveWithdrawAddress {},
    /// Withdraw from the contract to the given address. Anyone can call this,
    /// which is okay since withdraw address has been set by owner.
    WithdrawFunds {
        amount: Coin,
    },
}

#[cw_serde]
pub struct Cw721InstantiateMsg<TCollectionMetadataExtensionMsg> {
    /// Name of the collection metadata
    pub name: String,
    /// Symbol of the collection metadata
    pub symbol: String,
    /// Optional extension of the collection metadata
    pub collection_metadata_extension: TCollectionMetadataExtensionMsg,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: Option<String>,

    /// Sets the creator of collection. The creator is the only one eligible to update `CollectionMetadata`.
    pub creator: Option<String>,

    pub withdraw_address: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum Cw721QueryMsg<
    // Return type of NFT metadata defined in `NftInfo` and `AllNftInfo`.
    TNftMetadataExtension,
    // Return type of collection metadata extension defined in `GetCollectionMetadata`.
    TCollectionMetadataExtension,
> {
    /// Return the owner of the given token, error if token does not exist
    #[returns(OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(NumTokensResponse)]
    NumTokens {},

    #[deprecated(since = "0.19.0", note = "Please use GetCollectionMetadata instead")]
    #[returns(CollectionMetadata<TCollectionMetadataExtension>)]
    /// Deprecated: use GetCollectionMetadata instead! Will be removed in next release!
    ContractInfo {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(CollectionMetadata<TCollectionMetadataExtension>)]
    GetCollectionMetadata {},

    #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
    #[returns(Ownership<Addr>)]
    /// Deprecated: use GetMinterOwnership instead! Will be removed in next release!
    Ownership {},

    /// Return the minter
    #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
    #[returns(MinterResponse)]
    /// Deprecated: use GetMinterOwnership instead! Will be removed in next release!
    Minter {},

    #[returns(Ownership<Addr>)]
    GetMinterOwnership {},

    #[returns(Ownership<Addr>)]
    GetCreatorOwnership {},

    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(NftInfoResponse<TNftMetadataExtension>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(AllNftInfoResponse<TNftMetadataExtension>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
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

    #[returns(Option<String>)]
    GetWithdrawAddress {},

    // -- below queries, Extension and GetCollectionMetadataExtension, are just dummies, since type annotations are required for
    // -- TNftMetadataExtension and TCollectionMetadataExtension, Error:
    // -- "type annotations needed: cannot infer type for type parameter `TNftMetadataExtension` declared on the enum `Cw721QueryMsg`"
    /// Use NftInfo instead.
    /// No-op / NFT metadata query returning empty binary, needed for inferring type parameter during compile.
    ///
    /// Note: it may be extended in case there are use cases e.g. for specific NFT metadata query.
    #[returns(())]
    #[deprecated(since = "0.19.0", note = "Please use GetNftMetadata instead")]
    Extension { msg: TNftMetadataExtension },

    #[returns(())]
    GetNftMetadata { msg: TNftMetadataExtension },

    /// Use GetCollectionMetadata instead.
    /// No-op / collection metadata extension query returning empty binary, needed for inferring type parameter during compile
    ///
    /// Note: it may be extended in case there are use cases e.g. for specific collection metadata query.
    #[returns(())]
    GetCollectionMetadataExtension { msg: TCollectionMetadataExtension },
}

#[cw_serde]
pub enum Cw721MigrateMsg {
    WithUpdate {
        minter: Option<String>,
        creator: Option<String>,
    },
}

#[cw_serde]
pub struct CollectionMetadataMsg<TCollectionMetadataExtensionMsg> {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub extension: TCollectionMetadataExtensionMsg,
}

#[cw_serde]
pub struct CollectionMetadataExtensionMsg<TRoyaltyInfoResponse> {
    pub description: Option<String>,
    pub image: Option<String>,
    pub external_link: Option<String>,
    pub explicit_content: Option<bool>,
    pub start_trading_time: Option<Timestamp>,
    pub royalty_info: Option<TRoyaltyInfoResponse>,
}

impl<TRoyaltyInfoResponse> Cw721CustomMsg for CollectionMetadataExtensionMsg<TRoyaltyInfoResponse> where
    TRoyaltyInfoResponse: Cw721CustomMsg
{
}

impl StateFactory<CollectionMetadataExtension<RoyaltyInfo>>
    for CollectionMetadataExtensionMsg<RoyaltyInfoResponse>
{
    fn create(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        current: Option<&CollectionMetadataExtension<RoyaltyInfo>>,
    ) -> Result<CollectionMetadataExtension<RoyaltyInfo>, Cw721ContractError> {
        self.validate(deps, env, info, current)?;
        match current {
            // Some: update existing metadata
            Some(current) => {
                let mut updated = current.clone();
                if let Some(description) = &self.description {
                    updated.description = description.clone();
                }
                if let Some(image) = &self.image {
                    updated.image = image.clone();
                }
                if let Some(external_link) = &self.external_link {
                    updated.external_link = Some(external_link.clone());
                }
                if let Some(explicit_content) = self.explicit_content {
                    updated.explicit_content = Some(explicit_content);
                }
                if let Some(start_trading_time) = self.start_trading_time {
                    updated.start_trading_time = Some(start_trading_time);
                }
                if let Some(royalty_info_response) = &self.royalty_info {
                    match current.royalty_info.clone() {
                        // Some: existing royalty info for update
                        Some(current_royalty_info) => {
                            updated.royalty_info = Some(royalty_info_response.create(
                                deps,
                                env,
                                info,
                                Some(&current_royalty_info),
                            )?);
                        }
                        // None: no royalty info, so create new
                        None => {
                            updated.royalty_info =
                                Some(royalty_info_response.create(deps, env, info, None)?);
                        }
                    }
                }
                Ok(updated)
            }
            // None: create new metadata
            None => {
                let royalty_info = match &self.royalty_info {
                    // new royalty info
                    Some(royalty_info) => Some(royalty_info.create(deps, env, info, None)?),
                    // current royalty is none and new royalty is none
                    None => None,
                };
                let new = CollectionMetadataExtension {
                    description: self.description.clone().unwrap_or_default(),
                    image: self.image.clone().unwrap_or_default(),
                    external_link: self.external_link.clone(),
                    explicit_content: self.explicit_content,
                    start_trading_time: self.start_trading_time,
                    royalty_info,
                };
                Ok(new)
            }
        }
    }

    fn validate(
        &self,
        deps: Deps,
        _env: &Env,
        info: &MessageInfo,
        _current: Option<&CollectionMetadataExtension<RoyaltyInfo>>,
    ) -> Result<(), Cw721ContractError> {
        // start trading time can only be updated by minter
        let minter_initialized = MINTER.item.may_load(deps.storage)?;
        if self.start_trading_time.is_some()
            && minter_initialized.is_some()
            && MINTER.assert_owner(deps.storage, &info.sender).is_err()
        {
            return Err(Cw721ContractError::NotMinter {});
        }
        // all other props collection metadata extension can only be updated by the creator
        let creator_initialized = CREATOR.item.may_load(deps.storage)?;
        if (self.description.is_some()
            || self.image.is_some()
            || self.external_link.is_some()
            || self.explicit_content.is_some()
            || self.royalty_info.is_some())
            && creator_initialized.is_some()
            && CREATOR.assert_owner(deps.storage, &info.sender).is_err()
        {
            return Err(Cw721ContractError::NotCollectionCreator {});
        }
        // check description length, must not be empty and max 512 chars
        if let Some(description) = &self.description {
            if description.is_empty() {
                return Err(Cw721ContractError::CollectionDescriptionEmpty {});
            }
            if description.len() > MAX_COLLECTION_DESCRIPTION_LENGTH as usize {
                return Err(Cw721ContractError::CollectionDescriptionTooLong {
                    max_length: MAX_COLLECTION_DESCRIPTION_LENGTH,
                });
            }
        }

        // check images are URLs
        if let Some(image) = &self.image {
            Url::parse(image)?;
        }
        if let Some(external_link) = &self.external_link {
            Url::parse(external_link)?;
        }
        // no need to check royalty info, as it is checked during creation of RoyaltyInfo
        Ok(())
    }
}

#[cw_serde]
// This is both: a query response, and incoming message during instantiation and execution.
pub struct RoyaltyInfoResponse {
    pub payment_address: String,
    pub share: Decimal,
}

impl Cw721CustomMsg for RoyaltyInfoResponse {}

impl StateFactory<RoyaltyInfo> for RoyaltyInfoResponse {
    fn create(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        current: Option<&RoyaltyInfo>,
    ) -> Result<RoyaltyInfo, Cw721ContractError> {
        self.validate(deps, env, info, current)?;
        match current {
            // Some: update existing royalty info
            Some(current) => {
                let mut updated = current.clone();
                updated.payment_address = deps.api.addr_validate(self.payment_address.as_str())?;
                updated.share = self.share;
                Ok(updated)
            }
            // None: create new royalty info
            None => {
                let new = RoyaltyInfo {
                    payment_address: deps.api.addr_validate(self.payment_address.as_str())?,
                    share: self.share,
                };
                Ok(new)
            }
        }
    }

    fn validate(
        &self,
        _deps: Deps,
        _env: &Env,
        _info: &MessageInfo,
        current: Option<&RoyaltyInfo>,
    ) -> Result<(), Cw721ContractError> {
        if let Some(current_royalty_info) = current {
            // check max share delta
            if current_royalty_info.share < self.share {
                let share_delta = self.share.abs_diff(current_royalty_info.share);

                if share_delta > Decimal::percent(MAX_ROYALTY_SHARE_DELTA_PCT) {
                    return Err(Cw721ContractError::InvalidRoyalties(format!(
                        "Share increase cannot be greater than {MAX_ROYALTY_SHARE_DELTA_PCT}%"
                    )));
                }
            }
        }
        // check max share
        if self.share > Decimal::percent(MAX_ROYALTY_SHARE_PCT) {
            return Err(Cw721ContractError::InvalidRoyalties(format!(
                "Share cannot be greater than {MAX_ROYALTY_SHARE_PCT}%"
            )));
        }
        Ok(())
    }
}

impl From<RoyaltyInfo> for RoyaltyInfoResponse {
    fn from(royalty_info: RoyaltyInfo) -> Self {
        Self {
            payment_address: royalty_info.payment_address.to_string(),
            share: royalty_info.share,
        }
    }
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub owner: String,
    /// If set this address is approved to transfer/send the token as well
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct ApprovalResponse {
    pub approval: Approval,
}

#[cw_serde]
pub struct ApprovalsResponse {
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct OperatorResponse {
    pub approval: Approval,
}

#[cw_serde]
pub struct OperatorsResponse {
    pub operators: Vec<Approval>,
}

#[cw_serde]
pub struct NumTokensResponse {
    pub count: u64,
}

#[cw_serde]
pub struct NftInfoResponse<TNftMetadataExtension> {
    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,
    /// You can add any custom metadata here when you extend cw721-base
    pub extension: TNftMetadataExtension,
}

#[cw_serde]
pub struct AllNftInfoResponse<TNftMetadataExtension> {
    /// Who can transfer the token
    pub access: OwnerOfResponse,
    /// Data on the token itself,
    pub info: NftInfoResponse<TNftMetadataExtension>,
}

#[cw_serde]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_after` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}

/// Deprecated: use Cw721QueryMsg::GetMinterOwnership instead!
/// Shows who can mint these tokens.
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

#[cw_serde]
pub struct NftInfoMsg<TNftMetadataExtensionMsg> {
    /// The owner of the newly minted NFT
    pub owner: String,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Universal resource identifier for this NFT
    /// Should point to a JSON file that conforms to the ERC721
    /// Metadata JSON Schema
    pub token_uri: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: TNftMetadataExtensionMsg,
}

impl<TNftMetadataExtension, TNftMetadataExtensionMsg> StateFactory<NftInfo<TNftMetadataExtension>>
    for NftInfoMsg<TNftMetadataExtensionMsg>
where
    TNftMetadataExtension: Cw721State,
    TNftMetadataExtensionMsg: Cw721CustomMsg + StateFactory<TNftMetadataExtension>,
{
    fn create(
        &self,
        deps: Deps,
        env: &cosmwasm_std::Env,
        info: &cosmwasm_std::MessageInfo,
        optional_current: Option<&NftInfo<TNftMetadataExtension>>,
    ) -> Result<NftInfo<TNftMetadataExtension>, Cw721ContractError> {
        self.validate(deps, env, info, optional_current)?;
        match optional_current {
            // Some: update only token uri and extension in existing NFT (but not owner and approvals)
            Some(current) => {
                let mut updated = current.clone();
                if let Some(token_uri) = &self.token_uri {
                    updated.token_uri = Some(token_uri.clone());
                }
                // update extension
                let current_extension = optional_current.map(|c| &c.extension);
                updated.extension = self.extension.create(deps, env, info, current_extension)?;
                Ok(updated)
            }
            // None: create new NFT, note: msg is of same type, so we can clone it
            None => {
                let extension = self.extension.create(deps, env, info, None)?;
                Ok(NftInfo {
                    owner: deps.api.addr_validate(&self.owner)?, // only for creation we use owner, but not for update!
                    approvals: vec![],
                    token_uri: self.token_uri.clone(),
                    extension,
                })
            }
        }
    }

    fn validate(
        &self,
        deps: Deps,
        env: &cosmwasm_std::Env,
        info: &cosmwasm_std::MessageInfo,
        current: Option<&NftInfo<TNftMetadataExtension>>,
    ) -> Result<(), Cw721ContractError> {
        // validate token_uri is a URL
        if let Some(token_uri) = &self.token_uri {
            Url::parse(token_uri)?;
        }
        // validate extension
        // current is a nested option in option, so we need to flatten it
        let current_extension = current.map(|c| &c.extension);
        self.extension
            .validate(deps, env, info, current_extension)?;
        Ok(())
    }
}

impl<TMsg, TState> StateFactory<Option<TState>> for Option<TMsg>
where
    TState: Cw721State,
    TMsg: Cw721CustomMsg + StateFactory<TState>,
{
    fn create(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        current: Option<&Option<TState>>,
    ) -> Result<Option<TState>, Cw721ContractError> {
        // no msg, so no validation needed
        if self.is_none() {
            return Ok(None);
        }
        let msg = self.clone().unwrap();
        // current is a nested option in option, so we need to flatten it
        let current = current.and_then(|c| c.as_ref());
        let created_or_updated = msg.create(deps, env, info, current)?;
        Ok(Some(created_or_updated))
    }

    fn validate(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        current: Option<&Option<TState>>,
    ) -> Result<(), Cw721ContractError> {
        // no msg, so no validation needed
        if self.is_none() {
            return Ok(());
        }
        let msg = self.clone().unwrap();
        // current is a nested option in option, so we need to unwrap and then match it
        // current is a nested option in option, so we need to flatten it
        let current = current.and_then(|c| c.as_ref());
        msg.validate(deps, env, info, current)
    }
}

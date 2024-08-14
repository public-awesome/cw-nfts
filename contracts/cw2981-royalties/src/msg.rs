use crate::{
    DefaultOptionMetadataExtensionWithRoyalty, DefaultOptionMetadataExtensionWithRoyaltyMsg,
    MetadataWithRoyalty,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Deps, Empty, Env, MessageInfo, Uint128};
use cw721::msg::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, MinterResponse, NftInfoResponse,
    NumTokensResponse, OperatorResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};
use cw721::{
    error::Cw721ContractError,
    execute::{assert_creator, assert_minter},
    msg::{empty_as_none, CollectionInfoAndExtensionResponse, Cw721QueryMsg},
    traits::StateFactory,
};
use cw_ownable::Ownership;
use url::Url;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Should be called on sale to see if royalties are owed
    /// by the marketplace selling the NFT, if CheckRoyalties
    /// returns true
    /// See https://eips.ethereum.org/EIPS/eip-2981
    #[returns(RoyaltiesInfoResponse)]
    RoyaltyInfo {
        token_id: String,
        // the denom of this sale must also be the denom returned by RoyaltiesInfoResponse
        // this was originally implemented as a Coin
        // however that would mean you couldn't buy using CW20s
        // as CW20 is just mapping of addr -> balance
        sale_price: Uint128,
    },
    /// Called against contract to determine if this NFT
    /// implements royalties. Should return a boolean as part of
    /// CheckRoyaltiesResponse - default can simply be true
    /// if royalties are implemented at token level
    /// (i.e. always check on sale)
    #[returns(CheckRoyaltiesResponse)]
    CheckRoyalties {},

    // -- below copied from Cw721QueryMsg
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

    #[deprecated(since = "0.19.0", note = "Please use GetCollectionInfo instead")]
    #[returns(CollectionInfoAndExtensionResponse<Option<Empty>>)]
    ContractInfo {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(CollectionInfoAndExtensionResponse<Option<Empty>>)]
    GetCollectionInfoAndExtension {},

    #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
    #[returns(Ownership<Addr>)]
    Ownership {},

    #[returns(Ownership<Addr>)]
    GetMinterOwnership {},

    #[returns(Ownership<Addr>)]
    GetCreatorOwnership {},

    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(NftInfoResponse<DefaultOptionMetadataExtensionWithRoyalty>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(AllNftInfoResponse<DefaultOptionMetadataExtensionWithRoyalty>)]
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

    /// Return the minter
    #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
    #[returns(MinterResponse)]
    Minter {},

    #[returns(Option<String>)]
    GetWithdrawAddress {},

    #[returns(())]
    Extension {
        msg: DefaultOptionMetadataExtensionWithRoyaltyMsg,
    },

    #[returns(())]
    GetCollectionInfoExtension { msg: Empty },
}

impl From<QueryMsg> for Cw721QueryMsg<DefaultOptionMetadataExtensionWithRoyalty, Empty, Empty> {
    fn from(
        msg: QueryMsg,
    ) -> Cw721QueryMsg<DefaultOptionMetadataExtensionWithRoyalty, Empty, Empty> {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            #[allow(deprecated)]
            QueryMsg::ContractInfo {} => Cw721QueryMsg::GetCollectionInfoAndExtension {},
            QueryMsg::GetCollectionInfoAndExtension {} => {
                Cw721QueryMsg::GetCollectionInfoAndExtension {}
            }
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            },
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            },
            QueryMsg::AllTokens { start_after, limit } => {
                Cw721QueryMsg::AllTokens { start_after, limit }
            }
            #[allow(deprecated)]
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            QueryMsg::GetMinterOwnership {} => Cw721QueryMsg::GetMinterOwnership {},
            QueryMsg::GetCreatorOwnership {} => Cw721QueryMsg::GetCreatorOwnership {},
            QueryMsg::GetWithdrawAddress {} => Cw721QueryMsg::GetWithdrawAddress {},
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            },
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            },
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            },
            msg => unreachable!("Unsupported query: {:?}", msg),
        }
    }
}

#[cw_serde]
pub struct RoyaltiesInfoResponse {
    pub address: String,
    // Note that this must be the same denom as that passed in to RoyaltyInfo
    // rounding up or down is at the discretion of the implementer
    pub royalty_amount: Uint128,
}

/// Shows if the contract implements royalties
/// if royalty_payments is true, marketplaces should pay them
#[cw_serde]
pub struct CheckRoyaltiesResponse {
    pub royalty_payments: bool,
}

pub type MetadataWithRoyaltyMsg = MetadataWithRoyalty;

// this is simply a copy of NftExtension
impl StateFactory<MetadataWithRoyalty> for MetadataWithRoyaltyMsg {
    fn create(
        &self,
        deps: Deps,
        env: &Env,
        info: Option<&MessageInfo>,
        current: Option<&MetadataWithRoyalty>,
    ) -> Result<MetadataWithRoyalty, Cw721ContractError> {
        self.validate(deps, env, info, current)?;
        match current {
            // Some: update existing metadata
            Some(current) => {
                let mut updated = current.clone();
                if self.image.is_some() {
                    updated.image = empty_as_none(self.image.clone());
                }
                if self.image_data.is_some() {
                    updated.image_data = empty_as_none(self.image_data.clone());
                }
                if self.external_url.is_some() {
                    updated.external_url = empty_as_none(self.external_url.clone());
                }
                if self.description.is_some() {
                    updated.description = empty_as_none(self.description.clone());
                }
                if self.name.is_some() {
                    updated.name = empty_as_none(self.name.clone());
                }
                if self.attributes.is_some() {
                    updated.attributes = match self.attributes.clone() {
                        Some(attributes) => Some(attributes.create(deps, env, info, None)?),
                        None => None,
                    };
                }
                if self.background_color.is_some() {
                    updated.background_color = empty_as_none(self.background_color.clone())
                }
                if self.animation_url.is_some() {
                    updated.animation_url = empty_as_none(self.animation_url.clone());
                }
                if self.youtube_url.is_some() {
                    updated.youtube_url = empty_as_none(self.youtube_url.clone());
                }
                Ok(updated)
            }
            // None: create new metadata, note: msg is of same type as metadata, so we can clone it
            None => {
                let mut new_metadata: MetadataWithRoyalty = self.clone();
                if self.attributes.is_some() {
                    new_metadata.attributes = match self.attributes.clone() {
                        Some(attributes) => Some(attributes.create(deps, env, info, None)?),
                        None => None,
                    };
                }
                Ok(new_metadata)
            }
        }
    }

    fn validate(
        &self,
        deps: Deps,
        _env: &Env,
        info: Option<&MessageInfo>,
        current: Option<&MetadataWithRoyalty>,
    ) -> Result<(), Cw721ContractError> {
        // assert here is different to NFT Info:
        // - creator and minter can create NFT metadata
        // - only creator can update NFT metadata
        if current.is_none() {
            let info = info.ok_or(Cw721ContractError::NoInfo)?;
            // current is none: minter and creator can create new NFT metadata
            let minter_check = assert_minter(deps.storage, &info.sender);
            let creator_check = assert_creator(deps.storage, &info.sender);
            if minter_check.is_err() && creator_check.is_err() {
                return Err(Cw721ContractError::NotMinterOrCreator {});
            }
        } else {
            let info = info.ok_or(Cw721ContractError::NoInfo)?;
            // current is some: only creator can update NFT metadata
            assert_creator(deps.storage, &info.sender)?;
        }
        // check URLs
        let image = empty_as_none(self.image.clone());
        if let Some(image) = &image {
            Url::parse(image)?;
        }
        let external_url = empty_as_none(self.external_url.clone());
        if let Some(url) = &external_url {
            Url::parse(url)?;
        }
        let animation_url = empty_as_none(self.animation_url.clone());
        if let Some(animation_url) = &animation_url {
            Url::parse(animation_url)?;
        }
        let youtube_url = empty_as_none(self.youtube_url.clone());
        if let Some(youtube_url) = &youtube_url {
            Url::parse(youtube_url)?;
        }
        // no need to validate simple strings: image_data, description, name, and background_color
        Ok(())
    }
}

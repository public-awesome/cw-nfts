use crate::{DefaultOptionalNftExtension, MinterResponse};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Empty};
use cw721::{
    msg::{CollectionInfoAndExtensionResponse, Cw721ExecuteMsg, Cw721MigrateMsg},
    DefaultOptionalCollectionExtension, DefaultOptionalCollectionExtensionMsg,
    DefaultOptionalNftExtensionMsg,
};
use cw_ownable::Ownership;

pub type ExecuteMsg =
    Cw721ExecuteMsg<DefaultOptionalNftExtensionMsg, DefaultOptionalCollectionExtensionMsg, Empty>;
pub type MigrateMsg = Cw721MigrateMsg;

#[cw_serde]
pub struct InstantiateMsg {
    /// max 65535 days
    pub expiration_days: u16,

    // -------- below is from cw721-base/src/msg.rs --------
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,
    /// Optional extension of the collection metadata
    pub collection_info_extension: DefaultOptionalCollectionExtensionMsg,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: Option<String>,

    /// The creator is the only who can update collection info.
    pub creator: Option<String>,

    pub withdraw_address: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<TExtensionQueryMsg> {
    // -------- below adds `include_expired_nft` prop to cw721/src/msg.rs --------
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::msg::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::msg::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::msg::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::msg::NftInfoResponse<DefaultOptionalNftExtension>)]
    NftInfo {
        token_id: String,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    #[returns(Option<cw721::msg::NftInfoResponse<DefaultOptionalNftExtension>>)]
    GetNftByExtension {
        token_id: String,
        extension: DefaultOptionalNftExtension,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::msg::AllNftInfoResponse<DefaultOptionalNftExtension>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::msg::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::msg::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    // -------- below is from cw721/src/msg.rs --------
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::msg::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::msg::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued, including all expired NFTs
    #[returns(cw721::msg::NumTokensResponse)]
    NumTokens {},

    #[deprecated(
        since = "0.19.0",
        note = "Please use GetCollectionInfoAndExtension instead"
    )]
    #[returns(CollectionInfoAndExtensionResponse<cw721::DefaultOptionalCollectionExtension>)]
    /// Deprecated: use GetCollectionInfoAndExtension instead! Will be removed in next release!
    ContractInfo {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(CollectionInfoAndExtensionResponse<DefaultOptionalCollectionExtension>)]
    GetCollectionInfo {},

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

    /// Extension query
    #[returns(())]
    Extension {
        msg: TExtensionQueryMsg,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    /// This is a workaround and dummy query like (same as for Extension) for avoiding this compiler error:
    /// `cannot infer type for type parameter `TCollectionExtension` declared on the enum `QueryMsg`
    #[returns(())]
    GetCollectionExtension {
        msg: DefaultOptionalCollectionExtension,
    },

    #[returns(Option<String>)]
    GetWithdrawAddress {},
}

use crate::{DefaultOptionMetadataExtension, MinterResponse};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw721::state::CollectionInfo;
use cw_ownable::Ownership;

#[cw_serde]
pub struct InstantiateMsg {
    /// max 65535 days
    pub expiration_days: u16,

    // -------- below is from cw721-base/src/msg.rs --------
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    /// The minter is the only one who can create new NFTs.
    /// This is designed for a base NFT that is controlled by an external program
    /// or contract. You will likely replace this with custom logic in custom NFTs
    pub minter: Option<String>,

    pub withdraw_address: Option<String>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg<TMetadataExtension> {
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
    #[returns(cw721::msg::NftInfoResponse<DefaultOptionMetadataExtension>)]
    NftInfo {
        token_id: String,
        /// unset or false will filter out expired nfts, you must set to true to see them
        include_expired_nft: Option<bool>,
    },

    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::msg::AllNftInfoResponse<DefaultOptionMetadataExtension>)]
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

    #[returns(cw721::state::CollectionInfo)]
    ContractInfo {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(CollectionInfo)]
    GetCollectionInfo {},

    #[returns(Ownership<Addr>)]
    Ownership {},

    #[returns(Ownership<Addr>)]
    GetMinterOwnership {},

    /// Return the minter
    #[returns(MinterResponse)]
    Minter {},

    /// Extension query
    #[returns(())]
    Extension { msg: TMetadataExtension },

    #[returns(Option<String>)]
    GetWithdrawAddress {},
}

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, Coin};
use cw_ownable::{Action, Ownership};
use cw_utils::Expiration;
use schemars::JsonSchema;

use crate::query::{
    AllNftInfoResponse, ApprovalResponse, ApprovalsResponse, NftInfoResponse, NumTokensResponse,
    OperatorResponse, OperatorsResponse, OwnerOfResponse, TokensResponse,
};

use crate::state::CollectionInfo;

use cosmwasm_std::Empty;

#[cw_serde]
pub enum Cw721ExecuteMsg<TMetadata, TExtensionExecuteMsg, TCollectionInfoExtension> {
    #[deprecated(since = "0.19.0", note = "Please use UpdateMinterOwnership instead")]
    UpdateOwnership(Action),
    UpdateMinterOwnership(Action),
    UpdateCreatorOwnership(Action),

    /// Update the collection, can only called by the contract creator
    UpdateCollectionInfo {
        collection_info: CollectionInfoMsg<TCollectionInfoExtension>,
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
        extension: TMetadata,
    },

    /// Burn an NFT the sender has access to
    Burn {
        token_id: String,
    },

    /// Extension msg
    Extension {
        msg: TExtensionExecuteMsg,
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
pub struct Cw721InstantiateMsg<TCollectionInfoExtension> {
    /// Name of the NFT contract
    pub name: String,
    /// Symbol of the NFT contract
    pub symbol: String,

    pub collection_info_extension: TCollectionInfoExtension,

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
pub enum Cw721QueryMsg<TMetadataResponse: JsonSchema, TCollectionInfoExtension> {
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
    #[returns(CollectionInfo<Empty>)]
    ContractInfo {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(CollectionInfo<TCollectionInfoExtension>)]
    GetCollectionInfo {},

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
    #[returns(NftInfoResponse<TMetadataResponse>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(AllNftInfoResponse<TMetadataResponse>)]
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

    // -- below queries, Extension and GetCollectionInfoExtension, are just dummies, since type annotations are required for
    // -- TMetadataResponse and TCollectionInfoExtension, Error:
    // -- "type annotations needed: cannot infer type for type parameter `TMetadataResponse` declared on the enum `Cw721QueryMsg`"
    /// Do not use - dummy extension query, needed for inferring type parameter during compile
    #[returns(())]
    Extension { msg: TMetadataResponse },

    /// Do not use - dummy collection info extension query, needed for inferring type parameter during compile
    #[returns(())]
    GetCollectionInfoExtension { msg: TCollectionInfoExtension },
}

/// Shows who can mint these tokens
#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

#[cw_serde]
pub enum Cw721MigrateMsg {
    WithUpdate {
        minter: Option<String>,
        creator: Option<String>,
    },
}

#[cw_serde]
pub struct CollectionInfoMsg<TCollectionInfoExtension> {
    pub name: String,
    pub symbol: String,
    pub extension: TCollectionInfoExtension,
}

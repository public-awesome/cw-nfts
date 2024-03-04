use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw721_base::msg::QueryMsg as Cw721QueryMsg;

#[cw_serde]
pub struct InstantiateMsg<TCollectionInfoExtension> {
    pub admin: Option<String>,
    pub name: String,
    pub symbol: String,
    pub collection_info_extension: TCollectionInfoExtension,
    pub minter: Option<String>,
    pub creator: Option<String>,
    pub withdraw_address: Option<String>,
}

#[cw_serde]
pub enum QueryMsg {
    Admin {},
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    NumTokens {},
    #[deprecated(since = "0.19.0", note = "Please use GetCollectionInfo instead")]
    ContractInfo {},
    GetCollectionInfo {},
    GetMinterOwnership {},
    GetCreatorOwnership {},

    NftInfo {
        token_id: String,
    },
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[deprecated(since = "0.19.0", note = "Please use GetMinterOwnership instead")]
    Minter {},

    GetWithdrawAddress {},
}

impl From<QueryMsg> for Cw721QueryMsg<Empty> {
    fn from(msg: QueryMsg) -> Cw721QueryMsg<Empty> {
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
            QueryMsg::ContractInfo {} => Cw721QueryMsg::GetCollectionInfo {},
            QueryMsg::GetCollectionInfo {} => Cw721QueryMsg::GetCollectionInfo {},
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
            QueryMsg::Minter {} => Cw721QueryMsg::Minter {},
            QueryMsg::GetMinterOwnership {} => Cw721QueryMsg::GetMinterOwnership {},
            QueryMsg::GetCreatorOwnership {} => Cw721QueryMsg::GetCreatorOwnership {},
            QueryMsg::GetWithdrawAddress {} => Cw721QueryMsg::GetWithdrawAddress {},
            QueryMsg::AllOperators { .. } => unreachable!("AllOperators is not supported!"),
            QueryMsg::Approval { .. } => unreachable!("Approval is not supported!"),
            QueryMsg::Approvals { .. } => unreachable!("Approvals is not supported!"),
            QueryMsg::Admin { .. } => unreachable!("Approvals is not supported!"),
        }
    }
}

#[cw_serde]
pub struct AdminResponse {
    pub admin: Option<String>,
}

use std::convert::TryFrom;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdError, Uint128};
use cw721::cw721_query;
use cw721_base::QueryMsg as Cw721QueryMsg;

#[cw721_query]
#[cw_serde]
pub enum QueryMsg {
    /// Should be called on sale to see if royalties are owed
    /// by the marketplace selling the NFT, if CheckRoyalties
    /// returns true
    /// See https://eips.ethereum.org/EIPS/eip-2981
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
    CheckRoyalties {},
}

// TODO: perhaps this trait can be implemented by a macro
impl TryFrom<QueryMsg> for Cw721QueryMsg {
    type Error = StdError;

    fn try_from(msg: QueryMsg) -> Result<Self, Self::Error> {
        match msg {
            QueryMsg::OwnerOf {
                token_id,
                include_expired,
            } => Ok(Cw721QueryMsg::OwnerOf {
                token_id,
                include_expired,
            }),
            QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            } => Ok(Cw721QueryMsg::Approval {
                token_id,
                spender,
                include_expired,
            }),
            QueryMsg::Approvals {
                token_id,
                include_expired,
            } => Ok(Cw721QueryMsg::Approvals {
                token_id,
                include_expired,
            }),
            QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            } => Ok(Cw721QueryMsg::AllOperators {
                owner,
                include_expired,
                start_after,
                limit,
            }),
            QueryMsg::NumTokens {} => Ok(Cw721QueryMsg::NumTokens {}),
            QueryMsg::ContractInfo {} => Ok(Cw721QueryMsg::ContractInfo {}),
            QueryMsg::NftInfo { token_id } => Ok(Cw721QueryMsg::NftInfo { token_id }),
            QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            } => Ok(Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired,
            }),
            QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            } => Ok(Cw721QueryMsg::Tokens {
                owner,
                start_after,
                limit,
            }),
            QueryMsg::AllTokens { start_after, limit } => {
                Ok(Cw721QueryMsg::AllTokens { start_after, limit })
            }
            QueryMsg::RoyaltyInfo { .. } => Err(StdError::generic_err(
                "cannot cast `QueryMsg::RoyaltyInfo` in `cw721_base::QueryMsg`",
            )),
            QueryMsg::CheckRoyalties {} => Err(StdError::generic_err(
                "cannot cast `QueryMsg::CheckRoyalties` in `cw721_base::QueryMsg`",
            )),
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

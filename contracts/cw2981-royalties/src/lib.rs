pub mod msg;

use crate::msg::{CheckRoyaltiesResponse, RoyaltiesInfoResponse};
use cosmwasm_std::{to_binary, Deps, StdResult};
use percentage::Percentage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::msg::Cw2981QueryMsg;
use cosmwasm_std::Empty;
use cw721_base::Cw721Contract;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    /// specify whether royalties are set on this token
    pub royalty_payments: bool,
    /// This is how much the minter takes as a cut when sold
    pub royalty_percentage: Option<u32>,
    /// The payment address, may be different to or the same
    /// as the minter addr
    /// question: how do we validate this?
    pub royalty_payment_address: Option<String>,
}

pub type Extension = Option<Metadata>;

pub type MintExtension = Option<Extension>;

pub type Cw2981Contract<'a> = Cw721Contract<'a, Extension, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        Cw2981Contract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw2981Contract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: Cw2981QueryMsg) -> StdResult<Binary> {
        match msg {
            Cw2981QueryMsg::RoyaltyInfo {
                token_id,
                sale_price,
            } => to_binary(&query_royalties_info(deps, token_id, sale_price)?),
            Cw2981QueryMsg::CheckRoyalties {} => to_binary(&check_royalties(deps)?),
            _ => Cw2981Contract::default()
                .query(deps, env, msg.into())
                .map_err(|err| err.into()),
        }
    }
}

// NOTE: default behaviour here is to round down
// EIP2981 specifies that the rounding behaviour is at the discretion of the implementer
pub fn query_royalties_info(
    deps: Deps,
    token_id: String,
    sale_price: u128,
) -> StdResult<RoyaltiesInfoResponse> {
    let contract = Cw2981Contract::default();
    let token_info = contract.tokens.load(deps.storage, &token_id)?;

    let royalty_percentage = match token_info.extension {
        Some(ref ext) => match ext.royalty_percentage {
            Some(percentage) => Percentage::from(percentage),
            None => Percentage::from(0),
        },
        None => Percentage::from(0),
    };
    let royalty_from_sale_price = royalty_percentage.apply_to(sale_price);

    let royalty_address = match token_info.extension {
        Some(ext) => match ext.royalty_payment_address {
            Some(addr) => addr.to_string(),
            None => String::from(""),
        },
        None => String::from(""),
    };

    Ok(RoyaltiesInfoResponse {
        address: royalty_address,
        royalty_amount: royalty_from_sale_price,
    })
}

pub fn check_royalties(_deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
    Ok(CheckRoyaltiesResponse {
        royalty_payments: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Uint128;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw721::Cw721Query;

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw2981Contract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }

    #[test]
    fn check_royalties_response() {
        let mut deps = mock_dependencies();
        let contract = Cw2981Contract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = check_royalties(deps.as_ref()).unwrap();
        let expected = CheckRoyaltiesResponse {
            royalty_payments: true,
        };
        assert_eq!(res, expected);
    }

    #[test]
    fn check_token_royalties() {
        let mut deps = mock_dependencies();
        let contract = Cw2981Contract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "JeanLuc".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                royalty_payment_address: Some("JeanLuc".to_string()),
                royalty_percentage: Some(10),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = query_royalties_info(
            deps.as_ref(),
            token_id.to_string(),
            Uint128::new(100).u128(),
        )
        .unwrap();
        let expected = RoyaltiesInfoResponse {
            address: mint_msg.owner.to_string(),
            royalty_amount: Uint128::new(10).u128(),
        };
        assert_eq!(res, expected);
    }
}

use cosmwasm_std::Empty;
use cw1155::msg::{Cw1155ExecuteMsg, Cw1155QueryMsg};
use cw1155::state::Cw1155Config;
use cw1155_base::Cw1155Contract;
use cw2981_royalties::msg::QueryMsg as Cw2981QueryMsg;
use cw2981_royalties::DefaultOptionMetadataExtensionWithRoyalty;

mod query;
pub use query::query_royalties_info;

mod error;
pub use error::Cw1155RoyaltiesContractError;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw1155-royalties";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw1155RoyaltiesContract<'a> =
    Cw1155Contract<'a, DefaultOptionMetadataExtensionWithRoyalty, Empty, Empty, Cw2981QueryMsg>;
pub type Cw1155RoyaltiesExecuteMsg =
    Cw1155ExecuteMsg<DefaultOptionMetadataExtensionWithRoyalty, Empty>;
pub type Cw1155RoyaltiesQueryMsg =
    Cw1155QueryMsg<DefaultOptionMetadataExtensionWithRoyalty, Cw2981QueryMsg>;
pub type Cw1155RoyaltiesConfig<'a> =
    Cw1155Config<'a, DefaultOptionMetadataExtensionWithRoyalty, Empty, Empty, Cw2981QueryMsg>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::{entry_point, to_json_binary};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw1155::execute::Cw1155Execute;
    use cw1155::query::Cw1155Query;
    use cw2981_royalties::msg::QueryMsg as Cw2981QueryMsg;
    use cw2981_royalties::{check_royalties, MetadataWithRoyalty};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: cw1155::msg::Cw1155InstantiateMsg,
    ) -> Result<Response, Cw1155RoyaltiesContractError> {
        Cw1155RoyaltiesContract::default()
            .instantiate(
                deps.branch(),
                env,
                info,
                msg,
                CONTRACT_NAME,
                CONTRACT_VERSION,
            )
            .map_err(Into::into)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155RoyaltiesExecuteMsg,
    ) -> Result<Response, Cw1155RoyaltiesContractError> {
        if let Cw1155RoyaltiesExecuteMsg::Mint {
            msg:
                cw1155::msg::Cw1155MintMsg {
                    extension:
                        Some(MetadataWithRoyalty {
                            royalty_percentage: Some(royalty_percentage),
                            royalty_payment_address,
                            ..
                        }),
                    ..
                },
            ..
        } = &msg
        {
            // validate royalty_percentage to be between 0 and 100
            // no need to check < 0 because royalty_percentage is u64
            if *royalty_percentage > 100 {
                return Err(Cw1155RoyaltiesContractError::InvalidRoyaltyPercentage);
            }

            // validate royalty_payment_address to be a valid address
            if let Some(royalty_payment_address) = royalty_payment_address {
                deps.api.addr_validate(royalty_payment_address)?;
            } else {
                return Err(Cw1155RoyaltiesContractError::InvalidRoyaltyPaymentAddress);
            }
        }
        Ok(Cw1155RoyaltiesContract::default().execute(deps, env, info, msg)?)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: Cw1155RoyaltiesQueryMsg) -> StdResult<Binary> {
        match msg {
            Cw1155RoyaltiesQueryMsg::Extension { msg: ext_msg, .. } => match ext_msg {
                Cw2981QueryMsg::RoyaltyInfo {
                    token_id,
                    sale_price,
                } => to_json_binary(&query_royalties_info(deps, token_id, sale_price)?),
                Cw2981QueryMsg::CheckRoyalties {} => to_json_binary(&check_royalties(deps)?),
                _ => unimplemented!(),
            },
            _ => Cw1155RoyaltiesContract::default().query(deps, env, msg),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{from_json, Uint128};

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cw1155::msg::{Cw1155InstantiateMsg, Cw1155MintMsg};
    use cw2981_royalties::msg::{CheckRoyaltiesResponse, RoyaltiesInfoResponse};
    use cw2981_royalties::{check_royalties, MetadataWithRoyalty};

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let config = Cw1155RoyaltiesConfig::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = Cw1155InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: None,
        };
        entry::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let token_id = "Enterprise";
        let token_uri = Some("https://starships.example.com/Starship/Enterprise.json".into());
        let extension = Some(MetadataWithRoyalty {
            description: Some("Spaceship with Warp Drive".into()),
            name: Some("Starship USS Enterprise".to_string()),
            ..MetadataWithRoyalty::default()
        });
        let exec_msg = Cw1155RoyaltiesExecuteMsg::Mint {
            recipient: "john".to_string(),
            msg: Cw1155MintMsg {
                token_id: token_id.to_string(),
                token_uri: token_uri.clone(),
                extension: extension.clone(),
                amount: Uint128::one(),
            },
        };
        entry::execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let res = config.tokens.load(&deps.storage, token_id).unwrap();
        assert_eq!(res.token_uri, token_uri);
        assert_eq!(res.extension, extension);
    }

    #[test]
    fn validate_royalty_information() {
        let mut deps = mock_dependencies();
        let _contract = Cw1155RoyaltiesContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = Cw1155InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: None,
        };
        entry::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let token_id = "Enterprise";
        let exec_msg = Cw1155RoyaltiesExecuteMsg::Mint {
            recipient: "john".to_string(),
            msg: Cw1155MintMsg {
                token_id: token_id.to_string(),
                amount: Uint128::one(),
                token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
                extension: Some(MetadataWithRoyalty {
                    description: Some("Spaceship with Warp Drive".into()),
                    name: Some("Starship USS Enterprise".to_string()),
                    royalty_percentage: Some(101),
                    ..MetadataWithRoyalty::default()
                }),
            },
        };
        // mint will return StdError
        let err = entry::execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap_err();
        assert_eq!(err, Cw1155RoyaltiesContractError::InvalidRoyaltyPercentage);
    }

    #[test]
    fn check_royalties_response() {
        let mut deps = mock_dependencies();
        let _contract = Cw1155RoyaltiesContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = Cw1155InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: None,
        };
        entry::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let token_id = "Enterprise";
        let exec_msg = Cw1155RoyaltiesExecuteMsg::Mint {
            recipient: "john".to_string(),
            msg: Cw1155MintMsg {
                token_id: token_id.to_string(),
                amount: Uint128::one(),
                token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
                extension: Some(MetadataWithRoyalty {
                    description: Some("Spaceship with Warp Drive".into()),
                    name: Some("Starship USS Enterprise".to_string()),
                    ..MetadataWithRoyalty::default()
                }),
            },
        };
        entry::execute(deps.as_mut(), mock_env(), info, exec_msg).unwrap();

        let expected = CheckRoyaltiesResponse {
            royalty_payments: true,
        };
        let res = check_royalties(deps.as_ref()).unwrap();
        assert_eq!(res, expected);

        // also check the longhand way
        let query_msg = Cw1155RoyaltiesQueryMsg::Extension {
            msg: Cw2981QueryMsg::CheckRoyalties {},
            phantom: None,
        };
        let query_res: CheckRoyaltiesResponse =
            from_json(entry::query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_res, expected);
    }

    #[test]
    fn check_token_royalties() {
        let mut deps = mock_dependencies();

        let info = mock_info(CREATOR, &[]);
        let init_msg = Cw1155InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: None,
        };
        entry::instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg).unwrap();

        let token_id = "Enterprise";
        let owner = "jeanluc";
        let exec_msg = Cw1155RoyaltiesExecuteMsg::Mint {
            recipient: owner.into(),
            msg: Cw1155MintMsg {
                token_id: token_id.to_string(),
                amount: Uint128::one(),
                token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
                extension: Some(MetadataWithRoyalty {
                    description: Some("Spaceship with Warp Drive".into()),
                    name: Some("Starship USS Enterprise".to_string()),
                    royalty_payment_address: Some("jeanluc".to_string()),
                    royalty_percentage: Some(10),
                    ..MetadataWithRoyalty::default()
                }),
            },
        };
        entry::execute(deps.as_mut(), mock_env(), info.clone(), exec_msg).unwrap();

        let expected = RoyaltiesInfoResponse {
            address: owner.into(),
            royalty_amount: Uint128::new(10),
        };
        let res =
            query_royalties_info(deps.as_ref(), token_id.to_string(), Uint128::new(100)).unwrap();
        assert_eq!(res, expected);

        // also check the longhand way
        let query_msg = Cw1155RoyaltiesQueryMsg::Extension {
            msg: Cw2981QueryMsg::RoyaltyInfo {
                token_id: token_id.to_string(),
                sale_price: Uint128::new(100),
            },
            phantom: None,
        };
        let query_res: RoyaltiesInfoResponse =
            from_json(entry::query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        assert_eq!(query_res, expected);

        // check for rounding down
        // which is the default behaviour
        let voyager_token_id = "Voyager";
        let owner = "janeway";
        let voyager_exec_msg = Cw1155RoyaltiesExecuteMsg::Mint {
            recipient: owner.into(),
            msg: Cw1155MintMsg {
                token_id: voyager_token_id.to_string(),
                amount: Uint128::one(),
                token_uri: Some("https://starships.example.com/Starship/Voyager.json".into()),
                extension: Some(MetadataWithRoyalty {
                    description: Some("Spaceship with Warp Drive".into()),
                    name: Some("Starship USS Voyager".to_string()),
                    royalty_payment_address: Some("janeway".to_string()),
                    royalty_percentage: Some(4),
                    ..MetadataWithRoyalty::default()
                }),
            },
        };
        entry::execute(deps.as_mut(), mock_env(), info, voyager_exec_msg).unwrap();

        // 43 x 0.04 (i.e., 4%) should be 1.72
        // we expect this to be rounded down to 1
        let voyager_expected = RoyaltiesInfoResponse {
            address: owner.into(),
            royalty_amount: Uint128::new(1),
        };

        let res = query_royalties_info(
            deps.as_ref(),
            voyager_token_id.to_string(),
            Uint128::new(43),
        )
        .unwrap();
        assert_eq!(res, voyager_expected);
    }
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;

use cw1155::Cw1155ExecuteMsg;
pub use cw1155::{Cw1155ContractError, Cw1155InstantiateMsg, Cw1155MintMsg};
use cw2::set_contract_version;

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw1155-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
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
}

pub type Extension = Option<Metadata>;

pub type Cw1155MetadataContract<'a> =
    cw1155_base::Cw1155Contract<'a, Extension, Empty, Empty, Empty>;
pub type Cw1155MetadataExecuteMsg = Cw1155ExecuteMsg<Extension, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw1155::Cw1155QueryMsg;

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155InstantiateMsg,
    ) -> Result<Response, Cw1155ContractError> {
        let res = Cw1155MetadataContract::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw1155-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(Cw1155ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw1155MetadataExecuteMsg,
    ) -> Result<Response, Cw1155ContractError> {
        Cw1155MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: Cw1155QueryMsg<Empty>) -> StdResult<Binary> {
        Cw1155MetadataContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_json, Uint128};

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();
        let contract = Cw1155MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = Cw1155InstantiateMsg {
            name: "name".to_string(),
            symbol: "symbol".to_string(),
            minter: Some(CREATOR.to_string()),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = Cw1155MintMsg {
            token_id: token_id.to_string(),
            amount: Uint128::new(1),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Some(Metadata {
                description: Some("Spaceship with Warp Drive".into()),
                name: Some("Starship USS Enterprise".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = Cw1155MetadataExecuteMsg::Mint {
            recipient: "john".to_string(),
            msg: mint_msg.clone(),
        };
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res: cw1155::TokenInfoResponse<Extension> = from_json(
            contract
                .query(
                    deps.as_ref(),
                    mock_env(),
                    cw1155::Cw1155QueryMsg::TokenInfo {
                        token_id: token_id.to_string(),
                    },
                )
                .unwrap(),
        )
        .unwrap();

        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}
